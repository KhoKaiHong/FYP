// Modules
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, QueryBuilder, Row};

// User Notification
#[skip_serializing_none]
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

// Defines how to convert a row from the database into a User Notification struct.
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

// Fields used to create a user notification.
#[derive(Deserialize)]
pub struct UserNotificationForCreate {
    pub description: String,
    pub redirect: Option<String>,
    pub user_id: i64,
}

// Fields used to bulk create a user notifications.
#[derive(Deserialize)]
pub struct UserNotificationForCreateBulk {
    pub description: String,
    pub redirect: Option<String>,
    pub user_ids: Vec<i64>,
}

// User Notification Model Controller
pub struct UserNotificationModelController;

impl UserNotificationModelController {
    // Creates a new user notification.
    pub async fn create(
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

    // Creates multiple same user notifications for different users.
    pub async fn create_bulk(
        model_manager: &ModelManager,
        notification_created: UserNotificationForCreateBulk,
    ) -> Result<()> {
        let db = model_manager.db();

        if !notification_created.user_ids.is_empty() {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO user_notifications (description, redirect, user_id) ",
            );

            query_builder.push_values(notification_created.user_ids, |mut b, user_id| {
                b.push_bind(&notification_created.description)
                    .push_bind(&notification_created.redirect)
                    .push_bind(user_id);
            });

            query_builder.build().execute(db).await?;
        }

        Ok(())
    }

    // Gets a user notification by id.
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<UserNotification> {
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

    // Lists all user notifications for a user.
    pub async fn list_by_user_id(
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<UserNotification>> {
        let db = model_manager.db();

        let notifications =
            sqlx::query_as("SELECT * FROM user_notifications WHERE user_id = $1 ORDER BY id")
                .bind(user_id)
                .fetch_all(db)
                .await?;

        Ok(notifications)
    }

    // Marks a user notification as read.
    pub async fn read_notification(model_manager: &ModelManager, id: i64) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE user_notifications SET is_read = true WHERE id = $1")
            .bind(id)
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
        let notification_created = UserNotificationForCreate {
            description: "test_description".to_string(),
            redirect: None,
            user_id: 1000,
        };

        // Execute
        let id =
            UserNotificationModelController::create(&model_manager, notification_created).await?;

        // Check
        let notification = UserNotificationModelController::get(&model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.redirect, None);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.user_id, 1000);
        assert_eq!(notification.is_read, false);

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
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = UserNotificationModelController::get(&model_manager, id).await;

        // Check
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
    async fn test_list_by_user_id() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let notification_created1 = UserNotificationForCreate {
            description: "test_description1".to_string(),
            redirect: None,
            user_id: 1000,
        };
        let notification_created2 = UserNotificationForCreate {
            description: "test_description2".to_string(),
            redirect: Some(String::from("event")),
            user_id: 1001,
        };
        let notification_created3 = UserNotificationForCreate {
            description: "test_description3".to_string(),
            redirect: Some(String::from("event")),
            user_id: 1000,
        };

        let id1 =
            UserNotificationModelController::create(&model_manager, notification_created1).await?;
        let id2 =
            UserNotificationModelController::create(&model_manager, notification_created2).await?;
        let id3 =
            UserNotificationModelController::create(&model_manager, notification_created3).await?;

        // Execute
        let notifications1: Vec<UserNotification> =
            UserNotificationModelController::list_by_user_id(&model_manager, 1000).await?;
        let notifications2: Vec<UserNotification> =
            UserNotificationModelController::list_by_user_id(&model_manager, 1001).await?;

        // Check
        assert_eq!(notifications1.len(), 2);
        assert_eq!(notifications2.len(), 1);

        // Clean
        sqlx::query("DELETE FROM user_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM user_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM user_notifications WHERE id = $1")
            .bind(id3)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_read_notification() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let notification_created = UserNotificationForCreate {
            description: "test_description".to_string(),
            redirect: Some(String::from("event")),
            user_id: 1000,
        };

        // Execute
        let id =
            UserNotificationModelController::create(&model_manager, notification_created).await?;

        UserNotificationModelController::read_notification(&model_manager, id).await?;

        // Check
        let notification = UserNotificationModelController::get(&model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.redirect, Some(String::from("event")));
        assert_eq!(notification.user_id, 1000);
        assert_eq!(notification.is_read, true);

        // Clean
        sqlx::query("DELETE FROM user_notifications WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
