// Modules
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{Error, ModelManager, Result};

use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// User Session
#[derive(Debug, FromRow)]
pub struct UserSession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub user_id: i64,
}

// Fields used to create a user session.
#[derive(Debug, Deserialize)]
pub struct UserSessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub user_id: i64,
}

// User Session Model Controller
pub struct UserSessionModelController;

impl UserSessionModelController {
    // Creates a new user session.
    pub async fn create(
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

    // Gets a user session by its id.
    pub async fn get(model_manager: &ModelManager, refresh_token_id: Uuid) -> Result<UserSession> {
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

    // Updates a user session.
    pub async fn update(
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

    // Deletes a user session by its id.
    pub async fn delete_by_session(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        user_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();
        let count = sqlx::query("DELETE FROM user_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND user_id = $3")
            .bind(refresh_token_id)
            .bind(access_token_id)
            .bind(user_id)
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

    // Deletes all user sessions for a user.
    pub async fn delete_by_user_id(model_manager: &ModelManager, user_id: i64) -> Result<()> {
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

    // Checks if a user session exists.
    pub async fn check(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        user_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM user_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND user_id = $3 ")
            .bind(refresh_token_id)
            .bind(access_token_id)
            .bind(user_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user_session",
                field: UuidError(refresh_token_id),
            })?;

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create_ok() -> Result<()> {
        // Setup
        let model_manager: ModelManager = _dev_utils::init_test().await;

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let user_session_created = UserSessionForCreate {
            refresh_token_id,
            access_token_id,
            user_id: 1000,
        };

        // Execute
        UserSessionModelController::create(&model_manager, user_session_created).await?;

        // Check
        let user_session =
            UserSessionModelController::get(&model_manager, refresh_token_id).await?;
        println!("\n\nuser_session: {:?}", user_session);
        assert_eq!(user_session.user_id, 1000);

        // Clean
        sqlx::query("DELETE FROM user_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = Uuid::new_v4();

        // Execute
        let res = UserSessionModelController::get(&model_manager, id).await;

        // Check
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
    async fn test_update_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let user_session_created = UserSessionForCreate {
            refresh_token_id,
            access_token_id,
            user_id: 1000,
        };

        // Execute
        UserSessionModelController::create(&model_manager, user_session_created).await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let user_session_updated = UserSessionForCreate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
            user_id: 1001,
        };

        UserSessionModelController::update(&model_manager, user_session_updated, refresh_token_id)
            .await?;

        // Check
        let user_session =
            UserSessionModelController::get(&model_manager, refresh_token_id_updated).await?;
        assert_eq!(user_session.access_token_id, access_token_id_updated);
        assert_eq!(user_session.user_id, 1001);

        // Clean
        sqlx::query("DELETE FROM user_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id_updated)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_user_id_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

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

        // Execute
        UserSessionModelController::create(&model_manager, user_session_created1).await?;
        UserSessionModelController::create(&model_manager, user_session_created2).await?;
        UserSessionModelController::delete_by_user_id(&model_manager, 1000).await?;

        // Check
        let res = UserSessionModelController::get(&model_manager, refresh_token_id1).await;

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

        let res = UserSessionModelController::get(&model_manager, refresh_token_id2).await;

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
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = UserSessionModelController::delete_by_user_id(&model_manager, id).await;

        // Check
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
