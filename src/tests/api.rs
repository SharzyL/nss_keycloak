use log::{info};
use crate::api::{list_users, request_token, do_refresh_token};
use crate::config::{read_config};

#[test]
fn test_list_users() {
    let conf = read_config("kcnss.toml").unwrap();
    let users = list_users(&conf).unwrap();
    for user in users {
        info!("find user '{}' (uid '{}', github_id: '{}')", user.name, user.uid, user.github_id)
    }
}

#[test]
fn test_refresh() {
    let conf = read_config("kcnss.toml").unwrap();
    let token = request_token(&conf).unwrap();
    do_refresh_token(&token, &conf).unwrap();
}
