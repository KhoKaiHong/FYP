use crate::context::Context;
use crate::model::donation_history::DonationHistoryModelController;
use crate::state::AppState;
use crate::web::Result;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn list_by_user_id(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/donation-history/user-id",
            get(list_history_by_user_id_handler),
        )
        .with_state(app_state)
}

async fn list_history_by_user_id_handler(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_history_by_user_id_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let history =
        DonationHistoryModelController::list_by_user_id(&context, model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "donationHistory": history,
        }
    }));

    Ok(body)
}
