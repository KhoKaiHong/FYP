use crate::{Error, Result};
use serde::Deserialize;
use serde_with::{base64::Base64, base64::UrlSafe, serde_as};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::{str::FromStr, sync::OnceLock};

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseConfig {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }
}

#[serde_as]
#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    #[serde_as(as = "Base64<UrlSafe>")]
    pub access_token_key: Vec<u8>,
    #[serde_as(as = "Base64<UrlSafe>")]
    pub refresh_token_key: Vec<u8>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EmailConfig {
    pub resend_api_key: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FileConfig {
    pub web_folder: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FrontendConfig {
    pub frontend_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub application: ApplicationConfig,
    pub database: DatabaseConfig,
    pub email_client: EmailConfig,
    pub file_serving: FileConfig,
    pub frontend: FrontendConfig,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_configuration()
            .unwrap_or_else(|ex| panic!("FATAL- WHILE LOADING CONFIG - Cause: {ex:?}"))
    })
}

impl Config {
    pub fn load_configuration() -> Result<Config> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("configuration");

        // Detect the running environment. Default to `local` if unspecified.
        let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let environment =
            Environment::from_str(&environment).expect("Failed to parse APP_ENVIRONMENT.");
        let environment_filename = format!("{}.yaml", environment.to_string());

        let config = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            .build()
            .map_err(|_| Error::FailToBuildConfig)?;

        config
            .try_deserialize::<Config>()
            .map_err(|_| Error::FailToBuildConfig)
    }
}

#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum Environment {
    #[strum(serialize = "development")]
    Development,
    #[strum(serialize = "production")]
    Production,
}

// pub fn config() -> &'static Config {
//     static INSTANCE: OnceLock<Config> = OnceLock::new();

//     INSTANCE.get_or_init(|| {
//         Config::load_from_env()
//             .unwrap_or_else(|ex| panic!("FATAL- WHILE LOADING CONFIG - Cause: {ex:?}"))
//     })
// }

// #[allow(non_snake_case)]
// pub struct Config {
//     // -- Auth
//     pub ACCESS_TOKEN_KEY: Vec<u8>,
//     pub REFRESH_TOKEN_KEY: Vec<u8>,

//     // -- Database
//     pub DATABASE_URL: String,

//     // -- Web
//     pub WEB_FOLDER: String,

//     // -- Email Service
//     pub RESEND_API_KEY: String,

//     // -- Frontend URL
//     pub FRONTEND_URL: String,
// }

// impl Config {
//     fn load_from_env() -> Result<Config> {
//         Ok(Config {
//             // -- Auth
//             ACCESS_TOKEN_KEY: get_env_b64u_as_u8("ACCESS_TOKEN_KEY")?,
//             REFRESH_TOKEN_KEY: get_env_b64u_as_u8("REFRESH_TOKEN_KEY")?,

//             // -- Database
//             DATABASE_URL: get_env("DEV_DB_URL")?,

//             // -- Web
//             WEB_FOLDER: get_env("WEB_FOLDER")?,

//             // -- Email Service
//             RESEND_API_KEY: get_env("RESEND_API_KEY")?,

//             // -- Frontend URL
//             FRONTEND_URL: get_env("DEV_FRONTEND_URL")?,
//         })
//     }
// }

// fn get_env(name: &'static str) -> Result<String> {
//     env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
// }

// fn get_env_b64u_as_u8(name: &'static str) -> Result<Vec<u8>> {
//     base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
// }
