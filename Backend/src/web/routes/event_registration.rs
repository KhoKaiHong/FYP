use crate::context::Context;
use crate::model::registration::{RegistrationModelController, RegistrationForCreate};
use crate::state::AppState;
use crate::web::Result;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/registration/register", post(event_registration_handler))
        .with_state(app_state)
}

async fn event_registration_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<EventRegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - register_events_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let registration = RegistrationForCreate {
        event_id: payload.event_id,
    };

    let registration_id = RegistrationModelController::create(&context, model_manager, registration).await?;

    let body = Json(json!({
        "data": {
            "registrationId": registration_id,
        }
    }));

    Ok(body)
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct EventRegisterPayload {
    event_id: i64,
}