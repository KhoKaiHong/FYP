// Modules
use crate::context::Context;
use crate::model;
use crate::model::change_event_request::{
    ChangeEventRequestForCreate, ChangeEventRequestForUpdate, ChangeEventRequestModelController,
};
use crate::model::enums::EventRequestStatus;
use crate::model::event::{EventForUpdate, EventModelController};
use crate::model::facility_notification::{
    FacilityNotificationForCreate, FacilityNotificationModelController,
};
use crate::model::organiser_notification::{
    OrganiserNotificationForCreate, OrganiserNotificationModelController,
};
use crate::model::registration::RegistrationModelController;
use crate::model::user_notification::{
    UserNotificationForCreateBulk, UserNotificationModelController,
};
use crate::state::AppState;
use crate::web::{Error, Result};

use axum::extract::State;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use chrono::{prelude::*, DurationRound, TimeDelta};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use tracing::debug;

// Post routes
pub fn post_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request",
            post(post_change_event_request_handler),
        )
        .with_state(app_state)
}

// List by facility routes
pub fn list_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request/facility",
            get(list_change_event_requests_facility_handler),
        )
        .with_state(app_state)
}

// List by organiser routes
pub fn list_by_organiser_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request/organiser",
            get(list_change_event_requests_organiser_handler),
        )
        .with_state(app_state)
}

// Update by facility routes
pub fn update_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request",
            patch(update_change_event_request_facility_handler),
        )
        .with_state(app_state)
}

// Handler that creates a new change event request
async fn post_change_event_request_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ChangeEventRequestCreatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - post_change_event_request_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let start_time = payload
        .start_time
        .parse::<DateTime<Utc>>()
        .map_err(|_| Error::InvalidData("start time".to_string()))?
        .duration_trunc(TimeDelta::microseconds(1))
        .map_err(|_| Error::InvalidData("start time".to_string()))?;

    let end_time = payload
        .end_time
        .parse::<DateTime<Utc>>()
        .map_err(|_| Error::InvalidData("end time".to_string()))?
        .duration_trunc(TimeDelta::microseconds(1))
        .map_err(|_| Error::InvalidData("end time".to_string()))?;

    let event = EventModelController::get(model_manager, payload.event_id)
        .await
        .map_err(|e| match e {
            model::Error::EntityNotFound {
                entity: "event",
                field: model::EntityErrorField::I64Error(id),
            } if id == payload.event_id => Error::InvalidData("event id".to_string()),
            _ => Error::ModelError(e),
        })?;

    let change_event_request = ChangeEventRequestForCreate {
        location: payload.location,
        address: payload.address,
        start_time,
        end_time,
        max_attendees: payload.max_attendees,
        latitude: payload.latitude,
        longitude: payload.longitude,
        change_reason: payload.change_reason,
        event_id: payload.event_id,
        facility_id: event.facility_id,
        organiser_id: context.user_id(),
        state_id: event.state_id,
        district_id: event.district_id,
    };

    // Create the change event request
    ChangeEventRequestModelController::create(model_manager, change_event_request).await?;

    let notification = FacilityNotificationForCreate {
        description: "You have a pending event change request.".to_string(),
        redirect: Some("manage-change-requests".to_string()),
        facility_id: event.facility_id,
    };

    // Notifies the facility of the pending change event request
    FacilityNotificationModelController::create(model_manager, notification).await?;

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

// Request payload for creating a new change event request
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChangeEventRequestCreatePayload {
    pub location: String,
    pub address: String,
    pub start_time: String,
    pub end_time: String,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub change_reason: String,
    pub event_id: i64,
}

// Handler that lists all change event requests for a facility
async fn list_change_event_requests_facility_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!(
        "{:<12} - facility_list_change_event_requests_api",
        "HANDLER"
    );

    let model_manager = &app_state.model_manager;

    let events =
        ChangeEventRequestModelController::list_by_facility(model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "eventRequests": events,
        }
    }));

    Ok(body)
}

// Handler that lists all change event requests for an organiser
async fn list_change_event_requests_organiser_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!(
        "{:<12} - organiser_list_change_event_requests_api",
        "HANDLER"
    );

    let model_manager = &app_state.model_manager;

    let events =
        ChangeEventRequestModelController::list_by_organiser(model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "eventRequests": events,
        }
    }));

    Ok(body)
}

// Handler that updates a change event request by the facility
async fn update_change_event_request_facility_handler(
    _context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ChangeEventRequestUpdatePayload>,
) -> Result<Json<Value>> {
    debug!(
        "{:<12} - facility_update_change_event_request_api",
        "HANDLER"
    );

    let model_manager = &app_state.model_manager;

    let status = EventRequestStatus::from_str(&payload.status)
        .map_err(|_| Error::InvalidData("event request status".to_string()))?;

    let updated_request = ChangeEventRequestForUpdate {
        status: status.clone(),
        rejection_reason: payload.rejection_reason,
    };

    ChangeEventRequestModelController::update(model_manager, payload.id, updated_request).await?;

    let updated_event_details =
        ChangeEventRequestModelController::get(model_manager, payload.id).await?;

    match status {
        // If the status is approved
        EventRequestStatus::Approved => {
            let event_updated = EventForUpdate {
                location: Some(updated_event_details.location),
                address: Some(updated_event_details.address),
                start_time: Some(updated_event_details.start_time),
                end_time: Some(updated_event_details.end_time),
                max_attendees: Some(updated_event_details.max_attendees),
                latitude: Some(updated_event_details.latitude),
                longitude: Some(updated_event_details.longitude),
            };

            // Update the event details
            EventModelController::update(
                model_manager,
                updated_event_details.event_id,
                event_updated,
            )
            .await?;

            // Notify the organiser of the change event request
            let organiser_notification = OrganiserNotificationForCreate {
                description: "Your change event request has been accepted.".to_string(),
                redirect: Some("organiser-change-requests".to_string()),
                organiser_id: updated_event_details.organiser_id,
            };

            OrganiserNotificationModelController::create(model_manager, organiser_notification)
                .await?;

            // Get all users registered to the event
            let attendees_ids: Vec<i64> = RegistrationModelController::list_by_event_id(
                model_manager,
                updated_event_details.event_id,
            )
            .await?
            .into_iter()
            .map(|attendee| attendee.user_id)
            .collect();

            // Send notifications to affected users
            let user_notifications = UserNotificationForCreateBulk {
                description: "There is a change in a blood donation event you are registered in."
                    .to_string(),
                redirect: Some("event-registrations".to_string()),
                user_ids: attendees_ids,
            };

            UserNotificationModelController::create_bulk(model_manager, user_notifications).await?;
        }
        // If the status is rejected
        EventRequestStatus::Rejected => {
            // Notify the organiser of the rejected change event request
            let organiser_notification = OrganiserNotificationForCreate {
                description: "Your change event request has been rejected.".to_string(),
                redirect: Some("organiser-change-requests".to_string()),
                organiser_id: updated_event_details.organiser_id,
            };

            OrganiserNotificationModelController::create(model_manager, organiser_notification)
                .await?;
        }
        _ => {}
    }

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

// Request payload for updating a change event request
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChangeEventRequestUpdatePayload {
    pub id: i64,
    pub status: String,
    pub rejection_reason: Option<String>,
}
