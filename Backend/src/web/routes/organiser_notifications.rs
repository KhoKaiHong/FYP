use crate::context::Context;
use crate::model::organiser_notification::OrganiserNotificationModelController;
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
            "/organiser-notifications",
            get(list_organiser_notifications),
        )
        .route("/organiser-notification", patch(read_organiser_notification))
        .with_state(app_state)
}

async fn list_organiser_notifications(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_organiser_notifications_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let notifications = OrganiserNotificationModelController::list_by_organiser_id(
        model_manager,
        context.user_id(),
    )
    .await?;

    let body = Json(json!({
        "data": {
            "organiserNotifications": notifications,
        }
    }));

    Ok(body)
}

async fn read_organiser_notification(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ReadOrganiserNotificationPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - read_organiser_notification_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    OrganiserNotificationModelController::read_notification(
        &context,
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
struct ReadOrganiserNotificationPayload {
    notification_id: i64,
}
