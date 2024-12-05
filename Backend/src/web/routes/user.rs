use crate::auth;
use crate::auth::password::{encrypt_password, validate_password};
use crate::context::Context;
use crate::model::user::{UserForUpdate, UserModelController};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::patch;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn user_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/user", patch(user_update_handler))
        .with_state(app_state)
}

async fn user_update_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<UserUpdatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - update_user_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    if let (Some(password), Some(current_password)) = (payload.password, payload.current_password) {
        let user =
            UserModelController::get(&context, &app_state.model_manager, context.user_id()).await?;

        validate_password(&current_password, &user.password)
            .await
            .map_err(|err| match err {
                auth::Error::PasswordNotMatching => Error::CurrentPasswordNotMatching,
                _ => Error::AuthError(err),
            })?;

        let password_hash = encrypt_password(&password).await?;

        // Prepare update details with new password hash
        let updated_details = UserForUpdate {
            password: Some(password_hash),
            email: payload.email,
            phone_number: payload.phone_number,
            eligibility: None,
            state_id: payload.state_id,
            district_id: payload.district_id,
        };
        
        // Update user with new details
        UserModelController::update(&context, model_manager, context.user_id(), updated_details)
            .await?;
    } else {
        // If no password update, prepare update details without password
        let updated_details = UserForUpdate {
            password: None,
            email: payload.email,
            phone_number: payload.phone_number,
            eligibility: None,
            state_id: payload.state_id,
            district_id: payload.district_id,
        };
        
        // Update user with new details
        UserModelController::update(&context, model_manager, context.user_id(), updated_details)
            .await?;
    }

    let body = Json(json!({
        "data": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct UserUpdatePayload {
    current_password: Option<String>,
    password: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    state_id: Option<i32>,
    district_id: Option<i32>,
}
