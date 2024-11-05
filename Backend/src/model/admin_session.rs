use crate::context::Context;
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{Error, ModelManager, Result};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Admin Session Types
#[derive(Debug, FromRow)]
pub struct AdminSession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub admin_id: i64,
}

#[derive(Deserialize)]
pub struct AdminSessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub admin_id: i64,
}
// endregion:    --- Admin Session Types

// region:    --- Admin Session Model Controller
pub struct AdminSessionModelController;

impl AdminSessionModelController {
    pub async fn create(
        context: &Context,
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

    pub async fn get(
        context: &Context,
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

    pub async fn list_by_admin_id(
        context: &Context,
        model_manager: &ModelManager,
        admin_id: i64,
    ) -> Result<Vec<AdminSession>> {
        let db = model_manager.db();

        let admin_sessions = sqlx::query_as("SELECT * FROM admin_sessions WHERE admin_id = $1")
            .bind(admin_id)
            .fetch_all(db)
            .await?;

        Ok(admin_sessions)
    }

    pub async fn update(
        context: &Context,
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

    pub async fn delete(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM admin_sessions WHERE refresh_token_id = $1")
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

    pub async fn delete_by_session(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();
        let count =
            sqlx::query("DELETE FROM admin_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND admin_id = $3")
                .bind(refresh_token_id)
                .bind(context.token_id())
                .bind(context.user_id())
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

    pub async fn delete_by_admin_id(
        context: &Context,
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

    pub async fn check(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM admin_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND user_id = $3 ")
            .bind(refresh_token_id)
            .bind(context.token_id())
            .bind(context.user_id())
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "admin_session",
                field: UuidError(refresh_token_id),
            })?;

        Ok(())
    }
}
// endregion:    --- Admin Session Model Controller

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

        let admin_session_created = AdminSessionForCreate {
            refresh_token_id,
            access_token_id,
            admin_id: 1,
        };

        // -- Exec
        AdminSessionModelController::create(&context, &model_manager, admin_session_created)
            .await?;

        // -- Check
        let admin_session =
            AdminSessionModelController::get(&context, &model_manager, refresh_token_id).await?;
        assert_eq!(admin_session.admin_id, 1);
        assert_eq!(admin_session.access_token_id, access_token_id);

        // Clean
        AdminSessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

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
        let res = AdminSessionModelController::get(&context, &model_manager, id).await;

        // -- Check
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
    async fn test_list_by_admin_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

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

        // -- Exec
        AdminSessionModelController::create(&context, &model_manager, admin_session_created1)
            .await?;
        AdminSessionModelController::create(&context, &model_manager, admin_session_created2)
            .await?;
        let admin_sessions =
            AdminSessionModelController::list_by_admin_id(&context, &model_manager, 1).await?;

        assert_eq!(admin_sessions.len(), 2, "number of seeded admin sessions.");
        assert_eq!(admin_sessions[0].admin_id, 1);
        assert_eq!(admin_sessions[0].access_token_id, access_token_id1);
        assert_eq!(admin_sessions[1].admin_id, 1);
        assert_eq!(admin_sessions[1].access_token_id, access_token_id2);

        // Clean
        AdminSessionModelController::delete(&context, &model_manager, refresh_token_id1).await?;
        AdminSessionModelController::delete(&context, &model_manager, refresh_token_id2).await?;

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

        let admin_session_created = AdminSessionForCreate {
            refresh_token_id,
            access_token_id,
            admin_id: 1,
        };

        // -- Exec
        AdminSessionModelController::create(&context, &model_manager, admin_session_created)
            .await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let admin_session_updated = AdminSessionForCreate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
            admin_id: 2,
        };

        AdminSessionModelController::update(
            &context,
            &model_manager,
            admin_session_updated,
            refresh_token_id,
        )
        .await?;

        let admin_session =
            AdminSessionModelController::get(&context, &model_manager, refresh_token_id_updated)
                .await?;
        assert_eq!(admin_session.access_token_id, access_token_id_updated);
        assert_eq!(admin_session.admin_id, 2);

        AdminSessionModelController::delete(&context, &model_manager, refresh_token_id_updated)
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
        let admin_session_created = AdminSessionForCreate {
            refresh_token_id,
            access_token_id,
            admin_id: 1,
        };

        // -- Exec
        AdminSessionModelController::create(&context, &model_manager, admin_session_created)
            .await?;
        AdminSessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

        // -- Check
        let res =
            AdminSessionModelController::get(&context, &model_manager, refresh_token_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_session",
                    field: UuidError(id)
                }) if id == refresh_token_id
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_admin_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

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

        // -- Exec
        AdminSessionModelController::create(&context, &model_manager, admin_session_created1)
            .await?;

        AdminSessionModelController::create(&context, &model_manager, admin_session_created2)
            .await?;

        AdminSessionModelController::delete_by_admin_id(&context, &model_manager, 1).await?;

        // -- Check
        let res =
            AdminSessionModelController::get(&context, &model_manager, refresh_token_id1).await;

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
            AdminSessionModelController::get(&context, &model_manager, refresh_token_id2).await;

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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res =
            AdminSessionModelController::delete_by_admin_id(&context, &model_manager, id).await;

        // -- Check
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
