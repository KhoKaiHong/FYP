use crate::auth;
use crate::auth::token::{validate_access_token, validate_refresh_token};
use crate::context::Context;
use crate::model::ModelManager;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tracing::debug;

pub async fn mw_require_auth(
    ctx: Result<Context>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mm: State<ModelManager>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(Error::ContextExtractor(
            ContextExtractorError::AccessTokenNotInHeader,
        ))?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| Error::ContextExtractor(ContextExtractorError::InvalidAccessToken))?;

    let access_token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(Error::ContextExtractor(
            ContextExtractorError::InvalidAccessToken,
        ))?
        .to_string();

    let claims = validate_access_token(&access_token).map_err(|err| match err {
        auth::Error::AccessTokenExpired => {
            Error::ContextExtractor(ContextExtractorError::AccessTokenExpired)
        }
        _ => Error::ContextExtractor(ContextExtractorError::AccessTokenExpired),
    })?;

    let role = claims
        .role()
        .map_err(|_| Error::ContextExtractor(ContextExtractorError::AccessTokenExpired))?;

    let context = Context::new(claims.id(), role);

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(context);

    Ok(next.run(req).await)
}

// region:    --- Context Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<ContextExtractorResult>()
            .ok_or(Error::ContextExtractor(
                ContextExtractorError::ContextNotInRequestExtractor,
            ))?
            .clone()
            .map_err(Error::ContextExtractor)
    }
}

// endregion: --- Context Extractor

// region:    --- Context Extractor Result/Error
type ContextExtractorResult = core::result::Result<Context, ContextExtractorError>;

#[derive(Clone, Serialize, Debug)]
pub enum ContextExtractorError {
    AccessTokenNotInHeader,
    AccessTokenExpired,
    InvalidAccessToken,
    ContextNotInRequestExtractor,
    ContextCreateFail(String),
}
// endregion: --- Context Extractor Result/Error
