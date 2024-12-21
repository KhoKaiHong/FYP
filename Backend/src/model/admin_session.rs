// Modules
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{Error, ModelManager, Result};

use sqlx::FromRow;
use uuid::Uuid;

// Admin Session
#[derive(Debug, FromRow)]
pub struct AdminSession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub admin_id: i64,
}

// Fields used to create an Admin Session.
pub struct AdminSessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub admin_id: i64,
}

// Admin Session Model Controller
pub struct AdminSessionModelController;

impl AdminSessionModelController {
    // Creates an admin session
    pub async fn create(
        model_manager: &ModelManager,
        admin_session_created: AdminSessionForCreate,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query(
            "INSERT INTO admin_sessions (refresh_token_id, access_token_id, admin_id) values ($1, $2, $3)",
        )
        .bind(admin_session_created.refresh_token_id)
        .bind(admin_session_created.access_token_id)
        .bind(admin_session_created.admin_id)
        .execute(db)
        .await?;

        Ok(())
    }

    // Gets an admin session by its id
    pub async fn get(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<AdminSession> {
        let db = model_manager.db();

        let admin_session =
            sqlx::query_as("SELECT * FROM admin_sessions WHERE refresh_token_id = $1")
                .bind(refresh_token_id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound {
                    entity: "admin_session",
                    field: UuidError(refresh_token_id),
                })?;

        Ok(admin_session)
    }

    // Updates an admin session
    pub async fn update(
        model_manager: &ModelManager,
        admin_session_updated: AdminSessionForCreate,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE admin_sessions SET refresh_token_id = $1, access_token_id = $2, admin_id = $3 WHERE refresh_token_id = $4")
            .bind(admin_session_updated.refresh_token_id)
            .bind(admin_session_updated.access_token_id)
            .bind(admin_session_updated.admin_id)
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "admin_session",
                field: UuidError(refresh_token_id),
            });
        }

        Ok(())
    }

    // Deletes an admin session
    pub async fn delete_by_session(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        admin_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();
        let count =
            sqlx::query("DELETE FROM admin_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND admin_id = $3")
                .bind(refresh_token_id)
                .bind(access_token_id)
                .bind(admin_id)
                .execute(db)
                .await?
                .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "admin_session",
                field: UuidError(refresh_token_id),
            });
        }
        Ok(())
    }

    // Deletes all admin session by admin id
    pub async fn delete_by_admin_id(
        model_manager: &ModelManager,
        admin_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM admin_sessions WHERE admin_id = $1")
            .bind(admin_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "admin_session",
                field: I64Error(admin_id),
            });
        }

        Ok(())
    }

    // Checks if a admin session exists
    pub async fn check(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        admin_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM admin_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND admin_id = $3 ")
            .bind(refresh_token_id)
            .bind(access_token_id)
            .bind(admin_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "admin_session",
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
        let model_manager = _dev_utils::init_test().await;

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let admin_session_created = AdminSessionForCreate {
            refresh_token_id,
            access_token_id,
            admin_id: 1,
        };

        // Execute
        AdminSessionModelController::create(&model_manager, admin_session_created)
            .await?;

        // Check
        let admin_session =
            AdminSessionModelController::get(&model_manager, refresh_token_id).await?;
        assert_eq!(admin_session.admin_id, 1);
        assert_eq!(admin_session.access_token_id, access_token_id);

        // Clean
        sqlx::query("DELETE FROM admin_sessions WHERE refresh_token_id = $1")
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
        let res = AdminSessionModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_session",
                    field: UuidError(id)
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

        let admin_session_created = AdminSessionForCreate {
            refresh_token_id,
            access_token_id,
            admin_id: 1,
        };

        // Execute
        AdminSessionModelController::create(&model_manager, admin_session_created)
            .await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let admin_session_updated = AdminSessionForCreate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
            admin_id: 2,
        };

        AdminSessionModelController::update(
            &model_manager,
            admin_session_updated,
            refresh_token_id,
        )
        .await?;

        // Check
        let admin_session =
            AdminSessionModelController::get(&model_manager, refresh_token_id_updated)
                .await?;
        assert_eq!(admin_session.access_token_id, access_token_id_updated);
        assert_eq!(admin_session.admin_id, 2);

        // Clean
        sqlx::query("DELETE FROM admin_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id_updated)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_admin_id_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();
        let admin_session_created1 = AdminSessionForCreate {
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            admin_id: 1,
        };

        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();
        let admin_session_created2 = AdminSessionForCreate {
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            admin_id: 1,
        };

        // Execute
        AdminSessionModelController::create(&model_manager, admin_session_created1)
            .await?;

        AdminSessionModelController::create(&model_manager, admin_session_created2)
            .await?;

        AdminSessionModelController::delete_by_admin_id(&model_manager, 1).await?;

        // Check
        let res =
            AdminSessionModelController::get(&model_manager, refresh_token_id1).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_session",
                    field: UuidError(id)
                }) if id == refresh_token_id1
            ),
            "EntityNotFound not matching"
        );

        let res =
            AdminSessionModelController::get(&model_manager, refresh_token_id2).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_session",
                    field: UuidError(id)
                }) if id == refresh_token_id2
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_admin_id_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res =
            AdminSessionModelController::delete_by_admin_id(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_session",
                    field: I64Error(100)
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
