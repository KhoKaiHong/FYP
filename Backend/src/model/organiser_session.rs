// Modules
use crate::context::Context;
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{Error, ModelManager, Result};

use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// Organiser Session
#[derive(Debug, FromRow)]
pub struct OrganiserSession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub organiser_id: i64,
}

// Fields used to create an Organiser Session.
#[derive(Deserialize)]
pub struct OrganiserSessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub organiser_id: i64,
}

// Organiser Session Model Controller
pub struct OrganiserSessionModelController;

impl OrganiserSessionModelController {
    // Creates an organiser session.
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        organiser_session_created: OrganiserSessionForCreate,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query(
            "INSERT INTO organiser_sessions (refresh_token_id, access_token_id, organiser_id) values ($1, $2, $3)",
        )
        .bind(organiser_session_created.refresh_token_id)
        .bind(organiser_session_created.access_token_id)
        .bind(organiser_session_created.organiser_id)
        .execute(db)
        .await?;

        Ok(())
    }

    // Gets an organiser session by its id.
    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<OrganiserSession> {
        let db = model_manager.db();

        let organiser_session =
            sqlx::query_as("SELECT * FROM organiser_sessions WHERE refresh_token_id = $1")
                .bind(refresh_token_id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound {
                    entity: "organiser_session",
                    field: UuidError(refresh_token_id),
                })?;

        Ok(organiser_session)
    }

    // Updates an organiser session.
    pub async fn update(
        context: &Context,
        model_manager: &ModelManager,
        organiser_session_updated: OrganiserSessionForCreate,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE organiser_sessions SET refresh_token_id = $1, access_token_id = $2, organiser_id = $3 WHERE refresh_token_id = $4")
            .bind(organiser_session_updated.refresh_token_id)
            .bind(organiser_session_updated.access_token_id)
            .bind(organiser_session_updated.organiser_id)
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "organiser_session",
                field: UuidError(refresh_token_id),
            });
        }

        Ok(())
    }

    // Deletes an organiser session.
    pub async fn delete_by_session(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();
        let count = sqlx::query(
            "DELETE FROM organiser_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND organiser_id = $3",
        )
        .bind(refresh_token_id)
        .bind(context.token_id())
        .bind(context.user_id())
        .execute(db)
        .await?
        .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "organiser_session",
                field: UuidError(refresh_token_id),
            });
        }
        Ok(())
    }

    pub async fn delete_by_organiser_id(
        context: &Context,
        model_manager: &ModelManager,
        organiser_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM organiser_sessions WHERE organiser_id = $1")
            .bind(organiser_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "organiser_session",
                field: I64Error(organiser_id),
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

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM organiser_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND organiser_id = $3 ")
            .bind(refresh_token_id)
            .bind(context.token_id())
            .bind(context.user_id())
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "organiser_session",
                field: UuidError(refresh_token_id),
            })?;

        Ok(())
    }
}
// endregion:    --- Organiser Session Model Controller

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
        let organiser_session_created = OrganiserSessionForCreate {
            refresh_token_id,
            access_token_id,
            organiser_id: 1,
        };

        // -- Exec
        OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created,
        )
        .await?;

        // -- Check
        let organiser_session =
            OrganiserSessionModelController::get(&context, &model_manager, refresh_token_id)
                .await?;
        assert_eq!(organiser_session.organiser_id, 1);
        assert_eq!(organiser_session.access_token_id, access_token_id);

        // Clean
        sqlx::query("DELETE FROM organiser_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id)
            .execute(model_manager.db())
            .await?;

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
        let res = OrganiserSessionModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser_session",
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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let organiser_session_created = OrganiserSessionForCreate {
            refresh_token_id,
            access_token_id,
            organiser_id: 1,
        };

        // -- Exec
        OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created,
        )
        .await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let organiser_session_updated = OrganiserSessionForCreate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
            organiser_id: 2,
        };

        OrganiserSessionModelController::update(
            &context,
            &model_manager,
            organiser_session_updated,
            refresh_token_id,
        )
        .await?;

        let organiser_session = OrganiserSessionModelController::get(
            &context,
            &model_manager,
            refresh_token_id_updated,
        )
        .await?;
        assert_eq!(organiser_session.access_token_id, access_token_id_updated);
        assert_eq!(organiser_session.organiser_id, 2);

        sqlx::query("DELETE FROM organiser_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id_updated)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_organiser_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();
        let organiser_session_created1 = OrganiserSessionForCreate {
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            organiser_id: 1,
        };

        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();
        let organiser_session_created2 = OrganiserSessionForCreate {
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            organiser_id: 1,
        };

        // -- Exec
        OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created1,
        )
        .await?;
        OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created2,
        )
        .await?;
        OrganiserSessionModelController::delete_by_organiser_id(&context, &model_manager, 1)
            .await?;

        // -- Check
        let res =
            OrganiserSessionModelController::get(&context, &model_manager, refresh_token_id1).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser_session",
                    field: UuidError(id),
                }) if id == refresh_token_id1
            ),
            "EntityNotFound not matching"
        );

        let res =
            OrganiserSessionModelController::get(&context, &model_manager, refresh_token_id2).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser_session",
                    field: UuidError(id),
                }) if id == refresh_token_id2
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_organiser_id_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res =
            OrganiserSessionModelController::delete_by_organiser_id(&context, &model_manager, id)
                .await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser_session",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
