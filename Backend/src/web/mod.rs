mod error;

pub mod middleware_auth;
pub mod routes_login;
pub mod response_map;
pub mod routes_static;
pub mod routes_hello;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";
