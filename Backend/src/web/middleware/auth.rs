use crate::auth::Role;
use crate::context::Context;
use crate::web::{Error, Result};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::debug;

// Requires valid access token to continue
pub async fn require_auth(
    ctx: Result<Context>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// Requires user role to continue
pub async fn require_user(ctx: Context, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - require_user - {ctx:?}", "MIDDLEWARE");

    if let Role::User = ctx.role() {
        Ok(next.run(req).await)
    } else {
        Err(Error::UserRoleRequired)
    }
}

// Requires blood collection facility role to continue
pub async fn require_facility(ctx: Context, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - require_facility - {ctx:?}", "MIDDLEWARE");

    if let Role::BloodCollectionFacility = ctx.role() {
        Ok(next.run(req).await)
    } else {
        Err(Error::BloodCollectionFacilityRoleRequired)
    }
}

// Requires organiser role to continue
pub async fn require_organiser(ctx: Context, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - require_organiser - {ctx:?}", "MIDDLEWARE");

    if let Role::Organiser = ctx.role() {
        Ok(next.run(req).await)
    } else {
        Err(Error::OrganiserRoleRequired)
    }
}

// Requires admin role to continue
pub async fn require_admin(ctx: Context, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - require_admin - {ctx:?}", "MIDDLEWARE");

    if let Role::Admin = ctx.role() {
        Ok(next.run(req).await)
    } else {
        Err(Error::AdminRoleRequired)
    }
}
