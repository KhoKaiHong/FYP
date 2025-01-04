// Modules
use crate::model::enums::{BloodType, EligibilityStatus};
use crate::model::EntityErrorField::{I64Error, StringError};
use crate::model::{Error, ModelManager, Result};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;
use sqlx::FromRow;

// User
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub ic_number: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub blood_type: BloodType,
    pub eligibility: EligibilityStatus,
    pub state_id: i32,
    pub district_id: i32,
    pub state_name: String,
    pub district_name: String,
}

// Fields used to create a user.
#[derive(Deserialize)]
pub struct UserForCreate {
    pub ic_number: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub blood_type: BloodType,
    pub state_id: i32,
    pub district_id: i32,
}

// Fields used to update a user.
#[derive(Deserialize)]
pub struct UserForUpdate {
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub eligibility: Option<EligibilityStatus>,
    pub state_id: Option<i32>,
    pub district_id: Option<i32>,
}

// User Model Controller
pub struct UserModelController;

impl UserModelController {
    // Creates a new user.
    pub async fn create(model_manager: &ModelManager, user_created: UserForCreate) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, state_id, district_id) values ($1, $2, $3, $4, $5, $6, $7, $8) returning id",
        )
        .bind(user_created.ic_number)
        .bind(user_created.password)
        .bind(user_created.name)
        .bind(user_created.email)
        .bind(user_created.phone_number)
        .bind(user_created.blood_type)
        .bind(user_created.state_id)
        .bind(user_created.district_id)
        .fetch_one(db)
        .await
        .map_err(check_duplicate)?;

        Ok(id)
    }

    // Gets a user by id.
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<User> {
        let db = model_manager.db();

        let user = sqlx::query_as("SELECT users.*, states.name AS state_name, districts.name AS district_name FROM users JOIN states ON users.state_id = states.id JOIN districts ON users.district_id = districts.id WHERE users.id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user",
                field: I64Error(id),
            })?;

        Ok(user)
    }

    // Gets a user by IC number.
    pub async fn get_by_ic_number(
        model_manager: &ModelManager,
        ic_number: &str,
    ) -> Result<User> {
        let db = model_manager.db();

        let user = sqlx::query_as("SELECT users.*, states.name AS state_name, districts.name AS district_name FROM users JOIN states ON users.state_id = states.id JOIN districts ON users.district_id = districts.id WHERE users.ic_number = $1")
            .bind(ic_number)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user",
                field: StringError(ic_number.to_string()),
            })?;

        Ok(user)
    }

    // Lists all users.
    pub async fn list(model_manager: &ModelManager) -> Result<Vec<User>> {
        let db = model_manager.db();

        let users = sqlx::query_as("SELECT users.*, states.name AS state_name, districts.name AS district_name FROM users JOIN states ON users.state_id = states.id JOIN districts ON users.district_id = districts.id")
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    // Lists all eligible users for a district.
    pub async fn list_eligible_by_district(
        model_manager: &ModelManager,
        district_id: i32,
    ) -> Result<Vec<User>> {
        let db = model_manager.db();

        let users = sqlx::query_as("SELECT users.*, states.name AS state_name, districts.name AS district_name FROM users JOIN states ON users.state_id = states.id JOIN districts ON users.district_id = districts.id WHERE district_id = $1 AND eligibility = 'Eligible'")
            .bind(district_id)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    // Updates a user.
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        user_updated: UserForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");

        let mut first = true;

        if let Some(password) = user_updated.password {
            query_builder.push("password = ");
            query_builder.push_bind(password);
            first = false;
        }

        if let Some(email) = user_updated.email {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("email = ");
            query_builder.push_bind(email);
            first = false;
        }

        if let Some(phone_number) = user_updated.phone_number {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("phone_number = ");
            query_builder.push_bind(phone_number);
            first = false;
        }

        if let Some(eligibility) = user_updated.eligibility {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("eligibility = ");
            query_builder.push_bind(eligibility);
            first = false;
        }

        if let Some(state_id) = user_updated.state_id {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("state_id = ");
            query_builder.push_bind(state_id);
            first = false;
        }

        if let Some(district_id) = user_updated.district_id {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("district_id = ");
            query_builder.push_bind(district_id);
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
                entity: "user",
                field: I64Error(id),
            });
        };

        Ok(())
    }
}

// Function to check for duplicate constraint errors
fn check_duplicate(err: sqlx::Error) -> Error {
    match err {
        sqlx::Error::Database(ref e) => {
            if let Some(pg_err) = e.try_downcast_ref::<PgDatabaseError>() {
                if pg_err.code() == "23505" {
                    match pg_err.constraint() {
                        Some("users_ic_number_key") => Error::DuplicateKey {
                            table: "users",
                            column: "IC number",
                        },
                        Some("users_email_key") => Error::DuplicateKey {
                            table: "users",
                            column: "email",
                        },
                        Some("users_phone_number_key") => Error::DuplicateKey {
                            table: "users",
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
    async fn test_create_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let user_created = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: BloodType::APositive,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = UserModelController::create(&model_manager, user_created).await?;

        // Check
        let user = UserModelController::get(&model_manager, id).await?;
        assert_eq!(user.ic_number, "1234567890");
        assert_eq!(user.password, "password");
        assert_eq!(user.name, "Test");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.phone_number, "1234567890");
        assert_eq!(user.blood_type, BloodType::APositive);
        assert_eq!(user.eligibility, EligibilityStatus::Eligible);
        assert_eq!(user.state_id, 1);
        assert_eq!(user.district_id, 1);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_create_err() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let user_created = UserForCreate {
            ic_number: "850101-10-5001".to_string(),
            password: "password".to_string(),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: BloodType::APositive,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let res = UserModelController::create(&model_manager, user_created).await;

        // Check
        assert!(
            matches!(res, Err(Error::DuplicateKey {
                table: "users",
                column: "IC number",
            })),
            "Expected DuplicateKey error, got: {:?}",
            res
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // Setup 
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = UserModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user",
                    field: I64Error(100),
                })
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Execute
        let users = UserModelController::list(&model_manager).await?;

        // Check
        assert_eq!(users.len(), 10, "Testing list users");

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let user_created = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: BloodType::APositive,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = UserModelController::create(&model_manager, user_created).await?;

        let user_updated = UserForUpdate {
            password: Some("new_password".to_string()),
            email: None,
            phone_number: Some("9876543210".to_string()),
            eligibility: Some(EligibilityStatus::Ineligible),
            state_id: Some(2),
            district_id: Some(2),
        };

        UserModelController::update(&model_manager, id, user_updated).await?;

        // Check
        let user = UserModelController::get(&model_manager, id).await?;
        assert_eq!(user.password, "new_password");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.phone_number, "9876543210");
        assert_eq!(user.blood_type, BloodType::APositive);
        assert_eq!(user.eligibility, EligibilityStatus::Ineligible);
        assert_eq!(user.state_id, 2);
        assert_eq!(user.district_id, 2);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
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
        let user_created = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: BloodType::APositive,
            state_id: 1,
            district_id: 1,
        };

        let id = UserModelController::create(&model_manager, user_created).await?;

        // Execute
        let user = UserModelController::get_by_ic_number(&model_manager, "1234567890").await?;

        // Check
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password, "password");
        assert_eq!(user.name, "Test");
        assert_eq!(user.phone_number, "1234567890");
        assert_eq!(user.blood_type, BloodType::APositive);
        assert_eq!(user.state_id, 1);
        assert_eq!(user.district_id, 1);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
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
        let res = UserModelController::get_by_ic_number(&model_manager, "hello").await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user",
                    field: StringError(ref e)
                }) if e == "hello"
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }
}
