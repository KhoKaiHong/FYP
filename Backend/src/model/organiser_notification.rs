use crate::context::Context;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// region:    --- Organiser Notification Types

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganiserNotification {
    pub id: i64,
    pub description: String,
    pub redirect: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub organiser_id: i64,
}

impl<'r> FromRow<'r, PgRow> for OrganiserNotification {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(OrganiserNotification {
            id: row.try_get("id")?,
            description: row.try_get("description")?,
            redirect: row.try_get("redirect")?,
            is_read: row.try_get("is_read")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?.and_utc(),
            organiser_id: row.try_get("organiser_id")?,
        })
    }
}

#[derive(Deserialize)]
pub struct OrganiserNotificationForCreate {
    pub description: String,
    pub redirect: Option<String>,
    pub organiser_id: i64,
}

// endregion:    --- Organiser Notification Types

// region:    --- Organiser Notification Model Controller
pub struct OrganiserNotificationModelController;

impl OrganiserNotificationModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        notification_created: OrganiserNotificationForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO organiser_notifications (description, redirect, organiser_id) values ($1, $2, $3) returning id",
        )
        .bind(notification_created.description)
        .bind(notification_created.redirect)
        .bind(notification_created.organiser_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<OrganiserNotification> {
        let db = model_manager.db();

        let notification = sqlx::query_as("SELECT * FROM organiser_notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "organiser_notification",
                field: I64Error(id),
            })?;

        Ok(notification)
    }

    pub async fn list(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<OrganiserNotification>> {
        let db = model_manager.db();

        let notifications = sqlx::query_as("SELECT * FROM organiser_notifications ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(notifications)
    }

    pub async fn list_by_organiser_id(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<OrganiserNotification>> {
        let db = model_manager.db();

        let notifications =
            sqlx::query_as("SELECT * FROM organiser_notifications WHERE organiser_id = $1 ORDER BY id")
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

        let count = sqlx::query("UPDATE organiser_notifications SET is_read = true WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "organiser_notification",
                field: I64Error(id),
            });
        }

        Ok(())
    }
}
// endregion: --- Organiser Notification Model Controller

// Backend/src/model/organiser.rs
// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{_dev_utils, auth::Role};
    use anyhow::Result;
    use serial_test::serial;
    use uuid::Uuid;

    #[tokio::test]
    #[serial]
    async fn test_create() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let notification_created = OrganiserNotificationForCreate {
            description: "test_description".to_string(),
            redirect: None,
            organiser_id: 1,
        };

        // -- Exec
        let id =
            OrganiserNotificationModelController::create(&context, &model_manager, notification_created)
                .await?;

        // -- Check
        let notification =
            OrganiserNotificationModelController::get(&context, &model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.redirect, None);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.organiser_id, 1);
        assert_eq!(notification.is_read, false);

        println!("\n\nnotification: {:?}", notification);

        // Clean
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
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
        let res = OrganiserNotificationModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "organiser_notification",
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

        let notification_created1 = OrganiserNotificationForCreate {
            description: "test_description1".to_string(),
            redirect: None,
            organiser_id: 1,
        };
        let notification_created2 = OrganiserNotificationForCreate {
            description: "test_description2".to_string(),
            redirect: Some(String::from("event")),
            organiser_id: 2,
        };

        let id1 = OrganiserNotificationModelController::create(
            &context,
            &model_manager,
            notification_created1,
        )
        .await?;
        let id2 = OrganiserNotificationModelController::create(
            &context,
            &model_manager,
            notification_created2,
        )
        .await?;
        let notifications: Vec<OrganiserNotification> =
            OrganiserNotificationModelController::list(&context, &model_manager).await?;

        // Check
        assert_eq!(notifications.len(), 2);
        assert_eq!(notifications[0].id, id1);
        assert_eq!(notifications[1].id, id2);
        assert_eq!(notifications[0].description, "test_description1");
        assert_eq!(notifications[1].description, "test_description2");
        assert_eq!(notifications[0].redirect, None);
        assert_eq!(notifications[1].redirect, Some(String::from("event")));
        assert_eq!(notifications[0].organiser_id, 1);
        assert_eq!(notifications[1].organiser_id, 2);
        assert_eq!(notifications[0].is_read, false);
        assert_eq!(notifications[1].is_read, false);

        for notification in notifications.iter() {
            println!("notification: {:?}", notification);
        }

        // Clean
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_organiser_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::new(1, Role::Organiser, Uuid::new_v4());

        let notification_created1 = OrganiserNotificationForCreate {
            description: "test_description1".to_string(),
            redirect: None,
            organiser_id: 1,
        };
        let notification_created2 = OrganiserNotificationForCreate {
            description: "test_description2".to_string(),
            redirect: Some(String::from("event")),
            organiser_id: 2,
        };
        let notification_created3 = OrganiserNotificationForCreate {
            description: "test_description3".to_string(),
            redirect: Some(String::from("event")),
            organiser_id: 1,
        };

        let id1 = OrganiserNotificationModelController::create(
            &context,
            &model_manager,
            notification_created1,
        )
        .await?;
        let id2 = OrganiserNotificationModelController::create(
            &context,
            &model_manager,
            notification_created2,
        )
        .await?;
        let id3 = OrganiserNotificationModelController::create(
            &context,
            &model_manager,
            notification_created3,
        )
        .await?;
        let notifications: Vec<OrganiserNotification> =
            OrganiserNotificationModelController::list_by_organiser_id(&context, &model_manager).await?;

        // Check
        assert_eq!(notifications.len(), 2);
        assert_eq!(notifications[0].id, id1);
        assert_eq!(notifications[1].id, id3);
        assert_eq!(notifications[0].description, "test_description1");
        assert_eq!(notifications[1].description, "test_description3");
        assert_eq!(notifications[0].redirect, None);
        assert_eq!(notifications[1].redirect, Some(String::from("event")));
        assert_eq!(notifications[0].organiser_id, 1);
        assert_eq!(notifications[1].organiser_id, 1);
        assert_eq!(notifications[0].is_read, false);
        assert_eq!(notifications[1].is_read, false);

        for notification in notifications.iter() {
            println!("notification: {:?}", notification);
        }

        // Clean
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id3)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_read_notification() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();

        let notification_created = OrganiserNotificationForCreate {
            description: "test_description".to_string(),
            redirect: Some(String::from("event")),
            organiser_id: 1,
        };

        // -- Exec
        let id =
            OrganiserNotificationModelController::create(&context, &model_manager, notification_created)
                .await?;

        OrganiserNotificationModelController::read_notification(&context, &model_manager, id).await?;

        // -- Check
        let notification = OrganiserNotificationModelController::get(&context, &model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.redirect, Some(String::from("event")));
        assert_eq!(notification.organiser_id, 1);
        assert_eq!(notification.is_read, true);

        println!("\n\nnotification: {:?}", notification);

        // Clean
        sqlx::query("DELETE FROM organiser_notifications WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
