use crate::context::Context;
use crate::model::new_event_request::{NewEventRequestForCreate, NewEventRequestModelController};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::post;
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
