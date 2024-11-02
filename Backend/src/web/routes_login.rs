use crate::auth::{
    self, password::validate_password, token::generate_access_token, token::generate_refresh_token,
    Role,
};
use crate::context::Context;
use crate::model;
use crate::model::facility::FacilityModelController;
use crate::model::facility_session::{FacilitySessionForCreate, FacilitySessionModelController};
use crate::model::organiser::OrganiserModelController;
use crate::model::organiser_session::{OrganiserSessionForCreate, OrganiserSessionModelController};
use crate::model::user::UserModelController;
use crate::model::user_session::{UserSessionForCreate, UserSessionModelController};
use crate::model::EntityErrorField::StringError;
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/userlogin", post(user_login_handler))
        .route("/api/facilitylogin", post(facility_login_handler))
        .route("/api/organiserlogin", post(organiser_login_handler))
        .with_state(app_state)
}

async fn user_login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - user_login_api", "HANDLER");

    let user = UserModelController::get_by_ic_number(&app_state.model_manager, &payload.ic_number)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "user",
                field: StringError(ref ic_number),
            } if ic_number == &payload.ic_number => Error::LoginFailUsernameNotFound,
            _ => Error::ModelError(err),
        })?;

    validate_password(&payload.password, &user.password)
        .await
        .map_err(|err| match err {
            auth::Error::PasswordNotMatching => Error::LoginFailPasswordNotMatching,
            _ => Error::AuthError(err),
        })?;

    let access_token_id = Uuid::new_v4();
    let access_token = generate_access_token(&access_token_id.to_string(), user.id, &Role::User)?;

    let refresh_token_id = Uuid::new_v4();
    let refresh_token = generate_refresh_token(&refresh_token_id.to_string(), &Role::User)?;

    let user_session = UserSessionForCreate {
        refresh_token_id,
        access_token_id,
        user_id: user.id,
    };

    let context = Context::new(user.id, Role::User);

    UserSessionModelController::create(&context, &app_state.model_manager, user_session).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "access_token": access_token,
            "refresh_token": refresh_token,
            "user_details": user,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct UserLoginPayload {
    ic_number: String,
    password: String,
}

async fn facility_login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<FacilityLoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - facility_login_api", "HANDLER");

    let facility = FacilityModelController::get_by_email(&app_state.model_manager, &payload.email)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "facility",
                field: StringError(ref email),
            } if email == &payload.email => Error::LoginFailUsernameNotFound,
            _ => Error::ModelError(err),
        })?;

    validate_password(&payload.password, &facility.password)
        .await
        .map_err(|err| match err {
            auth::Error::PasswordNotMatching => Error::LoginFailPasswordNotMatching,
            _ => Error::AuthError(err),
        })?;

    let access_token_id = Uuid::new_v4();
    let access_token = generate_access_token(
        &access_token_id.to_string(),
        facility.id,
        &Role::BloodCollectionFacility,
    )?;

    let refresh_token_id = Uuid::new_v4();
    let refresh_token = generate_refresh_token(
        &refresh_token_id.to_string(),
        &Role::BloodCollectionFacility,
    )?;

    let facility_session = FacilitySessionForCreate {
        refresh_token_id,
        access_token_id,
        facility_id: facility.id,
    };

    let context = Context::new(facility.id, Role::BloodCollectionFacility);

    FacilitySessionModelController::create(&context, &app_state.model_manager, facility_session)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "access_token": access_token,
            "refresh_token": refresh_token,
            "facility_details": facility,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct FacilityLoginPayload {
    email: String,
    password: String,
}

async fn organiser_login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<OrganiserLoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - organiser_login_api", "HANDLER");

    let organiser =
        OrganiserModelController::get_by_email(&app_state.model_manager, &payload.email)
            .await
            .map_err(|err| match err {
                model::Error::EntityNotFound {
                    entity: "organiser",
                    field: StringError(ref email),
                } if email == &payload.email => Error::LoginFailUsernameNotFound,
                _ => Error::ModelError(err),
            })?;

    validate_password(&payload.password, &organiser.password)
        .await
        .map_err(|err| match err {
            auth::Error::PasswordNotMatching => Error::LoginFailPasswordNotMatching,
            _ => Error::AuthError(err),
        })?;

    let access_token_id = Uuid::new_v4();
    let access_token =
        generate_access_token(&access_token_id.to_string(), organiser.id, &Role::Organiser)?;

    let refresh_token_id = Uuid::new_v4();
    let refresh_token = generate_refresh_token(&refresh_token_id.to_string(), &Role::Organiser)?;

    let organiser_session = OrganiserSessionForCreate {
        refresh_token_id,
        access_token_id,
        organiser_id: organiser.id,
    };

    let context = Context::new(organiser.id, Role::Organiser);

    OrganiserSessionModelController::create(&context, &app_state.model_manager, organiser_session)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "access_token": access_token,
            "refresh_token": refresh_token,
            "organiser_details": organiser,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct OrganiserLoginPayload {
    email: String,
    password: String,
}
