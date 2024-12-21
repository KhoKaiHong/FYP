use std::str::FromStr;

use crate::context::Context;
use crate::model::enums::EventRequestStatus;
use crate::model::event::{EventForCreate, EventModelController};
use crate::model::facility_notification::{
    FacilityNotificationForCreate, FacilityNotificationModelController,
};
use crate::model::new_event_request::{
    NewEventRequestForCreate, NewEventRequestForUpdate, NewEventRequestModelController,
};
use crate::model::organiser_notification::{OrganiserNotificationForCreate, OrganiserNotificationModelController};
use crate::model::user::UserModelController;
use crate::model::user_notification::{UserNotificationForCreateBulk, UserNotificationModelController};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use chrono::{prelude::*, DurationRound, TimeDelta};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn post_route(app_state: AppState) -> Router {
    Router::new()
        .route("/new-event-request", post(post_new_event_request_handler))
        .with_state(app_state)
}

pub fn list_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/new-event-request/facility",
            get(list_new_event_requests_facility_handler),
        )
        .with_state(app_state)
}

pub fn list_by_organiser_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/new-event-request/organiser",
            get(list_new_event_requests_organiser_handler),
        )
        .with_state(app_state)
}

pub fn update_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/new-event-request",
            patch(update_new_event_request_facility_handler),
        )
        .with_state(app_state)
}

async fn post_new_event_request_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<NewEventRequestCreatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - post_new_event_request_api", "HANDLER");

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

    let new_event_request = NewEventRequestForCreate {
        location: payload.location,
        address: payload.address,
        start_time,
        end_time,
        max_attendees: payload.max_attendees,
        latitude: payload.latitude,
        longitude: payload.longitude,
        facility_id: payload.facility_id,
        organiser_id: context.user_id(),
        state_id: payload.state_id,
        district_id: payload.district_id,
    };

    NewEventRequestModelController::create(&context, model_manager, new_event_request).await?;

    let notification = FacilityNotificationForCreate {
        description: "You have a pending new event proposal.".to_string(),
        redirect: Some("manage-event-proposals".to_string()),
        facility_id: payload.facility_id,
    };

    FacilityNotificationModelController::create(model_manager, notification).await?;

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewEventRequestCreatePayload {
    pub location: String,
    pub address: String,
    pub start_time: String,
    pub end_time: String,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub facility_id: i64,
    pub state_id: i32,
    pub district_id: i32,
}

async fn list_new_event_requests_facility_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - facility_list_new_event_requests_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events =
        NewEventRequestModelController::list_by_facility(model_manager, context.user_id()).await?;

    let body = Json(json!({
        "data": {
            "eventRequests": events,
        }
    }));

    Ok(body)
}

async fn list_new_event_requests_organiser_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - organiser_list_new_event_requests_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events =
        NewEventRequestModelController::list_by_organiser(model_manager, context.user_id()).await?;

    let body = Json(json!({
        "data": {
            "eventRequests": events,
        }
    }));

    Ok(body)
}

async fn update_new_event_request_facility_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<NewEventRequestUpdatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - facility_update_new_event_request_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let status = EventRequestStatus::from_str(&payload.status)
        .map_err(|_| Error::InvalidData("event request status".to_string()))?;

    let updated_request = NewEventRequestForUpdate {
        status: status.clone(),
        rejection_reason: payload.rejection_reason,
    };

    NewEventRequestModelController::update(&context, model_manager, payload.id, updated_request)
        .await?;

    let new_event_details =
        NewEventRequestModelController::get(&context, model_manager, payload.id).await?;

    match status {
        EventRequestStatus::Approved => {
            let event_created = EventForCreate {
                location: new_event_details.location,
                address: new_event_details.address,
                start_time: new_event_details.start_time,
                end_time: new_event_details.end_time,
                max_attendees: new_event_details.max_attendees,
                latitude: new_event_details.latitude,
                longitude: new_event_details.longitude,
                facility_id: new_event_details.facility_id,
                organiser_id: new_event_details.organiser_id,
                state_id: new_event_details.state_id,
                district_id: new_event_details.district_id,
            };

            EventModelController::create(model_manager, event_created).await?;

            let organiser_notification = OrganiserNotificationForCreate {
                description: "Your new event proposal has been accepted.".to_string(),
                redirect: Some("organiser-event-proposals".to_string()),
                organiser_id: new_event_details.organiser_id,
            };

            OrganiserNotificationModelController::create(
                &context,
                model_manager,
                organiser_notification,
            )
            .await?;

            let users_ids: Vec<i64> = UserModelController::list_eligible_by_district(
                model_manager,
                new_event_details.district_id,
            )
            .await?
            .into_iter()
            .map(|user| user.id)
            .collect();

            let user_notifications = UserNotificationForCreateBulk {
                description: "There is a new blood donation event near you."
                    .to_string(),
                redirect: Some("events".to_string()),
                user_ids: users_ids,
            };

            UserNotificationModelController::create_bulk(
                model_manager,
                user_notifications,
            )
            .await?;
        }
        EventRequestStatus::Rejected => {
            let organiser_notification = OrganiserNotificationForCreate {
                description: "Your new event proposal has been rejected.".to_string(),
                redirect: Some("organiser-event-proposals".to_string()),
                organiser_id: new_event_details.organiser_id,
            };

            OrganiserNotificationModelController::create(
                &context,
                model_manager,
                organiser_notification,
            )
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

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewEventRequestUpdatePayload {
    pub id: i64,
    pub status: String,
    pub rejection_reason: Option<String>,
}
