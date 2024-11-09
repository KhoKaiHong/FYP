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
    // -- Auth
    pub ACCESS_TOKEN_KEY: Vec<u8>,
    pub REFRESH_TOKEN_KEY: Vec<u8>,

    // -- Database
    pub DATABASE_URL: String,

    // -- Web
    pub WEB_FOLDER: String,

    // -- Email Service
    pub RESEND_API_KEY: String,

    // -- Frontend URL
    pub FRONTEND_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Auth
            ACCESS_TOKEN_KEY: get_env_b64u_as_u8("ACCESS_TOKEN_KEY")?,
            REFRESH_TOKEN_KEY: get_env_b64u_as_u8("REFRESH_TOKEN_KEY")?,

            // -- Database
            DATABASE_URL: get_env("DEV_DB_URL")?,

            // -- Web
            WEB_FOLDER: get_env("WEB_FOLDER")?,

            // -- Email Service
            RESEND_API_KEY: get_env("RESEND_API_KEY")?,

            // -- Frontend URL
            FRONTEND_URL: get_env("DEV_FRONTEND_URL")?,
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

fn get_env_b64u_as_u8(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
