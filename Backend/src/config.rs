use crate::{Error, Result};
use std::env;

#[allow(non_snake_case)]
pub struct Config {
    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
           // -- Web
           WEB_FOLDER: env::var("SERVICE_WEB_FOLDER").unwrap(), 
        })
    }
}

// fn get_env(name: &'static str) -> Result<String> {
//     env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
// }