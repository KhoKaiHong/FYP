use crate::context::Context;
use crate::model::facility_notification::FacilityNotificationModelController;
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
        .route("/facility-notifications", get(list_facility_notifications))
        .route(
            "/facility-notification",
            patch(read_facility_notification),
        )
        .with_state(app_state)
}

async fn list_facility_notifications(
    context: Context,
    State(app_state): State<AppState>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_facility_notifications_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    let notifications =
        FacilityNotificationModelController::list_by_facility_id(model_manager, context.user_id())
            .await?;

    let body = Json(json!({
        "data": {
            "facilityNotifications": notifications,
        }
    }));

    Ok(body)
}

async fn read_facility_notification(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<ReadFacilityNotificationPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - read_facility_notification_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    FacilityNotificationModelController::read_notification(
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
struct ReadFacilityNotificationPayload {
    notification_id: i64,
}
