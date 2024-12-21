use crate::context::Context;
use crate::model::admin_notification::AdminNotificationModelController;
use crate::state::AppState;
use crate::web::Result;
use axum::extract::State;
use axum::routing::{get, patch};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/admin-notifications", get(list_admin_notifications))
        .route("/admin-notification", patch(read_admin_notification))
        .with_state(app_state)
}

async fn list_admin_notifications(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_admin_notifications_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let notifications =
        AdminNotificationModelController::list_by_admin_id(model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "adminNotifications": notifications,
        }
    }));

    Ok(body)
}

async fn read_admin_notification(
    _context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ReadAdminNotificationPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - read_admin_notification_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    AdminNotificationModelController::read_notification(
        model_manager,
        payload.notification_id,
    )
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
struct ReadAdminNotificationPayload {
    notification_id: i64,
}
