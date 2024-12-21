// Modules
use crate::model::EntityErrorField::{I64Error, StringError};
use crate::model::{Error, ModelManager, Result};
use serde::Serialize;
use sqlx::postgres::PgDatabaseError;
use sqlx::FromRow;

// Admin
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
}

// Fields used to create an admin.
pub struct AdminForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
}

// Fields used to update an admin
pub struct AdminForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

// Admin Model Controller
pub struct AdminModelController;

impl AdminModelController {
    // Creates an admin
    pub async fn create(
        model_manager: &ModelManager,
        admin_created: AdminForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO admins (email, password, name) values ($1, $2, $3) returning id",
        )
        .bind(admin_created.email)
        .bind(admin_created.password)
        .bind(admin_created.name)
        .fetch_one(db)
        .await
        .map_err(check_duplicate)?;

        Ok(id)
    }

    // Gets admin by id
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<Admin> {
        let db = model_manager.db();

        let admin = sqlx::query_as("SELECT * from admins WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "admin",
                field: I64Error(id),
            })?;

        Ok(admin)
    }

    // Gets admin by email
    pub async fn get_by_email(
        model_manager: &ModelManager,
        email: &str,
    ) -> Result<Admin> {
        let db = model_manager.db();

        let admin = sqlx::query_as("SELECT * from admins WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "admin",
                field: StringError(email.to_string()),
            })?;

        Ok(admin)
    }

    // Updates admin
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        admin_updated: AdminForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE admins SET ");

        let mut first = true;

        if let Some(email) = admin_updated.email {
            query_builder.push("email = ");
            query_builder.push_bind(email);
            first = false;
        }

        if let Some(password) = admin_updated.password {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("password = ");
            query_builder.push_bind(password);
            first = false;
        }

        if let Some(name) = admin_updated.name {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(name);
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
                entity: "admin",
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
                        Some("admins_email_key") => Error::DuplicateKey {
                            table: "admins",
                            column: "email",
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
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // Execute
        let id = AdminModelController::create(&model_manager, admin_created).await?;

        // Check
        let admin = AdminModelController::get(&model_manager, id).await?;
        assert_eq!(admin.email, "admin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "John Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        sqlx::query("DELETE from admins where id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_by_email_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // Execute
        let id = AdminModelController::create(&model_manager, admin_created).await?;
        let admin =
            AdminModelController::get_by_email(&model_manager, "admin@example.com")
                .await?;

        // Check
        assert_eq!(admin.email, "admin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "John Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        sqlx::query("DELETE from admins where id = $1")
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
        let res = AdminModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin",
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
    async fn test_get_by_email_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let email = "admin@example.com".to_string();

        // Execute
        let res = AdminModelController::get_by_email(&model_manager, &email).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin",
                    field: StringError(ref e)
                }) if e == &email
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // Execute
        let id = AdminModelController::create(&model_manager, admin_created).await?;

        let admin_updated = AdminForUpdate {
            email: Some("newadmin@example.com".to_string()),
            password: None,
            name: Some("Jane Doe".to_string()),
        };

        AdminModelController::update(&model_manager, id, admin_updated).await?;

        // Check
        let admin = AdminModelController::get(&model_manager, id).await?;
        assert_eq!(admin.email, "newadmin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "Jane Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        sqlx::query("DELETE from admins where id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
// endregion: --- Tests
