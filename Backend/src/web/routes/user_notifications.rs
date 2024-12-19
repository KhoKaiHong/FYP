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

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/user-notifications",
            get(list_user_notifications),
        )
        .route("/user-notification", patch(read_user_notification))
        .with_state(app_state)
}

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

async fn read_user_notification(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ReadUserNotificationPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - read_user_notification_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    UserNotificationModelController::read_notification(&context, model_manager, payload.notification_id)
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
struct ReadUserNotificationPayload {
    notification_id: i64,
}
