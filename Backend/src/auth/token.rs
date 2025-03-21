// Modules
use crate::auth::{Role, Error, Result};
use crate::config;
use crate::utils::now_add_sec;

use chrono::prelude::*;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// Access token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    jti: String,
    id: i64,
    role: String,
    iat: i64,
    exp: i64,
}

impl AccessTokenClaims {
    // Create a new access token
    fn new(jti: &str, id: i64, role: &Role) -> Self {
        let duration: i64;
        let jti = jti.to_string();

        match role {
            // 15 minutes
            Role::User => duration = 900,
            Role::Organiser => duration = 900,
            // 10 minutes
            Role::BloodCollectionFacility => duration = 600,
            // 5 minutes
            Role::Admin => duration = 300,
        }

        AccessTokenClaims {
            jti,
            id,
            role: role.to_string(),
            iat: Utc::now().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }

    // Get the jti
    pub fn jti(&self) -> &str {
        &self.jti
    }

    // Get the id
    pub fn id(&self) -> i64 {
        self.id
    }

    // Get the role
    pub fn role(&self) -> Result<Role> {
        Role::from_str(&self.role).map_err(|_| Error::AccessTokenInvalidFormat)
    }
}

// Function that generates an access token
pub fn generate_access_token(jti: &str, id: i64, role: &Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &AccessTokenClaims::new(jti, id, &role),
        &EncodingKey::from_secret(&config().application.access_token_key),
    )
    .map_err(|_| Error::FailGenerateAccessToken)
}

// Function that validates an access token (checks if it has expired)
pub fn validate_access_token(access_token: &str) -> Result<AccessTokenClaims> {
    let claims = jsonwebtoken::decode::<AccessTokenClaims>(
        access_token,
        &DecodingKey::from_secret(&config().application.access_token_key),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Error::AccessTokenExpired,
        _ => Error::AccessTokenInvalidFormat,
    })?
    .claims;

    Ok(claims)
}

// Function that parses an access token (does not check if it has expired)
pub fn parse_access_token(access_token: &str) -> Result<AccessTokenClaims> {
    let mut validation_strategy = Validation::new(Algorithm::HS512);
    validation_strategy.validate_exp = false;

    let claims = jsonwebtoken::decode::<AccessTokenClaims>(
        access_token,
        &DecodingKey::from_secret(&config().application.access_token_key),
        &validation_strategy,
    )
    .map_err(|_| Error::AccessTokenInvalidFormat)?
    .claims;

    Ok(claims)
}

// Refresh token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    jti: String,
    iat: i64,
    exp: i64,
}

impl RefreshTokenClaims {
    // Create a new refresh token
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
            iat: Utc::now().timestamp(),
            exp: now_add_sec(duration).timestamp(),
        }
    }

    // Create a new refresh token with a specific expiration
    fn new_with_duration(jti: &str, duration: i64) -> Self {
        let jti = jti.to_string();

        RefreshTokenClaims {
            jti,
            iat: Utc::now().timestamp(),
            exp: duration,
        }
    }

    // Get the jti
    pub fn jti(&self) -> &str {
        &self.jti
    }

    // Get the expiration
    pub fn exp(&self) -> i64 {
        self.exp
    }
}

// Function that generates a refresh token
pub fn generate_refresh_token(jti: &str, role: &Role) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &RefreshTokenClaims::new(jti, role),
        &EncodingKey::from_secret(&config().application.refresh_token_key),
    )
    .map_err(|_| Error::FailGenerateRefreshToken)
}

// Function that renews a refresh token
pub fn renew_refresh_token(jti: &str, duration: i64) -> Result<String> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &RefreshTokenClaims::new_with_duration(jti, duration),
        &EncodingKey::from_secret(&config().application.refresh_token_key),
    )
    .map_err(|_| Error::FailGenerateRefreshToken)
}

// Function that validates a refresh token (checks if it has expired)
pub fn validate_refresh_token(refresh_token: &str) -> Result<RefreshTokenClaims> {
    let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token,
        &DecodingKey::from_secret(&config().application.refresh_token_key),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Error::RefreshTokenExpired,
        _ => Error::RefreshTokenInvalidFormat,
    })?
    .claims;

    Ok(claims)
}

// Function that parses a refresh token (does not check if it has expired)
pub fn parse_refresh_token(refresh_token: &str) -> Result<RefreshTokenClaims> {
    let mut validation_strategy = Validation::new(Algorithm::HS512);
    validation_strategy.validate_exp = false;

    let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
        refresh_token,
        &DecodingKey::from_secret(&config().application.refresh_token_key),
        &validation_strategy,
    )
    .map_err(|_| Error::RefreshTokenInvalidFormat)?
    .claims;

    Ok(claims)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_utc_from_timestamp;
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
