use crate::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL- WHILE LOADING CONFIG - Cause: {ex:?}")
        })
    })
}

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

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}