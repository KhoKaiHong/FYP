use crate::auth;
use crate::auth::{token, Role};
use crate::model::user::UserModelController;
use crate::model::{self, ModelManager};
use crate::model::{facility, facility_session, organiser, organiser_session, user_session};
use crate::state::AppState;
use crate::web::{self, Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/userlogin", post(user_login_handler))
        .with_state(app_state)
}

async fn user_login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login", "HANDLER");

    let user = UserModelController::get_by_ic_number(&app_state.model_manager, payload.ic_number)
        .await
        .map_err(|err| match err {
            model::Error::UserNotFound => Error::LoginFailUsernameNotFound,
            _ => Error::ModelError(err),
        })?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct UserLoginPayload {
    ic_number: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct FacilityLoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct OrganiserLoginPayload {
    email: String,
    password: String,
}
