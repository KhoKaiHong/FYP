use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use crate::model::EntityErrorField::{I64Error, UuidError};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- User Session Types
#[derive(Debug, FromRow)]
pub struct UserSession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserSessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub user_id: i64,
}
// endregion:    --- User Session Types

// region:    --- User Session Model Controller
pub struct UserSessionModelController;

impl UserSessionModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        user_session_created: UserSessionForCreate,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query(
            "INSERT INTO user_sessions (refresh_token_id, access_token_id, user_id) values ($1, $2, $3)",
        )
        .bind(user_session_created.refresh_token_id)
        .bind(user_session_created.access_token_id)
        .bind(user_session_created.user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<UserSession> {
        let db = model_manager.db();

        let user_session =
            sqlx::query_as("SELECT * FROM user_sessions WHERE refresh_token_id = $1")
                .bind(refresh_token_id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound {
                    entity: "user_session",
                    field: UuidError(refresh_token_id),
                })?;

        Ok(user_session)
    }

    pub async fn list_by_user_id(
        context: &Context,
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<UserSession>> {
        let db = model_manager.db();

        let user_sessions = sqlx::query_as("SELECT * FROM user_sessions WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db)
            .await?;

        Ok(user_sessions)
    }

    pub async fn update(
        context: &Context,
        model_manager: &ModelManager,
        user_session_updated: UserSessionForCreate,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE user_sessions SET refresh_token_id = $1, access_token_id = $2, user_id = $3 WHERE refresh_token_id = $4")
            .bind(user_session_updated.refresh_token_id)
            .bind(user_session_updated.access_token_id)
            .bind(user_session_updated.user_id)
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_id),
            });
        }

        Ok(())
    }

    pub async fn delete(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM user_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_id),
            });
        }

        Ok(())
    }

    pub async fn delete_by_refresh_token_and_user_id(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();
        let count = sqlx::query("DELETE FROM user_sessions WHERE refresh_token_id = $1 AND user_id = $2")
            .bind(refresh_token_id)
            .bind(context.user_id())
            .execute(db)
            .await?
            .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_id),
            });
        }
        Ok(())
    }

    pub async fn delete_by_user_id(
        context: &Context,
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM user_sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "user_session",
                field: I64Error(user_id),
            });
        }

        Ok(())
    }
}
// endregion:    --- User Session Model Controller

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let user_session_created = UserSessionForCreate {
            refresh_token_id,
            access_token_id,
            user_id: 1000,
        };

        // -- Exec
        UserSessionModelController::create(&context, &model_manager, user_session_created).await?;

        // -- Check
        let user_session =
            UserSessionModelController::get(&context, &model_manager, refresh_token_id).await?;
        println!("\n\nuser_session: {:?}", user_session);
        assert_eq!(user_session.user_id, 1000);

        // Clean
        UserSessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = Uuid::new_v4();

        // -- Exec
        let res = UserSessionModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_session",
                    field: UuidError(id),
                }) if id == id
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_user_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();

        let user_session_created1 = UserSessionForCreate {
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            user_id: 1000,
        };

        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();

        let user_session_created2 = UserSessionForCreate {
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            user_id: 1000,
        };

        // -- Exec
        UserSessionModelController::create(&context, &model_manager, user_session_created1).await?;
        UserSessionModelController::create(&context, &model_manager, user_session_created2).await?;
        let user_sessions =
            UserSessionModelController::list_by_user_id(&context, &model_manager, 1000).await?;

        assert_eq!(user_sessions.len(), 2, "number of seeded user_sessions.");
        assert_eq!(user_sessions[0].user_id, 1000);
        assert_eq!(user_sessions[1].user_id, 1000);

        // Clean
        UserSessionModelController::delete(&context, &model_manager, refresh_token_id1).await?;
        UserSessionModelController::delete(&context, &model_manager, refresh_token_id2).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let user_session_created = UserSessionForCreate {
            refresh_token_id,
            access_token_id,
            user_id: 1000,
        };

        // -- Exec
        UserSessionModelController::create(&context, &model_manager, user_session_created).await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let user_session_updated = UserSessionForCreate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
            user_id: 1001,
        };

        UserSessionModelController::update(
            &context,
            &model_manager,
            user_session_updated,
            refresh_token_id,
        )
        .await?;

        let user_session =
            UserSessionModelController::get(&context, &model_manager, refresh_token_id_updated)
                .await?;
        assert_eq!(user_session.access_token_id, access_token_id_updated);
        assert_eq!(user_session.user_id, 1001);

        UserSessionModelController::delete(&context, &model_manager, refresh_token_id_updated)
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let user_session_created = UserSessionForCreate {
            refresh_token_id,
            access_token_id,
            user_id: 1000,
        };

        // -- Exec
        let id = UserSessionModelController::create(&context, &model_manager, user_session_created)
            .await?;
        UserSessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

        // -- Check
        let res = UserSessionModelController::get(&context, &model_manager, refresh_token_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_session",
                    field: UuidError(id),
                }) if id == refresh_token_id
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_user_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();

        let user_session_created1 = UserSessionForCreate {
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            user_id: 1000,
        };

        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();

        let user_session_created2 = UserSessionForCreate {
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            user_id: 1000,
        };

        // -- Exec
        UserSessionModelController::create(&context, &model_manager, user_session_created1).await?;
        UserSessionModelController::create(&context, &model_manager, user_session_created2).await?;
        UserSessionModelController::delete_by_user_id(&context, &model_manager, 1000).await?;

        // -- Check
        let res =
            UserSessionModelController::get(&context, &model_manager, refresh_token_id1).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_session",
                    field: UuidError(id),
                }) if id == refresh_token_id1
            ),
            "EntityNotFound not matching"
        );

        let res =
            UserSessionModelController::get(&context, &model_manager, refresh_token_id2).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_session",
                    field: UuidError(id),
                }) if id == refresh_token_id2
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_user_id_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res = UserSessionModelController::delete_by_user_id(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_session",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
