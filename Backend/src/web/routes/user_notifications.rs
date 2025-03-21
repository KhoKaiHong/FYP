// Modules
use crate::context::Context;
use crate::model::user_notification::UserNotificationModelController;
use crate::state::AppState;
use crate::web::Result;

use axum::extract::State;
use axum::routing::{get, patch};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

// Routes
pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/user-notifications",
            get(list_user_notifications),
        )
        .route("/user-notification", patch(read_user_notification))
        .with_state(app_state)
}

// Handler that lists all user notifications for the user
async fn list_user_notifications(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_user_notifications_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let notifications =
        UserNotificationModelController::list_by_user_id(model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "userNotifications": notifications,
        }
    }));

    Ok(body)
}

// Handler that marks a user notification as read
async fn read_user_notification(
    _context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ReadUserNotificationPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - read_user_notification_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    UserNotificationModelController::read_notification(model_manager, payload.notification_id)
        .await?;

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

// Request payload for reading a user notification
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct ReadUserNotificationPayload {
    notification_id: i64,
}
