use log::{info};
use crate::api::{list_users, request_token, do_refresh_token};

#[test]
fn test_list_users() {
    let _ = env_logger::builder().is_test(false).try_init();

    let users = list_users().unwrap();
    for user in users {
        info!("find user '{}' (uid '{}', github_id: '{}')", user.name, user.uid, user.github_id)
    }
}

#[test]
fn test_refresh() {
    let _ = env_logger::builder().is_test(false).try_init();

    let token = request_token().unwrap();
    do_refresh_token(&token).unwrap();
}