use std::sync::Arc;

use crate::{auth, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailPasswordNotMatching,

    // -- Access Token Errors
    AccessTokenExpired,
    InvalidAccessToken,

    // -- Refresh Token Errors
    RefreshTokenExpired,

    // -- Context Errors
    ContextExtractor(web::middleware_auth::ContextExtractorError),

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
            LoginFailUsernameNotFound => (StatusCode::UNAUTHORIZED, ClientError::USERNAME_NOT_FOUND),
            LoginFailPasswordNotMatching => (StatusCode::UNAUTHORIZED, ClientError::INCORRECT_PASSWORD),

            // -- Access Token Errors
            AccessTokenExpired => (StatusCode::UNAUTHORIZED, ClientError::ACCESS_TOKEN_EXPIRED),
            InvalidAccessToken => (StatusCode::UNAUTHORIZED, ClientError::INVALID_ACCESS_TOKEN),

            // -- Refresh Token Errors
            RefreshTokenExpired => (StatusCode::UNAUTHORIZED, ClientError::SESSION_EXPIRED),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    USERNAME_NOT_FOUND,
    INCORRECT_PASSWORD,
    ACCESS_TOKEN_EXPIRED,
    INVALID_ACCESS_TOKEN,
    SESSION_EXPIRED,
    NO_AUTH,
    SERVICE_ERROR,
}
// endregion: --- Client Error
