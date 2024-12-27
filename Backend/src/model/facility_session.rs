// Modules
use crate::model::EntityErrorField::{I64Error, UuidError};
use crate::model::{Error, ModelManager, Result};

use sqlx::FromRow;
use uuid::Uuid;

// Facility Session
#[derive(Debug, FromRow)]
pub struct FacilitySession {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub facility_id: i64,
}

// Fields used to create a Facility Session.
pub struct FacilitySessionForCreate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
    pub facility_id: i64,
}

// Fields used to update a Facility Session.
pub struct FacilitySessionForUpdate {
    pub refresh_token_id: Uuid,
    pub access_token_id: Uuid,
}

// Facility Session Model Controller
pub struct FacilitySessionModelController;

impl FacilitySessionModelController {
    // Create a new facility session
    pub async fn create(
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

    // Get a facility session by its id
    pub async fn get(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
    ) -> Result<FacilitySession> {
        let db = model_manager.db();

        let facility_session =
            sqlx::query_as("SELECT * FROM facility_sessions WHERE refresh_token_id = $1")
                .bind(refresh_token_id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound {
                    entity: "facility_session",
                    field: UuidError(refresh_token_id),
                })?;

        Ok(facility_session)
    }

    // Update a facility session
    pub async fn update(
        model_manager: &ModelManager,
        facility_session_updated: FacilitySessionForUpdate,
        refresh_token_id: Uuid,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE facility_sessions SET refresh_token_id = $1, access_token_id = $2 WHERE refresh_token_id = $3")
            .bind(facility_session_updated.refresh_token_id)
            .bind(facility_session_updated.access_token_id)
            .bind(refresh_token_id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "facility_session",
                field: UuidError(refresh_token_id),
            });
        }

        Ok(())
    }

    // Delete a facility session
    pub async fn delete_by_session(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        facility_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();
        let count = sqlx::query(
            "DELETE FROM facility_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND facility_id = $3",
        )
        .bind(refresh_token_id)
        .bind(access_token_id)
        .bind(facility_id)
        .execute(db)
        .await?
        .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "facility_session",
                field: UuidError(refresh_token_id),
            });
        }
        Ok(())
    }

    // Delete all facility sessions of a facility id
    pub async fn delete_by_facility_id(
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
                entity: "facility_session",
                field: I64Error(facility_id),
            });
        }

        Ok(())
    }

    // Check if a facility session exists
    pub async fn check(
        model_manager: &ModelManager,
        refresh_token_id: Uuid,
        access_token_id: Uuid,
        facility_id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM facility_sessions WHERE refresh_token_id = $1 AND access_token_id = $2 AND facility_id = $3 ")
            .bind(refresh_token_id)
            .bind(access_token_id)
            .bind(facility_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "facility_session",
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

        let facility_session_created = FacilitySessionForCreate {
            refresh_token_id,
            access_token_id,
            facility_id: 1,
        };

        // Execute
        FacilitySessionModelController::create(&model_manager, facility_session_created)
            .await?;

        // Check
        let facility_session =
            FacilitySessionModelController::get(&model_manager, refresh_token_id).await?;
        assert_eq!(facility_session.facility_id, 1);
        assert_eq!(facility_session.access_token_id, access_token_id);

        // Clean
        sqlx::query("DELETE FROM facility_sessions WHERE refresh_token_id = $1")
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
        let res = FacilitySessionModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_session",
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

        let facility_session_created = FacilitySessionForCreate {
            refresh_token_id,
            access_token_id,
            facility_id: 1,
        };

        // Execute
        FacilitySessionModelController::create(&model_manager, facility_session_created)
            .await?;

        let refresh_token_id_updated = Uuid::new_v4();
        let access_token_id_updated = Uuid::new_v4();

        let facility_session_updated = FacilitySessionForUpdate {
            refresh_token_id: refresh_token_id_updated,
            access_token_id: access_token_id_updated,
        };

        FacilitySessionModelController::update(
            &model_manager,
            facility_session_updated,
            refresh_token_id,
        )
        .await?;

        // Check
        let facility_session =
            FacilitySessionModelController::get(&model_manager, refresh_token_id_updated)
                .await?;
        assert_eq!(facility_session.access_token_id, access_token_id_updated);

        // Clean
        sqlx::query("DELETE FROM facility_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id_updated)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_facility_id_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

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

        // Execute
        FacilitySessionModelController::create(
            &model_manager,
            facility_session_created1,
        )
        .await?;
        FacilitySessionModelController::create(
            &model_manager,
            facility_session_created2,
        )
        .await?;
        FacilitySessionModelController::delete_by_facility_id(&model_manager, 1).await?;

        // Check
        let res =
            FacilitySessionModelController::get(&model_manager, refresh_token_id1).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_session",
                    field: UuidError(id),
                }) if id == refresh_token_id1
            ),
            "EntityNotFound not matching"
        );
        let res =
            FacilitySessionModelController::get(&model_manager, refresh_token_id2).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_session",
                    field: UuidError(id),
                }) if id == refresh_token_id2
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_facility_id_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res =
            FacilitySessionModelController::delete_by_facility_id(&model_manager, id)
                .await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_session",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let refresh_token_id = Uuid::new_v4();
        let access_token_id = Uuid::new_v4();

        let facility_session_created = FacilitySessionForCreate {
            refresh_token_id,
            access_token_id,
            facility_id: 1,
        };

        // Execute
        FacilitySessionModelController::create(&model_manager, facility_session_created)
            .await?;

        let res = FacilitySessionModelController::check(&model_manager, refresh_token_id, access_token_id, 1).await;

        // Check
        assert!(
            matches!(
                res,
                Ok(())
            ),
            "Ok not matching"
        );

        // Clean
        sqlx::query("DELETE FROM facility_sessions WHERE refresh_token_id = $1")
            .bind(refresh_token_id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_check_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = Uuid::new_v4();

        // Execute
        let res = FacilitySessionModelController::check(&model_manager, id, id, 1).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_session",
                    field: UuidError(id),
                }) if id == id
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
