use libnss::group::{Group, GroupHooks};
use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::shadow::{Shadow, ShadowHooks};
use log::error;
use crate::api::{list_users, User};
use crate::config::{CONFIG, Config};

struct KeycloakNssPasswd;
libnss_passwd_hooks!(keycloak, KeycloakNssPasswd);

fn user_to_passwd(u: &User, conf: &Config) -> Passwd {
    Passwd {
        name: u.name.to_string(),
        passwd: "".to_string(),
        uid: u.uid,
        gid: u.uid,
        gecos: "".to_string(),
        dir: format!("/home/{}", u.name),
        shell: conf.default_shell.to_string(),
    }
}

fn user_to_group(u: &User) -> Group {
    Group {
        name: u.name.to_string(),
        passwd: "".to_string(),
        gid: u.uid,
        members: vec![u.name.to_string()],
    }
}

fn user_to_shadow(u: &User) -> Shadow {
    Shadow {
        name: u.name.to_string(),
        passwd: "".to_string(),
        last_change: 0,
        change_min_days: 0,
        change_max_days: 0,
        change_warn_days: 0,
        change_inactive_days: 0,
        expire_date: 0,
        reserved: 0,
    }
}

impl PasswdHooks for KeycloakNssPasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let res = list_users(&conf)
                    .map(|users| {
                        users.into_iter()
                            .map(|u| user_to_passwd(&u, &conf))
                            .collect()
                    });
                match res {
                    Ok(_res) => {
                        Response::Success(_res)
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> Response<Passwd> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let passwd_list: Result<Option<_>, _> = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .find(|u| u.uid == uid)
                            .map(|u| user_to_passwd(&u, conf))
                    });
                match passwd_list {
                    Ok(Some(res)) => {
                        Response::Success(res)
                    }
                    Ok(None) => {
                        Response::NotFound
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let passwd_list: Result<Option<_>, _> = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .find(|u| u.name == name)
                            .map(|u| user_to_passwd(&u, conf))
                    });
                match passwd_list {
                    Ok(Some(res)) => {
                        Response::Success(res)
                    }
                    Ok(None) => {
                        Response::NotFound
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }
}

struct KeycloakNssGroup;
libnss_group_hooks!(keycloak, KeycloakNssGroup);

impl GroupHooks for KeycloakNssGroup {
    fn get_all_entries() -> Response<Vec<Group>> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let res = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .map(|u| user_to_group(&u))
                            .collect()
                    });
                match res {
                    Ok(_res) => {
                        Response::Success(_res)
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }

    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let passwd_list: Result<Option<_>, _> = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .find(|u| u.uid == gid)
                            .map(|u| user_to_group(&u))
                    });
                match passwd_list {
                    Ok(Some(res)) => {
                        Response::Success(res)
                    }
                    Ok(None) => {
                        Response::NotFound
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let passwd_list: Result<Option<_>, _> = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .find(|u| u.name == name)
                            .map(|u| user_to_group(&u))
                    });
                match passwd_list {
                    Ok(Some(res)) => {
                        Response::Success(res)
                    }
                    Ok(None) => {
                        Response::NotFound
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }
}

struct KeycloakNssShadow;
libnss_shadow_hooks!(keycloak, KeycloakNssShadow);

impl ShadowHooks for KeycloakNssShadow {
    fn get_all_entries() -> Response<Vec<Shadow>> {
        // TODO: Ensure we are a privileged user before returning results

        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let res = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .map(|u| user_to_shadow(&u))
                            .collect()
                    });
                match res {
                    Ok(_res) => {
                        Response::Success(_res)
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Shadow> {
        // TODO: Ensure we are a privileged user before returning results

        match CONFIG.lock().unwrap().as_ref() {
            None => Response::Unavail,
            Some(conf) => {
                let passwd_list: Result<Option<_>, _> = list_users(conf)
                    .map(|users| {
                        users.into_iter()
                            .find(|u| u.name == name)
                            .map(|u| user_to_shadow(&u))
                    });
                match passwd_list {
                    Ok(Some(res)) => {
                        Response::Success(res)
                    }
                    Ok(None) => {
                        Response::NotFound
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Response::Unavail
                    }
                }
            }
        }
    }
}
