use crate::context::Context;
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
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
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // FIXME - Compute real CtxAuthResult<Ctx>.
    let result_ctx =
        Context::new(100).map_err(|ex| ContextExtractorError::ContextCreateFail(ex.to_string()));

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(ContextExtractorError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

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
			.ok_or(Error::ContextExtractor(ContextExtractorError::ContextNotInRequestExtractor))?
			.clone()
			.map_err(Error::ContextExtractor)
    }
}

// endregion: --- Context Extractor

// region:    --- Context Extractor Result/Error
type ContextExtractorResult = core::result::Result<Context, ContextExtractorError>;

#[derive(Clone, Serialize, Debug)]
pub enum ContextExtractorError {
    TokenNotInCookie,
    ContextNotInRequestExtractor,
    ContextCreateFail(String),
}
// endregion: --- Context Extractor Result/Error
