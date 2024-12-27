use crate::auth::token::{
    parse_access_token, renew_refresh_token, validate_refresh_token, AccessTokenClaims,
};
use crate::auth::{self, token::generate_access_token, Role};
use crate::context::Context;
use crate::model::admin_session::{
    AdminSessionForCreate, AdminSessionForUpdate, AdminSessionModelController,
};
use crate::model::facility_session::{FacilitySessionForCreate, FacilitySessionForUpdate, FacilitySessionModelController};
use crate::model::organiser_session::{OrganiserSessionForCreate, OrganiserSessionForUpdate, OrganiserSessionModelController};
use crate::model::user_session::{UserSessionForCreate, UserSessionForUpdate, UserSessionModelController};
use crate::model::EntityErrorField::UuidError;
use crate::model::{self, ModelManager};
use crate::state::AppState;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::Authorization;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/refresh", post(refresh_handler))
        .with_state(app_state)
}

async fn refresh_handler(
    context: Result<Context>,
    header: Option<TypedHeader<Authorization<Bearer>>>,
    State(app_state): State<AppState>,
    Json(payload): Json<RefreshRequestPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - refresh_api", "HANDLER");

    if let Err(Error::AccessTokenExpired) = context {
        let model_manager = &app_state.model_manager;
        let context = Context::root_ctx();

        let TypedHeader(header) = header.ok_or(Error::RefreshFailInvalidAccessToken)?;

        let access_token = header.token();

        let access_token_claims = parse_access_token(access_token)?;

        let role = access_token_claims
            .role()
            .map_err(|_| Error::RefreshFailInvalidAccessToken)?;

        let updated_access_token: String;
        let updated_refresh_token: String;

        match role {
            Role::User => {
                (updated_access_token, updated_refresh_token) =
                    refresh_user_token(&context, model_manager, &access_token_claims, &payload)
                        .await?;
            }
            Role::BloodCollectionFacility => {
                (updated_access_token, updated_refresh_token) =
                    refresh_facility_token(&context, model_manager, &access_token_claims, &payload)
                        .await?;
            }
            Role::Organiser => {
                (updated_access_token, updated_refresh_token) = refresh_organiser_token(
                    &context,
                    model_manager,
                    &access_token_claims,
                    &payload,
                )
                .await?;
            }
            Role::Admin => {
                (updated_access_token, updated_refresh_token) =
                    refresh_admin_token(&context, model_manager, &access_token_claims, &payload)
                        .await?;
            }
        }

        let body = Json(json!({
            "data": {
                "accessToken": updated_access_token,
                "refreshToken": updated_refresh_token,
            }
        }));

        Ok(body)
    } else {
        Err(Error::RefreshFailInvalidAccessToken)
    }
}

async fn refresh_user_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<(String, String)> {
    let refresh_token_claims =
        validate_refresh_token(&payload.refresh_token).map_err(|err| match err {
            auth::Error::RefreshTokenExpired => Error::RefreshTokenExpired,
            _ => Error::AuthError(err),
        })?;

    let access_token_jti = Uuid::try_parse(access_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidAccessToken)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidRefreshToken)?;

    let user_session = UserSessionModelController::get(model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::RefreshFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    if user_session.user_id == access_token_claims.id()
        && user_session.access_token_id == access_token_jti
    {
        let new_access_token_id = Uuid::new_v4();
        let new_access_token = generate_access_token(
            &new_access_token_id.to_string(),
            access_token_claims.id(),
            &Role::User,
        )?;

        let new_refresh_token_id = Uuid::new_v4();
        let new_refresh_token = renew_refresh_token(
            &new_refresh_token_id.to_string(),
            refresh_token_claims.exp(),
        )?;

        let updated_user_session = UserSessionForUpdate {
            refresh_token_id: new_refresh_token_id,
            access_token_id: new_access_token_id,
        };

        UserSessionModelController::update(&model_manager, updated_user_session, refresh_token_jti)
            .await?;
        Ok((new_access_token, new_refresh_token))
    } else {
        Err(Error::RefreshFailNoSessionFound)
    }
}

async fn refresh_facility_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<(String, String)> {
    let refresh_token_claims =
        validate_refresh_token(&payload.refresh_token).map_err(|err| match err {
            auth::Error::RefreshTokenExpired => Error::RefreshTokenExpired,
            _ => Error::AuthError(err),
        })?;

    let access_token_jti = Uuid::try_parse(access_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidAccessToken)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidRefreshToken)?;

    let facility_session = FacilitySessionModelController::get(model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "facility_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::RefreshFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    if facility_session.facility_id == access_token_claims.id()
        && facility_session.access_token_id == access_token_jti
    {
        let new_access_token_id = Uuid::new_v4();
        let new_access_token = generate_access_token(
            &new_access_token_id.to_string(),
            access_token_claims.id(),
            &Role::BloodCollectionFacility,
        )?;

        let new_refresh_token_id = Uuid::new_v4();
        let new_refresh_token = renew_refresh_token(
            &new_refresh_token_id.to_string(),
            refresh_token_claims.exp(),
        )?;

        let updated_facility_session = FacilitySessionForUpdate {
            refresh_token_id: new_refresh_token_id,
            access_token_id: new_access_token_id,
        };

        FacilitySessionModelController::update(
            &model_manager,
            updated_facility_session,
            refresh_token_jti,
        )
        .await?;
        Ok((new_access_token, new_refresh_token))
    } else {
        Err(Error::RefreshFailNoSessionFound)
    }
}

async fn refresh_organiser_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<(String, String)> {
    let refresh_token_claims =
        validate_refresh_token(&payload.refresh_token).map_err(|err| match err {
            auth::Error::RefreshTokenExpired => Error::RefreshTokenExpired,
            _ => Error::AuthError(err),
        })?;

    let access_token_jti = Uuid::try_parse(access_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidAccessToken)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidRefreshToken)?;

    let organiser_session = OrganiserSessionModelController::get(model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "organiser_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::RefreshFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    if organiser_session.organiser_id == access_token_claims.id()
        && organiser_session.access_token_id == access_token_jti
    {
        let new_access_token_id = Uuid::new_v4();
        let new_access_token = generate_access_token(
            &new_access_token_id.to_string(),
            access_token_claims.id(),
            &Role::Organiser,
        )?;

        let new_refresh_token_id = Uuid::new_v4();
        let new_refresh_token = renew_refresh_token(
            &new_refresh_token_id.to_string(),
            refresh_token_claims.exp(),
        )?;

        let updated_organiser_session = OrganiserSessionForUpdate {
            refresh_token_id: new_refresh_token_id,
            access_token_id: new_access_token_id,
        };

        OrganiserSessionModelController::update(
            &model_manager,
            updated_organiser_session,
            refresh_token_jti,
        )
        .await?;
        Ok((new_access_token, new_refresh_token))
    } else {
        Err(Error::RefreshFailNoSessionFound)
    }
}

async fn refresh_admin_token(
    context: &Context,
    model_manager: &ModelManager,
    access_token_claims: &AccessTokenClaims,
    payload: &RefreshRequestPayload,
) -> Result<(String, String)> {
    let refresh_token_claims =
        validate_refresh_token(&payload.refresh_token).map_err(|err| match err {
            auth::Error::RefreshTokenExpired => Error::RefreshTokenExpired,
            _ => Error::AuthError(err),
        })?;

    let access_token_jti = Uuid::try_parse(access_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidAccessToken)?;

    let refresh_token_jti = Uuid::try_parse(refresh_token_claims.jti())
        .map_err(|_| Error::RefreshFailInvalidRefreshToken)?;

    let admin_session = AdminSessionModelController::get(model_manager, refresh_token_jti)
        .await
        .map_err(|err| match err {
            model::Error::EntityNotFound {
                entity: "admin_session",
                field: UuidError(refresh_token_jti),
            } if refresh_token_jti == refresh_token_jti => Error::RefreshFailNoSessionFound,
            _ => Error::ModelError(err),
        })?;

    if admin_session.admin_id == access_token_claims.id()
        && admin_session.access_token_id == access_token_jti
    {
        let new_access_token_id = Uuid::new_v4();
        let new_access_token = generate_access_token(
            &new_access_token_id.to_string(),
            access_token_claims.id(),
            &Role::Admin,
        )?;

        let new_refresh_token_id = Uuid::new_v4();
        let new_refresh_token = renew_refresh_token(
            &new_refresh_token_id.to_string(),
            refresh_token_claims.exp(),
        )?;

        let updated_admin_session = AdminSessionForUpdate {
            refresh_token_id: new_refresh_token_id,
            access_token_id: new_access_token_id,
        };

        AdminSessionModelController::update(
            &model_manager,
            updated_admin_session,
            refresh_token_jti,
        )
        .await?;
        Ok((new_access_token, new_refresh_token))
    } else {
        Err(Error::RefreshFailNoSessionFound)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct RefreshRequestPayload {
    refresh_token: String,
}
