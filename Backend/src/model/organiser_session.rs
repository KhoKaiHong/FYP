use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Organiser Session Types
#[derive(Debug, FromRow)]
pub struct OrganiserSession {
    pub id: Uuid,
    pub organiser_id: i64,
}

#[derive(Deserialize)]
pub struct OrganiserSessionForCreate {
    pub id: Uuid,
    pub organiser_id: i64,
}
// endregion:    --- Organiser Session Types

// region:    --- Organiser Session Model Controller
pub struct OrganiserSessionModelController;

impl OrganiserSessionModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        organiser_session_created: OrganiserSessionForCreate,
    ) -> Result<Uuid> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO organiser_sessions (id, organiser_id) values ($1, $2) returning id",
        )
        .bind(organiser_session_created.id)
        .bind(organiser_session_created.organiser_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<OrganiserSession> {
        let db = model_manager.db();

        let organiser_session = sqlx::query_as(
            "SELECT * FROM organiser_sessions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::SessionNotFound { session: "organiser_session", id })?;

        Ok(organiser_session)
    }

    pub async fn list_by_organiser_id(
        context: &Context,
        model_manager: &ModelManager,
        organiser_id: i64,
    ) -> Result<Vec<OrganiserSession>> {
        let db = model_manager.db();

        let organiser_sessions = sqlx::query_as(
            "SELECT * FROM organiser_sessions WHERE organiser_id = $1",
        )
        .bind(organiser_id)
        .fetch_all(db)
        .await?;

        Ok(organiser_sessions)
    }

    pub async fn delete(context: &Context, model_manager: &ModelManager, id: Uuid) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM organiser_sessions WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::SessionNotFound { session: "organiser_session", id });
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
            return Err(Error::EntityNotFound { entity: "organiser", id: organiser_id });
        }

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
        let organiser_session_created = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };

        // -- Exec
        let id = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created,
        )
        .await?;

        // -- Check
        let organiser_session = OrganiserSessionModelController::get(&context, &model_manager, id)
            .await?;
        assert_eq!(organiser_session.organiser_id, 1);

        // Clean
        OrganiserSessionModelController::delete(&context, &model_manager, id).await?;

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
                Err(Error::SessionNotFound {
                    session: "organiser_session",
                    id,
                })
            ),
            "SessionNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_organiser_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_session_created1 = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };
        let organiser_session_created2 = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };

        // -- Exec
        let id1 = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created1,
        )
        .await?;
        let id2 = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created2,
        )
        .await?;
        let organiser_sessions =
            OrganiserSessionModelController::list_by_organiser_id(&context, &model_manager, 1)
                .await?;

        assert_eq!(
            organiser_sessions.len(),
            2,
            "number of seeded organiser_sessions."
        );
        assert_eq!(organiser_sessions[0].organiser_id, 1);
        assert_eq!(organiser_sessions[1].organiser_id, 1);

        // Clean
        OrganiserSessionModelController::delete(&context, &model_manager, id1).await?;
        OrganiserSessionModelController::delete(&context, &model_manager, id2).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_session_created = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };

        // -- Exec
        let id = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created,
        )
        .await?;
        OrganiserSessionModelController::delete(&context, &model_manager, id).await?;

        // -- Check
        let res = OrganiserSessionModelController::get(&context, &model_manager, id).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "organiser_session",
                    id
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_organiser_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_session_created1 = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };
        let organiser_session_created2 = OrganiserSessionForCreate {
            id: Uuid::new_v4(),
            organiser_id: 1,
        };

        // -- Exec
        let id1 = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created1,
        )
        .await?;
        let id2 = OrganiserSessionModelController::create(
            &context,
            &model_manager,
            organiser_session_created2,
        )
        .await?;
        OrganiserSessionModelController::delete_by_organiser_id(&context, &model_manager, 1).await?;

        // -- Check
        let res = OrganiserSessionModelController::get(&context, &model_manager, id1).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "organiser_session",
                    id: id1
                })
            ),
            "EntityNotFound not matching"
        );
        let res = OrganiserSessionModelController::get(&context, &model_manager, id2).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "organiser_session",
                    id: id2
                })
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
        let res = OrganiserSessionModelController::delete_by_organiser_id(
            &context,
            &model_manager,
            id,
        )
        .await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}