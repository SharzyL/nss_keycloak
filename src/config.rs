use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub username: String,
    pub password: String,
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub group_id: String,
}

lazy_static! {
    pub(crate) static ref CONFIG: Mutex<Config> = Mutex::new(init_config());
}

fn init_config() -> Config {
    let conf_file_path = env::var_os("KEYCLOAK_NSS_CONF")
        .map(|s| String::from(s.to_str().expect("invalid unicode for env[KEYCLOAK_NSS_CONF]")))
        .unwrap_or("kcnss.toml".to_string());

    let mut conf_file = File::open(&conf_file_path)
        .map_err(|e| panic!("failed to open file '{}' from '{}': {:?}",
                             &conf_file_path,
                             env::current_dir().unwrap().to_str().unwrap(), e)
        )
        .unwrap();

    let mut buffer = String::new();
    conf_file.read_to_string(&mut buffer)
        .map_err(|e| panic!("failed to read file '{}' as string: {:?}", &conf_file_path, e))
        .unwrap()
    ;

    return toml::from_str::<Config>(&buffer)
        .map_err(|e| panic!("failed to parse '{conf_file_path}': {:?}", e))
        .unwrap();
}
