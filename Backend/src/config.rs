use crate::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL- WHILE LOADING CONFIG - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Crypt
    pub ACCESS_TOKEN_KEY: Vec<u8>,
    pub ACCESS_TOKEN_DURATION: f64,
    pub REFRESH_TOKEN_KEY: Vec<u8>,
    pub REFRESH_TOKEN_DURATION: f64,

    // -- Database
    pub DATABASE_URL: String,

    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Crypt
            ACCESS_TOKEN_KEY: get_env_b64u_as_u8s("ACCESS_TOKEN_KEY")?,
            ACCESS_TOKEN_DURATION: get_env_parse("ACCESS_TOKEN_DURATION")?,

            REFRESH_TOKEN_KEY: get_env_b64u_as_u8s("REFRESH_TOKEN_KEY")?,
            REFRESH_TOKEN_DURATION: get_env_parse("REFRESH_TOKEN_DURATION")?,

            // -- Database
            DATABASE_URL: get_env("SERVICE_DB_URL")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
