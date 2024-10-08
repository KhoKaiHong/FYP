use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::{format_time, now_add_sec, now_utc, parse_utc_from_str, parse_timestamp_from_utc};
use uuid;


#[derive(Debug)]
pub struct AccessTokenClaims {
    id: i64,
    role: String,
    iat: i64,
    exp: i64,
}


#[derive(Debug)]
pub struct RefreshTokenClaims {
    jti: String,
    iat: i64,
    exp: i64,
}
