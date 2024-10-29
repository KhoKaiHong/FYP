use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Facility Session Types
#[derive(Debug, FromRow)]
pub struct FacilitySession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub facility_id: i64,
}

#[derive(Deserialize)]
pub struct FacilitySessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub facility_id: i64,
}
// endregion:    --- Facility Session Types

// region:    --- Facility Session Model Controller
pub struct FacilitySessionModelController;

impl FacilitySessionModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        facility_session_created: FacilitySessionForCreate,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query(
            "INSERT INTO facility_sessions (refresh_token_id, access_token_id, facility_id) values ($1, $2, $3)",
        )
        .bind(facility_session_created.refresh_token_id)
        .bind(facility_session_created.access_token_id)
        .bind(facility_session_created.facility_id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<FacilitySession> {
        let db = model_manager.db();

        let facility_session =
            sqlx::query_as("SELECT * FROM facility_sessions WHERE refresh_token_id = $1")
                .bind(refresh_token_id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::SessionNotFound {
                    session: "facility_session",
                    id: refresh_token_id,
                })?;

        Ok(facility_session)
    }

    pub async fn list_by_facility_id(
        context: &Context,
        model_manager: &ModelManager,
        facility_id: i64,
    ) -> Result<Vec<FacilitySession>> {
        let db = model_manager.db();

        let facility_sessions =
            sqlx::query_as("SELECT * FROM facility_sessions WHERE facility_id = $1")
                .bind(facility_id)
                .fetch_all(db)
                .await?;

        Ok(facility_sessions)
    }

    pub async fn delete(
        context: &Context,
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM facility_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::SessionNotFound {
                session: "facility_session",
                id: refresh_token_id,
            });
        }

        Ok(())
    }

    pub async fn delete_by_facility_id(
        context: &Context,
        model_manager: &ModelManager,
        facility_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE FROM facility_sessions WHERE facility_id = $1")
            .bind(facility_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "facility",
                id: facility_id,
            });
        }

        Ok(())
    }
}
// endregion:    --- Facility Session Model Controller

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

        let facility_session_created = FacilitySessionForCreate {
            refresh_token_id,
            access_token_id,
            facility_id: 1,
        };

        // -- Exec
        FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created,
        )
        .await?;

        // -- Check
        let facility_session =
            FacilitySessionModelController::get(&context, &model_manager, refresh_token_id).await?;
        assert_eq!(facility_session.facility_id, 1);
        assert_eq!(facility_session.access_token_id, access_token_id);

        // Clean
        FacilitySessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

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
        let res = FacilitySessionModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "facility_session",
                    id,
                })
            ),
            "SessionNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_facility_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        
        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();
        let facility_session_created1 = FacilitySessionForCreate { 
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            facility_id: 1,
        };
        
        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();
        let facility_session_created2 = FacilitySessionForCreate { 
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            facility_id: 1,
        };

        // -- Exec
        FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created1,
        )
        .await?;
        FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created2,
        )
        .await?;
        let facility_sessions =
            FacilitySessionModelController::list_by_facility_id(&context, &model_manager, 1)
                .await?;

        assert_eq!(
            facility_sessions.len(),
            2,
            "number of seeded facility_sessions."
        );
        assert_eq!(facility_sessions[0].facility_id, 1);
        assert_eq!(facility_sessions[0].access_token_id, access_token_id1);
        assert_eq!(facility_sessions[1].facility_id, 1);
        assert_eq!(facility_sessions[1].access_token_id, access_token_id2);

        // Clean
        FacilitySessionModelController::delete(&context, &model_manager, refresh_token_id1).await?;
        FacilitySessionModelController::delete(&context, &model_manager, refresh_token_id2).await?;

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
        let facility_session_created = FacilitySessionForCreate { 
            refresh_token_id,
            access_token_id,
            facility_id: 1,
        };

        // -- Exec
        FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created,
        )
        .await?;
        FacilitySessionModelController::delete(&context, &model_manager, refresh_token_id).await?;

        // -- Check
        let res = FacilitySessionModelController::get(&context, &model_manager, refresh_token_id).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "facility_session",
                    id: refresh_token_id
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_facility_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        
        let refresh_token_id1 = Uuid::new_v4();
        let access_token_id1 = Uuid::new_v4();
        let facility_session_created1 = FacilitySessionForCreate { 
            refresh_token_id: refresh_token_id1,
            access_token_id: access_token_id1,
            facility_id: 1,
        }; 
        
        let refresh_token_id2 = Uuid::new_v4();
        let access_token_id2 = Uuid::new_v4();
        let facility_session_created2 = FacilitySessionForCreate { 
            refresh_token_id: refresh_token_id2,
            access_token_id: access_token_id2,
            facility_id: 1,
        };

        // -- Exec
        let id1 = FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created1,
        )
        .await?;
        let id2 = FacilitySessionModelController::create(
            &context,
            &model_manager,
            facility_session_created2,
        )
        .await?;
        FacilitySessionModelController::delete_by_facility_id(&context, &model_manager, 1).await?;

        // -- Check
        let res = FacilitySessionModelController::get(&context, &model_manager, refresh_token_id1).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "facility_session",
                    id: refresh_token_id1
                })
            ),
            "EntityNotFound not matching"
        );
        let res = FacilitySessionModelController::get(&context, &model_manager, refresh_token_id2).await;
        assert!(
            matches!(
                res,
                Err(Error::SessionNotFound {
                    session: "facility_session",
                    id: refresh_token_id2
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_facility_id_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res =
            FacilitySessionModelController::delete_by_facility_id(&context, &model_manager, id)
                .await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
