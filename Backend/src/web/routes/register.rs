use std::str::FromStr;
use crate::auth::password::encrypt_password;
use crate::context::Context;
use crate::model::admin::{AdminForCreate, AdminModelController};
use crate::model::facility::{FacilityForCreate, FacilityModelController};
use crate::model::organiser::{OrganiserForCreate, OrganiserModelController};
use crate::model::user::{UserForCreate, UserModelController};
use crate::model::enums::BloodType;
use crate::state::AppState;
use crate::web::{Result, Error};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/userregister", post(user_register_handler))
        .route("/api/facilityregister", post(facility_register_handler))
        .route("/api/organiserregister", post(organiser_register_handler))
        .route("/api/adminregister", post(admin_register_handler))
        .with_state(app_state)
}

async fn user_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - user_register_api", "HANDLER");

    let context = Context::root_ctx();

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_user = UserForCreate {
        ic_number: payload.ic_number,
        password: password_hash,
        name: payload.name,
        email: payload.email,
        phone_number: payload.phone_number,
        blood_type: BloodType::from_str(&payload.blood_type).map_err(|_| Error::InvalidData("blood type".to_string()))?,
        state_id: payload.state_id,
        district_id: payload.district_id,
    };

    UserModelController::create(&context, &app_state.model_manager, registered_user).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

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

async fn facility_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<FacilityRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - facility_register_api", "HANDLER");

    let context = Context::root_ctx();

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_facility = FacilityForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
        address: payload.address,
        phone_number: payload.phone_number,
        state_id: payload.state_id,
    };

    FacilityModelController::create(&context, &app_state.model_manager, registered_facility)
        .await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

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

async fn organiser_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<OrganiserRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - organiser_register_api", "HANDLER");

    let context = Context::root_ctx();

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_organiser = OrganiserForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
        phone_number: payload.phone_number,
    };

    OrganiserModelController::create(&context, &app_state.model_manager, registered_organiser)
        .await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct OrganiserRegisterPayload {
    email: String,
    password: String,
    name: String,
    phone_number: String,
}

async fn admin_register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AdminRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - admin_register_api", "HANDLER");

    let context = Context::root_ctx();

    let password_hash = encrypt_password(&payload.password).await?;

    let registered_admin = AdminForCreate {
        email: payload.email,
        password: password_hash,
        name: payload.name,
    };

    AdminModelController::create(&context, &app_state.model_manager, registered_admin).await?;

    let body = Json(json!({
        "data": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct AdminRegisterPayload {
    email: String,
    password: String,
    name: String,
}
