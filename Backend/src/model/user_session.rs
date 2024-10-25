use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- User Session Types
#[derive(Debug, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct UserSessionForCreate {
    pub id: Uuid,
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
    ) -> Result<Uuid> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO user_sessions (id, user_id) values ($1, $2) returning id",
        )
        .bind(user_session_created.id)
        .bind(user_session_created.user_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<UserSession> {
        let db = model_manager.db();

        let user_session = sqlx::query_as(
            "SELECT * FROM user_sessions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::SessionNotFound { session: "user_session", id })?;

        Ok(user_session)
    }

    pub async fn list_by_user_id(
        context: &Context,
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<UserSession>> {
        let db = model_manager.db();

        let user_sessions = sqlx::query_as(
            "SELECT * FROM user_sessions WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        Ok(user_sessions)
    }

    pub async fn delete(context: &Context, model_manager: &ModelManager, id: Uuid) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM user_sessions WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::SessionNotFound { session: "user_session", id });
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
            return Err(Error::EntityNotFound { entity: "user", id: user_id });
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
        let user_session_created = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };

        // -- Exec
        let id = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created,
        )
        .await?;

        // -- Check
        let user_session = UserSessionModelController::get(&context, &model_manager, id).await?;
        assert_eq!(user_session.user_id, 1000);

        // Clean
        UserSessionModelController::delete(&context, &model_manager, id).await?;

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
                Err(Error::SessionNotFound {
                    session: "user_session",
                    id,
                })
            ),
            "SessionNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_user_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let user_session_created1 = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };
        let user_session_created2 = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };

        // -- Exec
        let id1 = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created1,
        )
        .await?;
        let id2 = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created2,
        )
        .await?;
        let user_sessions =
            UserSessionModelController::list_by_user_id(&context, &model_manager, 1000).await?;

        assert_eq!(
            user_sessions.len(),
            2,
            "number of seeded user_sessions."
        );
        assert_eq!(user_sessions[0].user_id, 1000);
        assert_eq!(user_sessions[1].user_id, 1000);

        // Clean
        UserSessionModelController::delete(&context, &model_manager, id1).await?;
        UserSessionModelController::delete(&context, &model_manager, id2).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let user_session_created = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };

        // -- Exec
        let id = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created,
        )
        .await?;
        UserSessionModelController::delete(&context, &model_manager, id).await?;

        // -- Check
        let res = UserSessionModelController::get(&context, &model_manager, id).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "user_session",
                    id
                })
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
        let user_session_created1 = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };
        let user_session_created2 = UserSessionForCreate {
            id: Uuid::new_v4(),
            user_id: 1000,
        };

        // -- Exec
        let id1 = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created1,
        )
        .await?;
        let id2 = UserSessionModelController::create(
            &context,
            &model_manager,
            user_session_created2,
        )
        .await?;
        UserSessionModelController::delete_by_user_id(&context, &model_manager, 1000).await?;

        // -- Check
        let res = UserSessionModelController::get(&context, &model_manager, id1).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "user_session",
                    id: id1
                })
            ),
            "EntityNotFound not matching"
        );
        let res = UserSessionModelController::get(&context, &model_manager, id2).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "user_session",
                    id: id2
                })
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
                    entity: "user",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}