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
        let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| String::from("development"));

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
