use crate::context::Context;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// region:    --- User Notification Types

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserNotification {
    pub id: i64,
    pub description: String,
    pub redirect: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub user_id: i64,
}

impl<'r> FromRow<'r, PgRow> for UserNotification {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(UserNotification {
            id: row.try_get("id")?,
            description: row.try_get("description")?,
            redirect: row.try_get("redirect")?,
            is_read: row.try_get("is_read")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?.and_utc(),
            user_id: row.try_get("user_id")?,
        })
    }
}

#[derive(Deserialize)]
pub struct UserNotificationForCreate {
    pub description: String,
    pub redirect: Option<String>,
    pub user_id: i64,
}

// endregion:    --- User Notification Types

// region:    --- User Notification Model Controller
pub struct UserNotificationModelController;

impl UserNotificationModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        notification_created: UserNotificationForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO user_notifications (description, redirect, user_id) values ($1, $2, $3) returning id",
        )
        .bind(notification_created.description)
        .bind(notification_created.redirect)
        .bind(notification_created.user_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<UserNotification> {
        let db = model_manager.db();

        let notification = sqlx::query_as("SELECT * FROM user_notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user_notification",
                field: I64Error(id),
            })?;

        Ok(notification)
    }

    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<UserNotification>> {
        let db = model_manager.db();

        let notifications = sqlx::query_as("SELECT * FROM user_notifications ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(notifications)
    }

    pub async fn list_by_user_id(context: &Context, model_manager: &ModelManager) -> Result<Vec<UserNotification>> {
        let db = model_manager.db();

        let notifications = sqlx::query_as("SELECT * FROM user_notifications WHERE user_id = $1 ORDER BY id")
            .bind(context.user_id())
            .fetch_all(db)
            .await?;

        Ok(notifications)
    }

    pub async fn read_notification(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE user_notifications SET is_read = true WHERE id = $1")
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "user_notification",
                field: I64Error(id),
            });
        }

        Ok(())
    }
}
// endregion: --- User Notification Model Controller

// Backend/src/model/organiser.rs
// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{_dev_utils, model::user};
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let notification_created = UserNotificationForCreate { 
            description: "test_description".to_string(),
            redirect: None,
            user_id: 1,
        };

        // -- Exec
        let id =
            UserNotificationModelController::create(&context, &model_manager, notification_created).await?;

        // -- Check
        let notification = UserNotificationModelController::get(&context, &model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.redirect, None);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.user_id, 1);
        assert_eq!(notification.is_read, false);

        println!("\n\nnotification: {:?}", notification);

        // Clean
        sqlx::query("DELETE FROM user_notifications WHERE id = $1")
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
        let res = UserNotificationModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user_notification",
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
        let organiser =
            OrganiserModelController::get_by_email(&model_manager, "test_create_ok@example.com")
                .await?;

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
        let res =
            OrganiserModelController::get_by_email(&model_manager, "test_list_ok@example.com")
                .await;

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
