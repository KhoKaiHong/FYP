use crate::auth::{token::parse_refresh_token, Role};
use crate::context::Context;
use crate::model::admin_session::AdminSessionModelController;
use crate::model::facility_session::{self, FacilitySessionModelController};
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

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/logoutall", post(logout_all_handler))
        .with_state(app_state)
}

async fn logout_all_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<LogoutAllRequestPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - logout_api", "HANDLER");

    let model_manager = &app_state.model_manager;
    let refresh_token_claims = parse_refresh_token(&payload.refresh_token)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::LogoutFailInvalidRefreshToken)?;

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
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

async fn logout_all_user(
    context: &Context,
    refresh_token_jti: Uuid,
    model_manager: &ModelManager,
) -> Result<()> {
    UserSessionModelController::check(&context, model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    UserSessionModelController::delete_by_user_id(&context, model_manager, context.user_id())
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
    FacilitySessionModelController::check(&context, model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "facility_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    FacilitySessionModelController::delete_by_facility_id(
        &context,
        model_manager,
        context.user_id(),
    )
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
    OrganiserSessionModelController::check(&context, model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "organiser_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    OrganiserSessionModelController::delete_by_organiser_id(
        &context,
        model_manager,
        context.user_id(),
    )
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
    AdminSessionModelController::check(&context, model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "admin_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::LogoutFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    AdminSessionModelController::delete_by_admin_id(&context, model_manager, context.user_id())
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

#[derive(Debug, Deserialize)]
struct LogoutAllRequestPayload {
    refresh_token: String,
}
