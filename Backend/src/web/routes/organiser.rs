use crate::auth;
use crate::auth::password::{encrypt_password, validate_password};
use crate::context::Context;
use crate::model::organiser::{OrganiserForUpdate, OrganiserModelController};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::patch;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/organiser", patch(organiser_update_handler))
        .with_state(app_state)
}

async fn organiser_update_handler(
    context: Context,
    State(app_state): State<AppState>,
    Json(payload): Json<OrganiserUpdatePayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - update_organiser_api", "HANDLER");

    let model_manager = &app_state.model_manager;

    if let (Some(password), Some(current_password)) = (payload.password, payload.current_password) {
        let organiser =
            OrganiserModelController::get(&context, &app_state.model_manager, context.user_id())
                .await?;

        validate_password(&current_password, &organiser.password)
            .await
            .map_err(|err| match err {
                auth::Error::PasswordNotMatching => Error::CurrentPasswordNotMatching,
                _ => Error::AuthError(err),
            })?;

        let password_hash = encrypt_password(&password).await?;

        // Prepare update details with new password hash
        let updated_details = OrganiserForUpdate {
            password: Some(password_hash),
            name: payload.name,
            email: payload.email,
            phone_number: payload.phone_number,
        };

        // Update user with new details
        OrganiserModelController::update(
            &context,
            model_manager,
            context.user_id(),
            updated_details,
        )
        .await?;
    } else {
        // If no password update, prepare update details without password
        let updated_details = OrganiserForUpdate {
            password: None,
            name: payload.name,
            email: payload.email,
            phone_number: payload.phone_number,
        };

        // Update user with new details
        OrganiserModelController::update(
            &context,
            model_manager,
            context.user_id(),
            updated_details,
        )
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
struct OrganiserUpdatePayload {
    current_password: Option<String>,
    password: Option<String>,
    name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
}
