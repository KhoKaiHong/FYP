use crate::auth::Role;
use crate::context::Context;
use crate::model::admin::AdminModelController;
use crate::model::facility::FacilityModelController;
use crate::model::organiser::OrganiserModelController;
use crate::model::user::UserModelController;
use crate::model::EntityErrorField::I64Error;
use crate::model::{self, ModelManager};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/get-credentials", get(get_credentials_handler))
        .with_state(app_state)
}

async fn get_credentials_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - get_credentials_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let body: Json<Value>;

    match context.role() {
        Role::User => body = get_user_credentials(&context, model_manager).await?,
        Role::BloodCollectionFacility => {
            body = get_facility_credentials(&context, model_manager).await?
        }
        Role::Organiser => body = get_organiser_credentials(&context, model_manager).await?,
        Role::Admin => body = get_admin_credentials(&context, model_manager).await?,
    }

    Ok(body)
}

async fn get_user_credentials(
    context: &Context,
    model_manager: &ModelManager,
) -> Result<Json<Value>> {
    let user = UserModelController::get(context, model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "user",
                field: I64Error(id),
            } if id == context.user_id() => Error::NoUserFound,
            _ => Error::ModelError(err),
        })?;

    let body = Json(json!({
        "data": {
            "userDetails": user,
        }
    }));

    Ok(body)
}

async fn get_facility_credentials(
    context: &Context,
    model_manager: &ModelManager,
) -> Result<Json<Value>> {
    let facility = FacilityModelController::get(context, model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "facility",
                field: I64Error(id),
            } if id == context.user_id() => Error::NoUserFound,
            _ => Error::ModelError(err),
        })?;

    let body = Json(json!({
        "data": {
            "facilityDetails": facility,
        }
    }));

    Ok(body)
}

async fn get_organiser_credentials(
    context: &Context,
    model_manager: &ModelManager,
) -> Result<Json<Value>> {
    let organiser = OrganiserModelController::get(context, model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "organiser",
                field: I64Error(id),
            } if id == context.user_id() => Error::NoUserFound,
            _ => Error::ModelError(err),
        })?;

    let body = Json(json!({
        "data": {
            "organiserDetails": organiser,
        }
    }));

    Ok(body)
}

async fn get_admin_credentials(
    context: &Context,
    model_manager: &ModelManager,
) -> Result<Json<Value>> {
    let admin = AdminModelController::get(context, model_manager, context.user_id())
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "admin",
                field: I64Error(id),
            } if id == context.user_id() => Error::NoUserFound,
            _ => Error::ModelError(err),
        })?;

    let body = Json(json!({
        "data": {
            "adminDetails": admin,
        }
    }));

    Ok(body)
}
