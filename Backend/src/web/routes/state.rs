// Modules
use crate::model::state::StateModelController;
use crate::state::AppState;
use crate::web::Result;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

// Routes
pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/states", get(list_states_handler))
        .with_state(app_state)
}

// Handler that lists all states
async fn list_states_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_states_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let states = StateModelController::list(model_manager).await?;

    let body = Json(json!({
        "data": {
            "states": states,
        }
    }));

    Ok(body)
}