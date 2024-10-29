use crate::auth::token::{validate_access_token, validate_refresh_token};
use crate::context::Context;
use crate::model::ModelManager;
use crate::state::AppState;
use crate::web::{Error, Result};
use crate::{auth, model};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{header, HeaderMap, Request};
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
    State(app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let model_manager = &app_state.model_manager;

    let header = req.headers();

    let context = context_from_token(header).await;

    // if let Err(err) = context {
    //     match err {
    //         ContextExtractorError::AccessTokenExpired => {
    //             return Err(Error::AccessTokenExpired);
    //         }
    //         ContextExtractorError::InvalidAccessToken => {
    //             return Err(Error::InvalidAccessToken);
    //         }
    //         _ => {
    //             return Err(Error::ContextExtractor(err));
    //         }
    //     }
    // }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(context);

    next.run(req).await
}

async fn context_from_token(header: &HeaderMap) -> ContextExtractorResult {
    let auth_header = header
        .get(header::AUTHORIZATION)
        .ok_or(ContextExtractorError::AccessTokenNotInHeader)?
        .to_str()
        .map_err(|_| ContextExtractorError::InvalidAccessToken)?;

    let access_token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ContextExtractorError::InvalidAccessToken)?
        .to_string();

    let claims = validate_access_token(&access_token).map_err(|err| match err {
        auth::Error::AccessTokenExpired => ContextExtractorError::AccessTokenExpired,
        _ => ContextExtractorError::InvalidAccessToken,
    })?;

    let role = claims
        .role()
        .map_err(|_| ContextExtractorError::InvalidAccessToken)?;

    let context = Context::new(claims.id(), role);

    Ok(context)
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
}
// endregion: --- Context Extractor Result/Error
