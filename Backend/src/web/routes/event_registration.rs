// Modules
use crate::context::Context;
use crate::model::donation_history::{DonationHistoryForCreate, DonationHistoryModelController};
use crate::model::enums::{EligibilityStatus, RegistrationStatus};
use crate::model::registration::{
    RegistrationForCreate, RegistrationForUpdate, RegistrationModelController,
};
use crate::model::user::{UserForUpdate, UserModelController};
use crate::model::user_notification::{UserNotificationForCreate, UserNotificationModelController};
use crate::state::AppState;
use crate::web::{Error, Result};

use axum::extract::State;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use tracing::debug;

// Route to register for an event
pub fn register_route(app_state: AppState) -> Router {
    Router::new()
        .route("/registration/register", post(event_registration_handler))
        .with_state(app_state)
}

// Route to update registration status
pub fn update_route(app_state: AppState) -> Router {
    Router::new()
        .route("/registration", patch(update_registration_status_handler))
        .with_state(app_state)
}

// Route to list registrations by event id
pub fn list_by_event_id_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/registration/event-id",
            post(list_registrations_by_event_id),
        )
        .with_state(app_state)
}

// Route to list registrations by user id
pub fn list_by_user_id_route(app_state: AppState) -> Router {
    Router::new()
        .route("/registration/user-id", get(list_registrations_by_user_id))
        .with_state(app_state)
}

// Handler that registers for an event
async fn event_registration_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<EventRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - register_events_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let registration = RegistrationForCreate {
        event_id: payload.event_id,
        user_id: context.user_id(),
    };

    let registration_id = RegistrationModelController::create(model_manager, registration).await?;

    let body = Json(json!({
        "data": {
            "registrationId": registration_id,
        }
    }));

    Ok(body)
}

// Request payload for registering for an event
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct EventRegisterPayload {
    event_id: i64,
}

// Handler that lists all registrations for an event
async fn list_registrations_by_event_id(
    _context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ListRegistrationsByEventIdPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_registrations_by_event_id_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let registrations =
        RegistrationModelController::list_by_event_id(model_manager, payload.event_id).await?;

    let body = Json(json!({
        "data": {
            "registrations": registrations,
        }
    }));

    Ok(body)
}

// Request payload for listing event registrations by event id
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct ListRegistrationsByEventIdPayload {
    event_id: i64,
}

// Handler that updates a registration status
async fn update_registration_status_handler(
    _context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<UpdateRegistrationStatusPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - update_registration_status_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let status = RegistrationStatus::from_str(&payload.status)
        .map_err(|_| Error::InvalidData("registration status".to_string()))?;

    let registration_updated = RegistrationForUpdate {
        status: Some(status.clone()),
    };

    match status {
        // If registration status marked as absent
        RegistrationStatus::Absent => {
            // Update registration status
            RegistrationModelController::update(
                model_manager,
                payload.registration_id,
                registration_updated,
            )
            .await?;

            let registration =
                RegistrationModelController::get(model_manager, payload.registration_id).await?;

            // Notify the user
            let user_notification = UserNotificationForCreate {
                description: "You have been marked as absent from a blood donation event you are registered in."
                    .to_string(),
                redirect: Some("event-registrations".to_string()),
                user_id: registration.user_id,
            };

            UserNotificationModelController::create(model_manager, user_notification).await?;
        }
        // If the registration statius is marked as attended
        RegistrationStatus::Attended => {
            // Update the registration status
            RegistrationModelController::update(
                model_manager,
                payload.registration_id,
                registration_updated,
            )
            .await?;

            let registration =
                RegistrationModelController::get(model_manager, payload.registration_id).await?;

            // Add donation history
            let donation_history = DonationHistoryForCreate {
                user_id: registration.user_id,
                event_id: Some(registration.event_id),
            };

            DonationHistoryModelController::create(model_manager, donation_history).await?;

            // Update the user eligibility to be ineligible
            let updated_user = UserForUpdate {
                password: None,
                email: None,
                phone_number: None,
                eligibility: Some(EligibilityStatus::Ineligible),
                state_id: None,
                district_id: None,
            };

            UserModelController::update(model_manager, registration.user_id, updated_user).await?;

            // Notify the user
            let user_notification = UserNotificationForCreate {
                description: "You have been marked as present from a blood donation event you are registered in."
                    .to_string(),
                redirect: Some("event-registrations".to_string()),
                user_id: registration.user_id,
            };

            UserNotificationModelController::create(model_manager, user_notification).await?;
        }
        RegistrationStatus::Registered => {
            return Err(Error::InvalidData("registration status".to_string()));
        }
    }

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

// Request payload for updating a registration status
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct UpdateRegistrationStatusPayload {
    registration_id: i64,
    status: String,
}

// Handler that lists all registrations for a user
async fn list_registrations_by_user_id(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_registrations_by_event_id_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let registrations =
        RegistrationModelController::list_by_user_id(model_manager, context.user_id()).await?;

    let body = Json(json!({
        "data": {
            "registrations": registrations,
        }
    }));

    Ok(body)
}
