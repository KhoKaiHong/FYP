use std::str::FromStr;

use crate::context::Context;
use crate::model;
use crate::model::change_event_request::{
    ChangeEventRequestForCreate, ChangeEventRequestForUpdate, ChangeEventRequestModelController,
};
use crate::model::enums::EventRequestStatus;
use crate::model::event::{EventForUpdate, EventModelController};
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
        .route(
            "/change-event-request",
            post(post_change_event_request_handler),
        )
        .with_state(app_state)
}

pub fn list_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request/facility",
            get(list_change_event_requests_facility_handler),
        )
        .with_state(app_state)
}

pub fn list_by_organiser_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request/organiser",
            get(list_change_event_requests_organiser_handler),
        )
        .with_state(app_state)
}

pub fn update_by_facility_route(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/change-event-request",
            patch(update_change_event_request_facility_handler),
        )
        .with_state(app_state)
}

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

    let event = EventModelController::get(&context, model_manager, payload.event_id)
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

    ChangeEventRequestModelController::create(&context, model_manager, change_event_request)
        .await?;

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

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

async fn update_change_event_request_facility_handler(
    context: Context,
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

    ChangeEventRequestModelController::update(&context, model_manager, payload.id, updated_request)
        .await?;

    if let EventRequestStatus::Approved = status {
        let updated_event_details =
            ChangeEventRequestModelController::get(&context, model_manager, payload.id).await?;

        let event_updated = EventForUpdate {
            location: Some(updated_event_details.location),
            address: Some(updated_event_details.address),
            start_time: Some(updated_event_details.start_time),
            end_time: Some(updated_event_details.end_time),
            max_attendees: Some(updated_event_details.max_attendees),
            latitude: Some(updated_event_details.latitude),
            longitude: Some(updated_event_details.longitude),
        };

        EventModelController::update(
            &context,
            model_manager,
            updated_event_details.event_id,
            event_updated,
        )
        .await?;
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
pub struct ChangeEventRequestUpdatePayload {
    pub id: i64,
    pub status: String,
    pub rejection_reason: Option<String>,
}
