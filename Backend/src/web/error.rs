use std::sync::Arc;

use crate::model::registration::RegistrationError;
use crate::{auth, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
// #[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailEmailNotFound,
    LoginFailIcNotFound,
    LoginFailPasswordNotMatching,

    // -- Role
    UserRoleRequired,
    BloodCollectionFacilityRoleRequired,
    OrganiserRoleRequired,
    AdminRoleRequired,

    // -- No User Found
    NoUserFound,

    // -- Access Token Errors
    AccessTokenExpired,

    // -- Refresh Token Errors
    RefreshTokenExpired,

    // -- Refresh Request Errors
    RefreshFailInvalidAccessToken,
    RefreshFailInvalidRefreshToken,
    RefreshFailNoSessionFound,

    // -- Logout Request Errors
    LogoutFailInvalidRefreshToken,
    LogoutFailNoSessionFound,


    // Invalid Data Errors
    InvalidData(String),

    // -- Context Errors
    ContextExtractor(web::middleware::ContextExtractorError),

    // -- Model Error
    ModelError(model::Error),

    // -- Auth Error
    AuthError(auth::Error),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::ModelError(val)
    }
}

impl From<auth::Error> for Error {
    fn from(val: auth::Error) -> Self {
        Self::AuthError(val)
    }
}

impl From<web::middleware::ContextExtractorError> for Error {
    fn from(val: web::middleware::ContextExtractorError) -> Self {
        Self::ContextExtractor(val)
    }
}
// endregion: --- Froms

// region:    --- Client Error
/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        match self {
            // -- Context Extractor Errors
            ContextExtractor(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Auth Errors
            AuthError(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Login Fail
            LoginFailEmailNotFound => (StatusCode::UNAUTHORIZED, ClientError::EMAIL_NOT_FOUND),
            LoginFailIcNotFound => (StatusCode::UNAUTHORIZED, ClientError::IC_NOT_FOUND),
            LoginFailPasswordNotMatching => {
                (StatusCode::UNAUTHORIZED, ClientError::INCORRECT_PASSWORD)
            }

            // -- Role
            UserRoleRequired => (StatusCode::FORBIDDEN, ClientError::PERMISSION_DENIED),
            BloodCollectionFacilityRoleRequired => {
                (StatusCode::FORBIDDEN, ClientError::PERMISSION_DENIED)
            }
            OrganiserRoleRequired => (StatusCode::FORBIDDEN, ClientError::PERMISSION_DENIED),
            AdminRoleRequired => (StatusCode::FORBIDDEN, ClientError::PERMISSION_DENIED),

            // -- No User Found
            NoUserFound => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Access Token Errors
            AccessTokenExpired => (StatusCode::UNAUTHORIZED, ClientError::ACCESS_TOKEN_EXPIRED),

            // -- Refresh Token Errors
            RefreshTokenExpired => (StatusCode::UNAUTHORIZED, ClientError::SESSION_EXPIRED),

            // -- Refresh Request Errors
            RefreshFailInvalidAccessToken => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            RefreshFailInvalidRefreshToken => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            RefreshFailNoSessionFound => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Logout Request Errors
            LogoutFailInvalidRefreshToken => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            LogoutFailNoSessionFound => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Invalid Data Errors
            InvalidData(_) => (StatusCode::BAD_REQUEST, ClientError::INVALID_REQUEST),

            // -- Duplicate Record Errors
            ModelError(model::Error::DuplicateKey { table: _, column }) => (
                StatusCode::BAD_REQUEST,
                ClientError::DUPLICATE_RECORD(column.to_string()),
            ),
            
            // -- Event Registration Errors
            ModelError(model::Error::EventRegistration(RegistrationError::EventAtCapacity)) => (
                StatusCode::BAD_REQUEST,
                ClientError::EVENT_AT_CAPACITY,
            ),

            ModelError(model::Error::EventRegistration(RegistrationError::ExistingEventRegistration)) => (
                StatusCode::BAD_REQUEST,
                ClientError::EXISTING_EVENT_REGISTRATION,
            ),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    EMAIL_NOT_FOUND,
    IC_NOT_FOUND,
    INCORRECT_PASSWORD,
    ACCESS_TOKEN_EXPIRED,
    SESSION_EXPIRED,
    INVALID_REQUEST,
    NO_AUTH,
    SERVICE_ERROR,
    DUPLICATE_RECORD(String),
    PERMISSION_DENIED,
    EVENT_AT_CAPACITY,
    EXISTING_EVENT_REGISTRATION,
}
// endregion: --- Client Error
