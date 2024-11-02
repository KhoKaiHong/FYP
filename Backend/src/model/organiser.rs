use crate::context::Context;
use crate::model::error::EntityErrorField::{IntError, StringError};
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:    --- Organiser Types

#[derive(Debug, FromRow, Serialize)]
pub struct Organiser {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct OrganiserForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct OrganiserForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

// endregion:    --- Organiser Types

// region:    --- Organiser Model Controller
pub struct OrganiserModelController;

impl OrganiserModelController {
    pub async fn create(
        context: &Context,
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
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<Organiser> {
        let db = model_manager.db();

        let organiser = sqlx::query_as("SELECT * FROM event_organisers WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "organiser",
                field: IntError(id),
            })?;

        Ok(organiser)
    }

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

    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<Organiser>> {
        let db = model_manager.db();

        let organisers = sqlx::query_as("SELECT * FROM event_organisers ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(organisers)
    }

    pub async fn update(
        context: &Context,
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
        query.execute(db).await?;

        Ok(())
    }
}
// endregion: --- Organiser Model Controller

// Backend/src/model/organiser.rs
// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_created = OrganiserForCreate {
            email: "test_create_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser".to_string(),
            phone_number: "1234567890".to_string(),
        };

        // -- Exec
        let id =
            OrganiserModelController::create(&context, &model_manager, organiser_created).await?;

        // -- Check
        let organiser = OrganiserModelController::get(&context, &model_manager, id).await?;
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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let id = 100;

        // -- Exec
        let res = OrganiserModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser",
                    field: IntError(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let organiser_created1 = OrganiserForCreate {
            email: "test_email1@example.com".to_string(),
            password: "welcome1".to_string(),
            name: "Test Organiser 01".to_string(),
            phone_number: "1234567890".to_string(),
        };
        let organiser_created2 = OrganiserForCreate {
            email: "test_email2@example.com".to_string(),
            password: "welcome2".to_string(),
            name: "Test Organiser 02".to_string(),
            phone_number: "987654321".to_string(),
        };

        let id1 =
            OrganiserModelController::create(&context, &model_manager, organiser_created1).await?;
        let id2 =
            OrganiserModelController::create(&context, &model_manager, organiser_created2).await?;
        let organisers = OrganiserModelController::list(&context, &model_manager).await?;

        // Check
        assert_eq!(organisers.len(), 5, "Number of organisers");
        assert_eq!(organisers[3].id, id1);
        assert_eq!(organisers[4].id, id2);
        assert_eq!(organisers[3].email, "test_email1@example.com");
        assert_eq!(organisers[4].email, "test_email2@example.com");
        assert_eq!(organisers[3].password, "welcome1");
        assert_eq!(organisers[4].password, "welcome2");
        assert_eq!(organisers[3].name, "Test Organiser 01");
        assert_eq!(organisers[4].name, "Test Organiser 02");
        assert_eq!(organisers[3].phone_number, "1234567890");
        assert_eq!(organisers[4].phone_number, "987654321");

        for organiser in organisers.iter() {
            println!("organiser: {:?}", organiser);
        }

        // Clean
        sqlx::query("DELETE FROM event_organisers WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM event_organisers WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_created = OrganiserForCreate {
            email: "test_list_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser 01".to_string(),
            phone_number: "1234567890".to_string(),
        };

        let id =
            OrganiserModelController::create(&context, &model_manager, organiser_created).await?;

        let organiser_updated = OrganiserForUpdate {
            email: Some("new_email@gmail.com".to_string()),
            password: None,
            name: Some("New name".to_string()),
            phone_number: None,
        };

        OrganiserModelController::update(&context, &model_manager, id, organiser_updated).await?;

        // -- Check
        let organiser = OrganiserModelController::get(&context, &model_manager, id).await?;
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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let organiser_created = OrganiserForCreate {
            email: "test_create_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Organiser".to_string(),
            phone_number: "1234567890".to_string(),
        };

        let id =
            OrganiserModelController::create(&context, &model_manager, organiser_created).await?;

        // -- Exec
        let organiser = OrganiserModelController::get_by_email(&model_manager, "test_create_ok@example.com").await?;

        // -- Check
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
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;

        // -- Exec
        let res = OrganiserModelController::get_by_email(&model_manager, "test_list_ok@example.com").await;

        // -- Check
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
