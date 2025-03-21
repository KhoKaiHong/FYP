// Modules
use crate::auth::{token::parse_refresh_token, Role};
use crate::context::Context;
use crate::model::admin_session::AdminSessionModelController;
use crate::model::facility_session::FacilitySessionModelController;
use crate::model::organiser_session::OrganiserSessionModelController;
use crate::model::user_session::UserSessionModelController;
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{self, ModelManager};
use crate::state::AppState;
use crate::web::{Error, Result};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

// Routes
pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/logout-all", post(logout_all_handler))
        .with_state(app_state)
}

// Handler that logs out all sessions for a user
async fn logout_all_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<LogoutAllRequestPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - logout_all_api", "HANDLER");

    let model_manager = &app_state.model_manager;
    let refresh_token_claims = parse_refresh_token(&payload.refresh_token)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::LogoutFailInvalidRefreshToken)?;

    // Depending on the user performing the request, logout all sessions for that user
    match context.role() {
        Role::User => {
            logout_all_user(&context, refresh_token_jti, model_manager).await?;
        }
        Role::BloodCollectionFacility => {
            logout_all_facility(&context, refresh_token_jti, model_manager).await?;
        }
        Role::Organiser => {
            logout_all_organiser(&context, refresh_token_jti, model_manager).await?;
        }
        Role::Admin => {
            logout_all_admin(&context, refresh_token_jti, model_manager).await?;
        }
    }

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

async fn logout_all_user(
    context: &Context,
    refresh_token_jti: Uuid,
    model_manager: &ModelManager,
) -> Result<()> {
    // Checks if the user session exists
    UserSessionModelController::check(
        model_manager,
        refresh_token_jti,
        context.token_id(),
        context.user_id(),
    )
    .await
    .map_err(|err| match err {
        model::Error::EntityNotFound {
            entity: "user_session",
            field: UuidError(refresh_token_jti),
        } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
        _ => Error::ModelError(err),
    })?;

    // Deletes all user sessions for the user
    UserSessionModelController::delete_by_user_id(model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "user_session",
                field: I64Error(user_id),
            } if user_id == context.user_id() => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    Ok(())
}

async fn logout_all_facility(
    context: &Context,
    refresh_token_jti: Uuid,
    model_manager: &ModelManager,
) -> Result<()> {
    // Checks if the facility session exists
    FacilitySessionModelController::check(
        model_manager,
        refresh_token_jti,
        context.token_id(),
        context.user_id(),
    )
    .await
    .map_err(|err| match err {
        model::Error::EntityNotFound {
            entity: "facility_session",
            field: UuidError(refresh_token_jti),
        } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
        _ => Error::ModelError(err),
    })?;

    // Deletes all facility sessions for the facility
    FacilitySessionModelController::delete_by_facility_id(model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "facility_session",
                field: I64Error(facility_id),
            } if facility_id == context.user_id() => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    Ok(())
}

async fn logout_all_organiser(
    context: &Context,
    refresh_token_jti: Uuid,
    model_manager: &ModelManager,
) -> Result<()> {
    // Checks if the organiser session exists
    OrganiserSessionModelController::check(
        model_manager,
        refresh_token_jti,
        context.token_id(),
        context.user_id(),
    )
    .await
    .map_err(|err| match err {
        model::Error::EntityNotFound {
            entity: "organiser_session",
            field: UuidError(refresh_token_jti),
        } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
        _ => Error::ModelError(err),
    })?;

    // Deletes all organiser sessions for the organiser
    OrganiserSessionModelController::delete_by_organiser_id(model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "organiser_session",
                field: I64Error(organiser_id),
            } if organiser_id == context.user_id() => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    Ok(())
}

async fn logout_all_admin(
    context: &Context,
    refresh_token_jti: Uuid,
    model_manager: &ModelManager,
) -> Result<()> {
    // Checks if the admin session exists
    AdminSessionModelController::check(
        model_manager,
        refresh_token_jti,
        context.token_id(),
        context.user_id(),
    )
    .await
    .map_err(|err| match err {
        model::Error::EntityNotFound {
            entity: "admin_session",
            field: UuidError(refresh_token_jti),
        } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
        _ => Error::ModelError(err),
    })?;

    // Deletes all admin sessions for the admin
    AdminSessionModelController::delete_by_admin_id(model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "admin_session",
                field: I64Error(admin_id),
            } if admin_id == context.user_id() => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    Ok(())
}

// Request payload for logging out all sessions for a user
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct LogoutAllRequestPayload {
    refresh_token: String,
}
