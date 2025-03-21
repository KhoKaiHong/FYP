// Modules
use crate::auth::password::encrypt_password;
use crate::model::admin::{AdminForCreate, AdminModelController};
use crate::model::enums::BloodType;
use crate::model::facility::{FacilityForCreate, FacilityModelController};
use crate::model::organiser::{OrganiserForCreate, OrganiserModelController};
use crate::model::user::{UserForCreate, UserModelController};
use crate::state::AppState;
use crate::web::{Error, Result};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use tracing::debug;

// Register routes that are available to all users
pub fn routes_public(app_state: AppState) -> Router {
    Router::new()
        .route("/user-register", post(user_register_handler))
        .route("/organiser-register", post(organiser_register_handler))
        .with_state(app_state)
}

// Register routes that are available to admins only
pub fn routes_admin(app_state: AppState) -> Router {
    Router::new()
        .route("/facility-register", post(facility_register_handler))
        .route("/admin-register", post(admin_register_handler))
        .with_state(app_state)
}

// Handler that registers a user
async fn user_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - user_register_api", "HANDLER");

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_user = UserForCreate {
        ic_number: payload.ic_number,
        password: password_hash,
        name: payload.name,
        email: payload.email,
        phone_number: payload.phone_number,
        blood_type: BloodType::from_str(&payload.blood_type)
            .map_err(|_| Error::InvalidData("blood type".to_string()))?,
        state_id: payload.state_id,
        district_id: payload.district_id,
    };

    UserModelController::create(&app_state.model_manager, registered_user).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

// Request payload for registering a user
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct UserRegisterPayload {
    ic_number: String,
    password: String,
    name: String,
    email: String,
    phone_number: String,
    blood_type: String,
    state_id: i32,
    district_id: i32,
}

// Handler that registers a facility
async fn facility_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<FacilityRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - facility_register_api", "HANDLER");

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_facility = FacilityForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
        address: payload.address,
        phone_number: payload.phone_number,
        state_id: payload.state_id,
    };

    FacilityModelController::create(&app_state.model_manager, registered_facility).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

// Request payload for registering a facility
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct FacilityRegisterPayload {
    email: String,
    password: String,
    name: String,
    address: String,
    phone_number: String,
    state_id: i32,
}

// Handler that registers an organiser
async fn organiser_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<OrganiserRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - organiser_register_api", "HANDLER");

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_organiser = OrganiserForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
        phone_number: payload.phone_number,
    };

    OrganiserModelController::create(&app_state.model_manager, registered_organiser).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

// Request payload for registering an organiser
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct OrganiserRegisterPayload {
    email: String,
    password: String,
    name: String,
    phone_number: String,
}

// Handler that registers an admin
async fn admin_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AdminRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - admin_register_api", "HANDLER");

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_admin = AdminForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
    };

    AdminModelController::create(&app_state.model_manager, registered_admin).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

// Request payload for registering an admin
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct AdminRegisterPayload {
    email: String,
    password: String,
    name: String,
}
