use super::Role;
use crate::auth::{Error, Result};
use crate::config;
use crate::utils::{
    format_time, now_add_sec, now_utc, parse_utc_from_str, parse_utc_from_timestamp,
};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    jti: String,
    id: i64,
    role: String,
    iat: i64,
    exp: i64,
}

impl AccessTokenClaims {
    fn new(jti: &str, id: i64, role: &Role) -> Self {
        let duration: i64;
        let jti = jti.to_string();

        match role {
            Role::User => duration = -1000,
            Role::Organiser => duration = 900,
            Role::BloodCollectionFacility => duration = 600,
            Role::Admin => duration = 300,
        }

        AccessTokenClaims {
            jti,
            id,
            role: role.to_string(),
            iat: now_utc().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }

    pub fn jti(&self) -> &str {
        &self.jti
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn role(&self) -> Result<Role> {
        Role::from_str(&self.role).map_err(|_| Error::AccessTokenInvalidFormat)
    }
}

pub fn generate_access_token(jti: &str, id: i64, role: &Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &AccessTokenClaims::new(jti, id, &role),
        &EncodingKey::from_secret(&config().ACCESS_TOKEN_KEY),
    )
    .map_err(|_| Error::FailGenerateAccessToken)
}

pub fn validate_access_token(access_token: &str) -> Result<AccessTokenClaims> {
    let claims = jsonwebtoken::decode::<AccessTokenClaims>(
        access_token,
        &DecodingKey::from_secret(&config().ACCESS_TOKEN_KEY),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Error::AccessTokenExpired,
        _ => Error::AccessTokenInvalidFormat,
    })?
    .claims;

    Ok(claims)
}

pub fn parse_access_token(access_token: &str) -> Result<AccessTokenClaims> {
    let mut validation_strategy = Validation::new(Algorithm::HS512);
    validation_strategy.validate_exp = false;

    let claims = jsonwebtoken::decode::<AccessTokenClaims>(
        access_token,
        &DecodingKey::from_secret(&config().ACCESS_TOKEN_KEY),
        &validation_strategy,
    )
    .map_err(|_| Error::AccessTokenInvalidFormat)?
    .claims;

    Ok(claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    jti: String,
    iat: i64,
    exp: i64,
}

impl RefreshTokenClaims {
    fn new(jti: &str, role: &Role) -> Self {
        let duration: i64;
        let jti = jti.to_string();

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
            jti,
            iat: now_utc().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }

    fn new_with_duration(jti: &str, duration: i64) -> Self {
        let jti = jti.to_string();

        RefreshTokenClaims {
            jti,
            iat: now_utc().timestamp(),
            exp: duration,
        }
    }

    pub fn jti(&self) -> &str {
        &self.jti
    }

    pub fn exp(&self) -> i64 {
        self.exp
    }
}

pub fn generate_refresh_token(jti: &str, role: &Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &RefreshTokenClaims::new(jti, role),
        &EncodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
    )
    .map_err(|_| Error::FailGenerateRefreshToken)
}

pub fn renew_refresh_token(jti: &str, duration: i64) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &RefreshTokenClaims::new_with_duration(jti, duration),
        &EncodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
    )
    .map_err(|_| Error::FailGenerateRefreshToken)
}

pub fn validate_refresh_token(refresh_token: &str) -> Result<RefreshTokenClaims> {
    let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token,
        &DecodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Error::RefreshTokenExpired,
        _ => Error::RefreshTokenInvalidFormat,
    })?
    .claims;

    Ok(claims)
}

pub fn parse_refresh_token(refresh_token: &str) -> Result<RefreshTokenClaims> {
    let mut validation_strategy = Validation::new(Algorithm::HS512);
    validation_strategy.validate_exp = false;

    let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token,
        &DecodingKey::from_secret(&config().REFRESH_TOKEN_KEY),
        &validation_strategy,
    )
    .map_err(|_| Error::RefreshTokenInvalidFormat)?
    .claims;

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::TimeDelta;
    use serial_test::serial;
    use uuid::Uuid;

    #[test]
    #[serial]
    fn access_token_test() -> Result<()> {
        let id = 10;
        let jti = Uuid::new_v4().to_string();
        let role = Role::BloodCollectionFacility;
        let access_token = generate_access_token(&jti, id, &role)?;

        let validation_result = validate_access_token(&access_token)?;

        assert_eq!(validation_result.role, role.to_string());
        assert_eq!(validation_result.id, id);
        assert_eq!(
            parse_utc_from_timestamp(validation_result.iat).unwrap()
                + TimeDelta::try_seconds(600).unwrap(),
            parse_utc_from_timestamp(validation_result.exp).unwrap()
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn refresh_token_test() -> Result<()> {
        let role = Role::User;
        let jti = Uuid::new_v4().to_string();
        let refresh_token = generate_refresh_token(&jti, &role)?;

        let validation_result = validate_refresh_token(&refresh_token)?;

        assert_eq!(
            parse_utc_from_timestamp(validation_result.iat).unwrap()
                + TimeDelta::try_seconds(1296000).unwrap(),
            parse_utc_from_timestamp(validation_result.exp).unwrap()
        );
        assert_eq!(jti, validation_result.jti);

        Ok(())
    }
}
