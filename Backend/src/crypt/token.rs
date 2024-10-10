use super::Role;
use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::{
    format_time, now_add_sec, now_utc, parse_utc_from_str, parse_utc_from_timestamp,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    id: i64,
    role: String,
    iat: i64,
    exp: i64,
}

impl AccessTokenClaims {
    fn new(id: i64, role: Role) -> Self {
        let duration: i64;

        match role {
            Role::User => duration = 900,
            Role::Organiser => duration = 900,
            Role::BloodCollectionFacility => duration = 600,
            Role::Admin => duration = 300,
        }

        AccessTokenClaims {
            id,
            role: role.to_string(),
            iat: now_utc().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }
}

pub fn generate_access_token(id: i64, role: Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &AccessTokenClaims::new(id, role),
        &EncodingKey::from_secret(&config().ACCESS_TOKEN_KEY),
    )
    .map_err(|_| Error::FailGenerateAccessToken)
}

pub fn validate_access_token(access_token: String) -> Result<()> {
    let claims = jsonwebtoken::decode::<AccessTokenClaims>(
        &access_token,
        &DecodingKey::from_secret(&config().ACCESS_TOKEN_KEY),
        &Validation::default(),
    )
    .map_err(|_| Error::RefreshTokenInvalidFormat)?
    .claims;

    let exp =
        parse_utc_from_timestamp(claims.exp).map_err(|_| Error::AccessTokenExpInvalidFormat)?;

    if exp < now_utc() {
        return Err(Error::AccessTokenExpired);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    jti: String,
    iat: i64,
    exp: i64,
}

impl RefreshTokenClaims {
    fn new(role: Role) -> Self {
        let duration: i64;

        match role {
            // 15 days
            Role::User => duration = 1296000,
            Role::Organiser => duration = 1296000,
            // 7 days
            Role::BloodCollectionFacility => duration = 604800,
            // 3 days
            Role::Admin => duration = 259200,
        }

        RefreshTokenClaims {
            jti: Uuid::new_v4().to_string(),
            iat: now_utc().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }
}

pub fn generate_refresh_token(role: Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &RefreshTokenClaims::new(role),
        &EncodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
    )
    .map_err(|_| Error::FailGenerateRefreshToken)
}

pub fn validate_refresh_token(refresh_token: String) -> Result<()> {
    let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        &refresh_token,
        &DecodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
        &Validation::default(),
    )
    .map_err(|_| Error::RefreshTokenInvalidFormat)?
    .claims;

    let exp =
        parse_utc_from_timestamp(claims.exp).map_err(|_| Error::RefreshTokenExpInvalidFormat)?;

    if exp < now_utc() {
        return Err(Error::RefreshTokenExpired);
    }

    Ok(())
}
