use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

use lazy_static::lazy_static;
use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub username: String,
    pub password: String,
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub group_id: String,
    pub default_shell: String,
}

lazy_static! {
    pub(crate) static ref CONFIG: Mutex<Option<Config>> = Mutex::new(init_config());
}

fn init_config() -> Option<Config> {
    let _ = env_logger::builder().is_test(false).try_init();

    let conf_file_path = env::var_os("KEYCLOAK_NSS_CONF")
        .map(|s| String::from(s.to_str().expect("invalid unicode for env[KEYCLOAK_NSS_CONF]")))
        .unwrap_or("/etc/kcnss.toml".to_string());

    read_config(&conf_file_path)
}

pub(crate) fn read_config(config_file_path: &str) -> Option<Config> {
    let mut conf_file = File::open(config_file_path)
        .map_err(|e| error!("failed to open file '{}' from '{}': {:?}",
                             &config_file_path,
                             env::current_dir().unwrap().to_str().unwrap(), e)
        ).ok()?;


    let mut buffer = String::new();
    conf_file.read_to_string(&mut buffer)
        .map_err(|e| error!("failed to read file '{}' as string: {:?}", &config_file_path, e))
        .ok()?
    ;

    toml::from_str::<Config>(&buffer)
        .map_err(|e| error!("failed to parse '{}': {:?}", config_file_path, e))
        .ok()
}