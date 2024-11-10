use crate::context::Context;
use crate::model::EntityErrorField::{I64Error, StringError};
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;
use sqlx::FromRow;

// region:    --- Admin Types

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AdminForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AdminForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

// endregion: --- Admin Types

// region:    --- User Model Controller
pub struct AdminModelController;

impl AdminModelController {
    pub async fn create(
        context: &Context,
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

    pub async fn get(context: &Context, model_manager: &ModelManager, id: i64) -> Result<Admin> {
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

    pub async fn get_by_email(
        context: &Context,
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

    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<Admin>> {
        let db = model_manager.db();

        let admins = sqlx::query_as("SELECT * from admins ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(admins)
    }

    pub async fn update(
        context: &Context,
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

    pub async fn delete(context: &Context, model_manager: &ModelManager, id: i64) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE from admins where id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "admin",
                field: I64Error(id),
            });
        }

        Ok(())
    }
}

// check for duplicate constraints
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
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // -- Exec
        let id = AdminModelController::create(&context, &model_manager, admin_created).await?;

        // -- Check
        let admin = AdminModelController::get(&context, &model_manager, id).await?;
        assert_eq!(admin.email, "admin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "John Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        AdminModelController::delete(&context, &model_manager, id).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_by_email_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // -- Exec
        let id = AdminModelController::create(&context, &model_manager, admin_created).await?;
        let admin =
            AdminModelController::get_by_email(&context, &model_manager, "admin@example.com")
                .await?;

        // -- Check
        assert_eq!(admin.email, "admin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "John Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        AdminModelController::delete(&context, &model_manager, id).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res = AdminModelController::get(&context, &model_manager, id).await;

        // -- Check
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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let email = "admin@example.com".to_string();

        // -- Exec
        let res = AdminModelController::get_by_email(&context, &model_manager, &email).await;

        // -- Check
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
    async fn test_list() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let admin_created1 = AdminForCreate {
            email: "1admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };
        let admin_created2 = AdminForCreate {
            email: "2admin@example.com".to_string(),
            password: "password".to_string(),
            name: "Jane Doe".to_string(),
        };

        // -- Exec
        let id1 = AdminModelController::create(&context, &model_manager, admin_created1).await?;
        let id2 = AdminModelController::create(&context, &model_manager, admin_created2).await?;
        let admins = AdminModelController::list(&context, &model_manager).await?;

        // -- Check
        // Seeded 5 at the first place
        assert_eq!(admins.len(), 5);
        assert_eq!(admins[3].email, "1admin@example.com");
        assert_eq!(admins[4].email, "2admin@example.com");

        for admin in admins {
            println!("admin: {:?}", admin);
        }

        // Clean
        AdminModelController::delete(&context, &model_manager, id1).await?;
        AdminModelController::delete(&context, &model_manager, id2).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let admin_created = AdminForCreate {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
        };

        // -- Exec
        let id = AdminModelController::create(&context, &model_manager, admin_created).await?;

        let admin_updated = AdminForUpdate {
            email: Some("newadmin@example.com".to_string()),
            password: None,
            name: Some("Jane Doe".to_string()),
        };

        AdminModelController::update(&context, &model_manager, id, admin_updated).await?;

        // -- Check
        let admin = AdminModelController::get(&context, &model_manager, id).await?;
        assert_eq!(admin.email, "newadmin@example.com");
        assert_eq!(admin.password, "password");
        assert_eq!(admin.name, "Jane Doe");

        println!("\n\nadmin: {:?}", admin);

        // Clean
        AdminModelController::delete(&context, &model_manager, id).await?;

        Ok(())
    }
}
// endregion: --- Tests
