// Modules
use crate::model::EntityErrorField::{I64Error, StringError};
use crate::model::{Error, ModelManager, Result};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;
use sqlx::FromRow;

// Organiser
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organiser {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub phone_number: String,
}

// Fields used to create an organiser.
#[derive(Deserialize)]
pub struct OrganiserForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: String,
}

// Fields used to update an organiser.
#[derive(Deserialize)]
pub struct OrganiserForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

// Organiser Model Controller
pub struct OrganiserModelController;

impl OrganiserModelController {
    // Creates an organiser.
    pub async fn create(
        model_manager: &ModelManager,
        organiser_created: OrganiserForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO event_organisers (email, password, name, phone_number) values ($1, $2, $3, $4) returning id",
        )
        .bind(organiser_created.email)
        .bind(organiser_created.password)
        .bind(organiser_created.name)
        .bind(organiser_created.phone_number)
        .fetch_one(db)
        .await
        .map_err(check_duplicate)?;

        Ok(id)
    }

    // Gets an organiser by id.
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<Organiser> {
        let db = model_manager.db();

        let organiser = sqlx::query_as("SELECT * FROM event_organisers WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "organiser",
                field: I64Error(id),
            })?;

        Ok(organiser)
    }

    // Gets an organiser by email.
    pub async fn get_by_email(model_manager: &ModelManager, email: &str) -> Result<Organiser> {
        let db = model_manager.db();

        let organiser = sqlx::query_as("SELECT * FROM event_organisers WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "organiser",
                field: StringError(email.to_string()),
            })?;

        Ok(organiser)
    }

    // Updates an organiser.
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        organiser_updated: OrganiserForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE event_organisers SET ");

        let mut first = true;

        if let Some(email) = organiser_updated.email {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("email = ");
            query_builder.push_bind(email);
            first = false;
        }

        if let Some(password) = organiser_updated.password {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("password = ");
            query_builder.push_bind(password);
            first = false;
        }

        if let Some(name) = organiser_updated.name {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(name);
            first = false;
        }

        if let Some(phone_number) = organiser_updated.phone_number {
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
                entity: "organiser",
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
                        Some("event_organisers_email_key") => Error::DuplicateKey {
                            table: "event_organisers",
                            column: "email",
                        },
                        Some("event_organisers_phone_number_key") => Error::DuplicateKey {
                            table: "event_organisers",
                            column: "phone number",
                        },
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
        let organiser_created = OrganiserForCreate {
            email: "test_create_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser".to_string(),
            phone_number: "1234567890".to_string(),
        };

        // Execute
        let id = OrganiserModelController::create(&model_manager, organiser_created).await?;

        // Check
        let organiser = OrganiserModelController::get(&model_manager, id).await?;
        assert_eq!(organiser.email, "test_create_ok@example.com");
        assert_eq!(organiser.password, "welcome");
        assert_eq!(organiser.name, "Test Organiser");
        assert_eq!(organiser.phone_number, "1234567890");

        println!("\n\norganiser: {:?}", organiser);

        // Clean
        sqlx::query("DELETE FROM event_organisers WHERE id = $1")
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
        let res = OrganiserModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser",
                    field: I64Error(100),
                })
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
        let organiser_created = OrganiserForCreate {
            email: "test_list_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser 01".to_string(),
            phone_number: "1234567890".to_string(),
        };

        let id = OrganiserModelController::create(&model_manager, organiser_created).await?;

        let organiser_updated = OrganiserForUpdate {
            email: Some("new_email@gmail.com".to_string()),
            password: None,
            name: Some("New name".to_string()),
            phone_number: None,
        };

        OrganiserModelController::update(&model_manager, id, organiser_updated).await?;

        // Check
        let organiser = OrganiserModelController::get(&model_manager, id).await?;
        assert_eq!(organiser.email, "new_email@gmail.com");
        assert_eq!(organiser.password, "welcome");
        assert_eq!(organiser.name, "New name");
        assert_eq!(organiser.phone_number, "1234567890");

        println!("\n\norganiser: {:?}", organiser);

        // Clean
        sqlx::query("DELETE FROM event_organisers WHERE id = $1")
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
        let organiser_created = OrganiserForCreate {
            email: "test_create_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser".to_string(),
            phone_number: "1234567890".to_string(),
        };

        let id = OrganiserModelController::create(&model_manager, organiser_created).await?;

        // Execute
        let organiser =
            OrganiserModelController::get_by_email(&model_manager, "test_create_ok@example.com")
                .await?;

        // Check
        assert_eq!(organiser.email, "test_create_ok@example.com");
        assert_eq!(organiser.password, "welcome");
        assert_eq!(organiser.name, "Test Organiser");
        assert_eq!(organiser.phone_number, "1234567890");

        println!("\n\norganiser: {:?}", organiser);

        // Clean
        sqlx::query("DELETE FROM event_organisers WHERE id = $1")
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
            OrganiserModelController::get_by_email(&model_manager, "test_list_ok@example.com")
                .await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser",
                    field: StringError(ref e)
                }) if e == "test_list_ok@example.com"
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }
}
