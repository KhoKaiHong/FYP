// Modules
use crate::model::EntityErrorField::{I64Error, StringError};
use crate::model::{Error, ModelManager, Result};
use serde::Serialize;
use sqlx::postgres::PgDatabaseError;
use sqlx::FromRow;

// Facility
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub state_id: i32,
    pub state_name: String,
}

// Fields used to create a Facility.
pub struct FacilityForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub state_id: i32,
}

// Fields used to update a Facility
pub struct FacilityForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

// Facility Model Controller
pub struct FacilityModelController;

impl FacilityModelController {
    // Creates a facility
    pub async fn create(
        model_manager: &ModelManager,
        facility_created: FacilityForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO blood_collection_facilities (email, password, name, address, phone_number, state_id) values ($1, $2, $3, $4, $5, $6) returning id",
        )
        .bind(facility_created.email)
        .bind(facility_created.password)
        .bind(facility_created.name)
        .bind(facility_created.address)
        .bind(facility_created.phone_number)
        .bind(facility_created.state_id)
        .fetch_one(db)
        .await
        .map_err(check_duplicate)?;

        Ok(id)
    }

    // Gets a facility by its id
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<Facility> {
        let db = model_manager.db();

        let facility = sqlx::query_as(
            "SELECT blood_collection_facilities.*, states.name AS state_name FROM blood_collection_facilities JOIN states ON blood_collection_facilities.state_id = states.id WHERE blood_collection_facilities.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "facility",
            field: I64Error(id),
        })?;

        Ok(facility)
    }

    // Gets a facility by its email
    pub async fn get_by_email(model_manager: &ModelManager, email: &str) -> Result<Facility> {
        let db = model_manager.db();

        let facility = sqlx::query_as(
            "SELECT blood_collection_facilities.*, states.name AS state_name FROM blood_collection_facilities JOIN states ON blood_collection_facilities.state_id = states.id WHERE blood_collection_facilities.email = $1",
        )
        .bind(email)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "facility",
            field: StringError(email.to_string()),
        })?;

        Ok(facility)
    }

    // Lists all facilities
    pub async fn list(model_manager: &ModelManager) -> Result<Vec<Facility>> {
        let db = model_manager.db();

        let facilities = sqlx::query_as("SELECT blood_collection_facilities.*, states.name AS state_name FROM blood_collection_facilities JOIN states ON blood_collection_facilities.state_id = states.id ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(facilities)
    }

    // Updates a facility
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        facility_updated: FacilityForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE blood_collection_facilities SET ");

        let mut first = true;

        if let Some(email) = facility_updated.email {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("email = ");
            query_builder.push_bind(email);
            first = false;
        }

        if let Some(password) = facility_updated.password {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("password = ");
            query_builder.push_bind(password);
            first = false;
        }

        if let Some(name) = facility_updated.name {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(name);
            first = false;
        }

        if let Some(address) = facility_updated.address {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("address = ");
            query_builder.push_bind(address);
            first = false;
        }

        if let Some(phone_number) = facility_updated.phone_number {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("phone_number = ");
            query_builder.push_bind(phone_number);
            first = false;
        }

        // If no fields were updated, return early
        if first {
            return Ok(());
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();
        let count = query
            .execute(db)
            .await
            .map_err(check_duplicate)?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "facility",
                field: I64Error(id),
            });
        };

        Ok(())
    }
}

// Function that checks for duplicate constraint errors
fn check_duplicate(err: sqlx::Error) -> Error {
    match err {
        sqlx::Error::Database(ref e) => {
            if let Some(pg_err) = e.try_downcast_ref::<PgDatabaseError>() {
                if pg_err.code() == "23505" {
                    match pg_err.constraint() {
                        Some("blood_collection_facilities_email_key") => Error::DuplicateKey {
                            table: "blood_collection_facilities",
                            column: "email",
                        },
                        Some("blood_collection_facilities_phone_number_key") => {
                            Error::DuplicateKey {
                                table: "blood_collection_facilities",
                                column: "phone number",
                            }
                        }
                        _ => Error::Sqlx(err),
                    }
                } else {
                    Error::Sqlx(err)
                }
            } else {
                Error::Sqlx(err)
            }
        }
        _ => Error::Sqlx(err),
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
    async fn test_create() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let facility_created = FacilityForCreate {
            email: "test@example.com".to_string(),
            password: "hello".to_string(),
            name: "Test Facility".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };

        // Execute
        let id =
            FacilityModelController::create(&model_manager, facility_created).await?;

        // Check
        let facility = FacilityModelController::get(&model_manager, id).await?;
        assert_eq!(facility.email, "test@example.com");
        assert_eq!(facility.password, "hello");
        assert_eq!(facility.name, "Test Facility");
        assert_eq!(facility.address, "123 Fake St");
        assert_eq!(facility.phone_number, "1234567890");
        assert_eq!(facility.state_id, 1);

        // Clean
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = FacilityModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Check
        let facilities = FacilityModelController::list(&model_manager).await?;
        assert_eq!(facilities.len(), 22, "Testing list facilities");

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let facility_created = FacilityForCreate {
            email: "test1@example.com".to_string(),
            password: "hello".to_string(),
            name: "Test Facility 1".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };

        let id =
            FacilityModelController::create(&model_manager, facility_created).await?;

        let facility_updated = FacilityForUpdate {
            email: Some("new_email@gmail.com".to_string()),
            password: None,
            name: Some("New name".to_string()),
            address: None,
            phone_number: Some("987654321".to_string()),
        };

        FacilityModelController::update(&model_manager, id, facility_updated).await?;

        // Check
        let facility = FacilityModelController::get(&model_manager, id).await?;
        assert_eq!(facility.email, "new_email@gmail.com");
        assert_eq!(facility.password, "hello");
        assert_eq!(facility.name, "New name");
        assert_eq!(facility.address, "123 Fake St");
        assert_eq!(facility.phone_number, "987654321");

        // Clean
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn get_by_email_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let facility_created = FacilityForCreate {
            email: "test@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Facility 01".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };

        let id =
            FacilityModelController::create(&model_manager, facility_created).await?;

        // Execute
        let facility =
            FacilityModelController::get_by_email(&model_manager, "test@example.com").await?;

        // Check
        assert_eq!(facility.password, "welcome");
        assert_eq!(facility.name, "Test Facility 01");
        assert_eq!(facility.address, "123 Fake St");
        assert_eq!(facility.phone_number, "1234567890");
        assert_eq!(facility.state_id, 1);

        // Clean
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn get_by_email_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Execute
        let res =
            FacilityModelController::get_by_email(&model_manager, "test@example.com").await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility",
                    field: StringError(ref e)
                }) if e == "test@example.com"
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }
}
