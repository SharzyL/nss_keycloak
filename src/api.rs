use std::time::SystemTime;

use chrono::{DateTime, Utc, Duration};
use log::{info};
use mut_static::MutStatic;
use lazy_static::lazy_static;

use serde::Deserialize;

use crate::err::KeycloakError;
use crate::config::CONFIG;

#[derive(Clone)]
pub(crate) struct KeycloakCredentials {
    access_token: String,
    refresh_token: String,
    access_token_expire: DateTime<Utc>,
    refresh_token_expire: DateTime<Utc>,
}

lazy_static! {
    static ref CRED: MutStatic<Option<KeycloakCredentials>> = MutStatic::from(None);
}

#[derive(Deserialize, Debug, Clone)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    refresh_expires_in: i64,
}

pub(crate) fn request_token() -> Result<KeycloakCredentials, KeycloakError> {
    let conf = &CONFIG.lock().unwrap();
    let base_url = conf.base_url.to_owned();
    let client_id = conf.client_id.to_owned();
    let realm = conf.realm.to_owned();
    let username = conf.username.to_owned();
    let password = conf.password.to_owned();

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/realms/{}/protocol/openid-connect/token", base_url, realm);
    info!("request token from '{}'", url);
    let response_text = client
        .post(&url)
        .form(&[
            ("grant_type", "password"),
            ("client_id", &client_id),
            ("username", &username),
            ("password", &password),
        ])
        .send()?
        .text()?;
    let token_resp = serde_json::from_str::<TokenResponse>(&response_text)
        .map_err(|err| {
            KeycloakError::JSONParseError(response_text, err)
        })?;
    info!("get token (expires in {} sec, refresh expires in {} sec)",
        token_resp.expires_in, token_resp.refresh_expires_in);

    let now: DateTime<Utc> = SystemTime::now().into();

    Ok(KeycloakCredentials {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        access_token_expire: now + Duration::seconds(token_resp.expires_in),
        refresh_token_expire: now + Duration::seconds(token_resp.refresh_expires_in),
    })
}

pub(crate) fn do_refresh_token(cred: &KeycloakCredentials) -> Result<KeycloakCredentials, KeycloakError> {
    let conf = &CONFIG.lock().unwrap();
    let base_url = conf.base_url.to_owned();
    let realm = conf.realm.to_owned();
    let client_id = conf.client_id.to_owned();

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/realms/{}/protocol/openid-connect/token", base_url, realm);
    info!("refresh token from '{}'", url);
    let response_text = client
        .post(url)
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &client_id),
            ("refresh_token", &cred.refresh_token)
        ])
        .send()?
        .text()?;
    let token_resp = serde_json::from_str::<TokenResponse>(&response_text)
        .map_err(|err| {
            KeycloakError::JSONParseError(response_text, err)
        })?;
    info!("token refresh succeeded");

    let now: DateTime<Utc> = SystemTime::now().into();

    Ok(KeycloakCredentials {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        access_token_expire: now + Duration::seconds(token_resp.expires_in),
        refresh_token_expire: now + Duration::seconds(token_resp.refresh_expires_in),
    })
}

fn ensure_cred(option_cred: &mut Option<KeycloakCredentials>) -> Result<KeycloakCredentials, KeycloakError> {
    match option_cred {
        None => {
            request_token()
        }
        Some(cred) => {
            let now: DateTime<Utc> = SystemTime::now().into();
            if now < cred.access_token_expire - Duration::seconds(5) {
                Ok(cred.to_owned())
            } else if now < cred.refresh_token_expire - Duration::seconds(5) {
                do_refresh_token(cred)
            } else {
                request_token()
            }
        }
    }
}

pub(crate) struct User {
    pub name: String,
    pub uid: i16,
    pub github_id: String,
}

pub(crate) fn list_users() -> Result<Vec<User>, KeycloakError> {
    #[derive(Deserialize, Clone, Default)]
    struct GroupMemberAttributes {
        uid: Option<Vec<String>>,
        github_id: Option<Vec<String>>,
    }

    #[derive(Deserialize, Clone)]
    struct GroupMember {
        username: String,

        #[serde(default)]
        attributes: GroupMemberAttributes,
    }

    let mut cred_guard = CRED.write().unwrap();
    let cred = ensure_cred(&mut cred_guard)?;
    let access_token = cred.access_token.to_owned();
    *cred_guard = Some(cred);
    drop(cred_guard);  // no longer need it, release the lock

    let conf = &CONFIG.lock().unwrap();
    let base_url = conf.base_url.to_owned();
    let realm = conf.realm.to_owned();
    let group_id = conf.group_id.to_owned();

    let client = reqwest::blocking::Client::new();

    let url = format!("{}/admin/realms/{}/groups/{}/members", base_url, realm, group_id);
    info!("get group members from '{}'", url);
    let response_text = client
        .get(url)
        .bearer_auth(access_token)
        .timeout(std::time::Duration::from_secs(60))  // sometime keycloak server is slow
        .send()?
        .text()?;
    let members = serde_json::from_str::<Vec<GroupMember>>(&response_text)
        .map_err(|err| {
            KeycloakError::JSONParseError(response_text, err)
        })?;

    fn get_attr<A: Clone>(attr: &Option<Vec<A>>, name: &str, member_name: &str) -> Result<A, KeycloakError> {
        match attr {
            Some(vs) => {
                match vs.len() {
                    0 => Err(KeycloakError::DataError(format!("attribute '{name}' for user '{member_name}' is an empty array"))),
                    1 => Ok(vs[0].to_owned()),
                    _ => Err(KeycloakError::DataError(format!("multiple definition of attribute '{name}' for user '{member_name}'")))
                }
            }
            None => Err(KeycloakError::DataError(format!("no attribute '{name}' for user '{member_name}'")))
        }
    }

    let users: Result<Vec<_>, _> = members.iter().map(|member| -> Result<User, KeycloakError> {
        let member_name = &member.username;
        let attributes = &member.attributes;
        Ok(User {
            name: member.username.clone(),
            uid: get_attr(&attributes.uid, "uid", member_name)
                .and_then(|s|
                    s.parse::<i16>().map_err(
                        |e| KeycloakError::DataError(format!("unable to parse uid '{s}' as int: {e:?}"))
                    )
                )?,
            github_id: get_attr(&attributes.github_id, "github_id", member_name)?,
        })
    }).collect();
    users
}