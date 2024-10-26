use crate::auth;
use crate::auth::{
    password::validate_password, token::generate_access_token, token::generate_refresh_token, Role,
};
use crate::context::Context;
use crate::model;
use crate::model::user::UserModelController;
use crate::model::user_session::{UserSessionForCreate, UserSessionModelController};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::body::Body;
use axum::extract::State;
use axum::http::header::{self, HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

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

    validate_password(&payload.password, &user.password)
        .await
        .map_err(|err| match err {
            auth::Error::PasswordNotMatching => Error::LoginFailPasswordNotMatching,
            _ => Error::AuthError(err),
        })?;

    let access_token =
        generate_access_token(user.id, &Role::User).map_err(|err| Error::AuthError(err))?;

    let refresh_token_id = Uuid::new_v4();

    let refresh_token = generate_refresh_token(&refresh_token_id.to_string(), &Role::User)
        .map_err(|err| Error::AuthError(err))?;

    let user_session = UserSessionForCreate {
        id: refresh_token_id,
        user_id: user.id,
    };

    let context = Context::new(user.id, Role::User);

    UserSessionModelController::create(&context, &app_state.model_manager, user_session)
        .await
        .map_err(|err| Error::ModelError(err))?;

    let body = Json(json!({
        "result": {
            "success": true,
            "access_token": access_token,
            "refresh_token": refresh_token,
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
