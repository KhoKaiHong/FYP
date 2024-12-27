// Modules
use crate::auth;
use crate::auth::token::validate_access_token;
use crate::context::Context;
use crate::state::AppState;
use crate::web::{Error, Result};

use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use serde::Serialize;
use tracing::debug;
use uuid::Uuid;

// Converts access token claims to context
pub async fn context_resolver(
    State(_app_state): State<AppState>,
    header: Option<TypedHeader<Authorization<Bearer>>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - context_resolver", "MIDDLEWARE");

    let context = context_from_token(header).await;

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(context);

    next.run(req).await
}

async fn context_from_token(header: Option<TypedHeader<Authorization<Bearer>>>) -> ContextExtractorResult {
    let TypedHeader(header) = header.ok_or(ContextExtractorError::InvalidAccessToken)?;

    let access_token = header.token();

    let claims = validate_access_token(access_token).map_err(|err| match err {
        auth::Error::AccessTokenExpired => ContextExtractorError::AccessTokenExpired,
        _ => ContextExtractorError::InvalidAccessToken,
    })?;

    let role = claims
        .role()
        .map_err(|_| ContextExtractorError::InvalidAccessToken)?;

    let jti = Uuid::try_parse(claims.jti()).map_err(|_| ContextExtractorError::InvalidAccessToken)?;

    let context = Context::new(claims.id(), role, jti);

    Ok(context)
}

// Context extractor from a request
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Context", "EXTRACTOR");

        parts
            .extensions
            .get::<ContextExtractorResult>()
            .ok_or(Error::ContextExtractor(
                ContextExtractorError::ContextNotInRequestExtractor,
            ))?
            .clone()
            .map_err(|err| match err {
                ContextExtractorError::AccessTokenExpired => Error::AccessTokenExpired,
                _ => Error::ContextExtractor(err),
            })
    }
}

// Context Extractor Result and Errors
pub type ContextExtractorResult = core::result::Result<Context, ContextExtractorError>;

#[derive(Clone, Serialize, Debug)]
pub enum ContextExtractorError {
    AccessTokenExpired,
    InvalidAccessToken,
    ContextNotInRequestExtractor,
}
