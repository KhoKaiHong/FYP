// Modules
use crate::context::Context;
use crate::model::event::EventModelController;
use crate::state::AppState;
use crate::web::Result;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn list_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/events", get(list_events_handler))
        .route("/events/future", get(list_future_events_handler))
        .with_state(app_state)
}

pub fn list_routes_organiser(app_state: AppState) -> Router {
    Router::new()
        .route("/events/organiser", get(list_events_by_organiser_handler))
        .with_state(app_state)
}

pub fn list_routes_facility(app_state: AppState) -> Router {
    Router::new()
        .route("/events/facility", get(list_events_by_facility_handler))
        .with_state(app_state)
}

async fn list_events_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_events_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events = EventModelController::list(model_manager).await?;

    let body = Json(json!({
        "data": {
            "events": events,
        }
    }));

    Ok(body)
}

async fn list_future_events_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_future_events_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events = EventModelController::list_future_events(model_manager).await?;

    let body = Json(json!({
        "data": {
            "events": events,
        }
    }));

    Ok(body)
}

async fn list_events_by_organiser_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_events_by_organiser_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events = EventModelController::list_by_organiser(model_manager, context.user_id()).await?;

    let body = Json(json!({
        "data": {
            "events": events,
        }
    }));

    Ok(body)
}

async fn list_events_by_facility_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_events_by_facility_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let events = EventModelController::list_by_facility(model_manager, context.user_id()).await?;

    let body = Json(json!({
        "data": {
            "events": events,
        }
    }));

    Ok(body)
}