use crate::auth::token::{
    parse_access_token, validate_access_token, validate_refresh_token, AccessTokenClaims,
};
use crate::auth::{
    self, password::validate_password, token::generate_access_token, token::generate_refresh_token,
    Role,
};
use crate::context::Context;
use crate::model::facility::FacilityModelController;
use crate::model::facility_session::{FacilitySessionForCreate, FacilitySessionModelController};
use crate::model::organiser::OrganiserModelController;
use crate::model::organiser_session::{OrganiserSessionForCreate, OrganiserSessionModelController};
use crate::model::user::UserModelController;
use crate::model::user_session::{UserSessionForCreate, UserSessionModelController};
use crate::model::EntityErrorField::UuidError;
use crate::model::{self, ModelManager};
use crate::state::AppState;
use crate::web::middleware_auth::{ContextExtractorError, ContextExtractorResult};
use crate::web::{Error, Result};
use axum::extract::State;
use axum::http::header::REFRESH;
use axum::http::{header, HeaderMap};
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use uuid::timestamp::context;
use uuid::Uuid;

async fn refresh_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RefreshRequestPayload>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    debug!("{:<12} - refresh_api", "HANDLER");

    let auth_header = headers
        .get(header::AUTHORIZATION)
        .ok_or(Error::InvalidRefreshAttempt)?
        .to_str()
        .map_err(|_| Error::InvalidRefreshAttempt)?;

    let access_token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(Error::InvalidRefreshAttempt)?
        .to_string();

    let validation_result = validate_access_token(&access_token);

    if let Err(auth::Error::AccessTokenExpired) = validation_result {
        let model_manager = &app_state.model_manager;
        let context = Context::root_ctx();

        let access_token_claims =
            parse_access_token(&access_token).map_err(|err| Error::AuthError(err))?;

        let role = access_token_claims
            .role()
            .map_err(|_| Error::InvalidRefreshAttempt)?;

        match role {
            Role::User => {
                refresh_user_token(&context, model_manager, &access_token_claims, &payload).await;
            }
            Role::BloodCollectionFacility => {
                refresh_facility_token(&context, model_manager, &access_token_claims, &payload)
                    .await;
            }
            Role::Organiser => {
                refresh_organiser_token(&context, model_manager, &access_token_claims, &payload)
                    .await;
            }
            Role::Admin => {
                refresh_admin_token(&context, model_manager, &access_token_claims, &payload).await;
            }
        }

        let refresh_token_claims =
            validate_refresh_token(&payload.refresh_token).map_err(|err| match err {
                auth::Error::RefreshTokenExpired => Error::RefreshTokenExpired,
                _ => Error::AuthError(err),
            })?;

        let access_token_jti = Uuid::from_slice(access_token_claims.jti().as_bytes())
            .map_err(|_| Error::InvalidRefreshAttempt)?;

        let refresh_token_jti = Uuid::from_slice(refresh_token_claims.jti().as_bytes())
            .map_err(|_| Error::InvalidRefreshAttempt)?;

        let user_session =
            UserSessionModelController::get(&context, model_manager, refresh_token_jti)
                .await
                .map_err(|err| match err {
                    model::Error::EntityNotFound {
                        entity: "user_session",
                        field: UuidError(refresh_token_jti),
                    } if refresh_token_jti == refresh_token_jti => Error::InvalidRefreshAttempt,
                    _ => Error::ModelError(err),
                })?;

        if user_session.user_id == access_token_claims.id()
            && user_session.access_token_id == access_token_jti
        {
            todo!()
        }

        let body = Json(json!({
            "result": {
                "success": true,
                "access_token": access_token,
                // "refresh_token": refresh_token,
            }
        }));

        Ok(body)
    } else if validation_result.is_ok() {
        Err(Error::InvalidRefreshAttempt)
    } else {
        Err(Error::AuthError(validation_result.unwrap_err()))
    }
}

async fn refresh_user_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<()> {
    Ok(())
}

async fn refresh_facility_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<()> {
    Ok(())
}

async fn refresh_organiser_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<()> {
    Ok(())
}

async fn refresh_admin_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<()> {
    Ok(())
}

#[derive(Debug, Deserialize)]
struct RefreshRequestPayload {
    refresh_token: String,
}
