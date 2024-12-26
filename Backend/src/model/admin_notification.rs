// Modules
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::Serialize;
use serde_with::skip_serializing_none;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// Admin Notification
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminNotification {
    pub id: i64,
    pub description: String,
    pub redirect: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub admin_id: i64,
}

// Defines how to convert a row from the database into an Admin Notification struct.
impl<'r> FromRow<'r, PgRow> for AdminNotification {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(AdminNotification {
            id: row.try_get("id")?,
            description: row.try_get("description")?,
            redirect: row.try_get("redirect")?,
            is_read: row.try_get("is_read")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?.and_utc(),
            admin_id: row.try_get("admin_id")?,
        })
    }
}

// Fields used to create an Admin Notification.
pub struct AdminNotificationForCreate {
    pub description: String,
    pub redirect: Option<String>,
    pub admin_id: i64,
}

// Admin Notification Model Controller
pub struct AdminNotificationModelController;

impl AdminNotificationModelController {
    // Creates an admin notification.
    pub async fn create(
        model_manager: &ModelManager,
        notification_created: AdminNotificationForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO admin_notifications (description, redirect, admin_id) values ($1, $2, $3) returning id",
        )
        .bind(notification_created.description)
        .bind(notification_created.redirect)
        .bind(notification_created.admin_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    // Gets an admin notification by its id.
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<AdminNotification> {
        let db = model_manager.db();

        let notification = sqlx::query_as("SELECT * FROM admin_notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "admin_notification",
                field: I64Error(id),
            })?;

        Ok(notification)
    }

    // Lists all admin notifications for a specific admin.
    pub async fn list_by_admin_id(
        model_manager: &ModelManager,
        admin_id: i64,
    ) -> Result<Vec<AdminNotification>> {
        let db = model_manager.db();

        let notifications =
            sqlx::query_as("SELECT * FROM admin_notifications WHERE admin_id = $1 ORDER BY id")
                .bind(admin_id)
                .fetch_all(db)
                .await?;

        Ok(notifications)
    }

    // Marks an admin notification as read.
    pub async fn read_notification(model_manager: &ModelManager, id: i64) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("UPDATE admin_notifications SET is_read = true WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "admin_notification",
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
        let notification_created = AdminNotificationForCreate {
            description: "test".to_string(),
            redirect: None,
            admin_id: 1,
        };

        // Execute
        let id =
            AdminNotificationModelController::create(&model_manager, notification_created).await?;

        // Check
        let notification = AdminNotificationModelController::get(&model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.redirect, None);
        assert_eq!(notification.description, "test");
        assert_eq!(notification.admin_id, 1);
        assert_eq!(notification.is_read, false);

        // Clean
        sqlx::query("DELETE FROM admin_notifications WHERE id = $1")
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
        let res = AdminNotificationModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "admin_notification",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_admin_id() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        let notification_created1 = AdminNotificationForCreate {
            description: "test1".to_string(),
            redirect: None,
            admin_id: 1,
        };
        let notification_created2 = AdminNotificationForCreate {
            description: "test2".to_string(),
            redirect: Some(String::from("event")),
            admin_id: 1,
        };
        let notification_created3 = AdminNotificationForCreate {
            description: "test3".to_string(),
            redirect: Some(String::from("event")),
            admin_id: 2,
        };

        let id1 =
            AdminNotificationModelController::create(&model_manager, notification_created1).await?;
        let id2 =
            AdminNotificationModelController::create(&model_manager, notification_created2).await?;
        let id3 =
            AdminNotificationModelController::create(&model_manager, notification_created3).await?;

        // Execute
        let notifications1: Vec<AdminNotification> =
            AdminNotificationModelController::list_by_admin_id(&model_manager, 1).await?;
        let notifications2: Vec<AdminNotification> =
            AdminNotificationModelController::list_by_admin_id(&model_manager, 2).await?;

        // Check
        assert_eq!(notifications1.len(), 2);
        assert_eq!(notifications2.len(), 1);

        // Clean
        sqlx::query("DELETE FROM admin_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM admin_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM admin_notifications WHERE id = $1")
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

        let notification_created = AdminNotificationForCreate {
            description: "test".to_string(),
            redirect: Some(String::from("event")),
            admin_id: 1,
        };

        // Execute
        let id =
            AdminNotificationModelController::create(&model_manager, notification_created).await?;

        AdminNotificationModelController::read_notification(&model_manager, id).await?;

        // Check
        let notification = AdminNotificationModelController::get(&model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.description, "test");
        assert_eq!(notification.redirect, Some(String::from("event")));
        assert_eq!(notification.admin_id, 1);
        assert_eq!(notification.is_read, true);

        // Clean
        sqlx::query("DELETE FROM admin_notifications WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
