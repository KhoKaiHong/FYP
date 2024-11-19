use crate::context::Context;
use crate::model::district::DistrictModelController;
use crate::state::AppState;
use crate::web::Result;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/districts", get(list_districts_handler))
        .with_state(app_state)
}

async fn list_districts_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_districts_api", "HANDLER");

    let model_manager = &app_state.model_manager;
    let context =  Context::root_ctx();

    let districts = DistrictModelController::list_with_state(&context, model_manager).await?;

    let body = Json(json!({
        "data": {
            "districts": districts,
        }
    }));

    Ok(body)
}