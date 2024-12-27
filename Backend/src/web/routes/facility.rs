// Modules
use crate::auth;
use crate::auth::password::{encrypt_password, validate_password};
use crate::context::Context;
use crate::model::facility::{FacilityForUpdate, FacilityModelController};
use crate::state::AppState;
use crate::web::{Error, Result};

use axum::extract::State;
use axum::routing::{get, patch};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

// Route that updates a facility
pub fn update_route(app_state: AppState) -> Router {
    Router::new()
        .route("/facility", patch(facility_update_handler))
        .with_state(app_state)
}

// Route that lists all facilities
pub fn list_route(app_state: AppState) -> Router {
    Router::new()
        .route("/facilities", get(facility_list_handler))
        .with_state(app_state)
}

// Handler that updates a facility
async fn facility_update_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<FacilityUpdatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - update_facility_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    // If password is going to be updated
    if let (Some(password), Some(current_password)) = (payload.password, payload.current_password) {
        let facility =
            FacilityModelController::get(&app_state.model_manager, context.user_id()).await?;

        // Perform password validation
        validate_password(&current_password, &facility.password)
            .await
            .map_err(|err| match err {
                auth::Error::PasswordNotMatching => Error::CurrentPasswordNotMatching,
                _ => Error::AuthError(err),
            })?;

        // Encrypts the password
        let password_hash = encrypt_password(&password).await?;

        // Prepare update details with new password hash
        let updated_details = FacilityForUpdate {
            password: Some(password_hash),
            email: payload.email,
            name: payload.name,
            address: payload.address,
            phone_number: payload.phone_number,
        };

        // Update facility with new details
        FacilityModelController::update(model_manager, context.user_id(), updated_details).await?;
    } else {
        // If no password update, prepare update details without password
        let updated_details = FacilityForUpdate {
            password: None,
            email: payload.email,
            name: payload.name,
            address: payload.address,
            phone_number: payload.phone_number,
        };

        // Update facility with new details
        FacilityModelController::update(model_manager, context.user_id(), updated_details).await?;
    }

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

// Request payload for updating a facility
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct FacilityUpdatePayload {
    current_password: Option<String>,
    password: Option<String>,
    email: Option<String>,
    name: Option<String>,
    address: Option<String>,
    phone_number: Option<String>,
}

// Handler that lists all facilities
async fn facility_list_handler(State(app_state): State<AppState>) -> Result<Json<Value>> {
    debug!("{:<12} - list_facilities_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let facility_list = FacilityModelController::list(model_manager).await?;

    let body = Json(json!({
        "data": {
            "facilities": facility_list,
        }
    }));

    Ok(body)
}
