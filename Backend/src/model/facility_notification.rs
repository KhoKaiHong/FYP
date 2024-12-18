use crate::context::Context;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// region:    --- Facility Notification Types

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FacilityNotification {
    pub id: i64,
    pub description: String,
    pub redirect: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub facility_id: i64,
}

impl<'r> FromRow<'r, PgRow> for FacilityNotification {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(FacilityNotification {
            id: row.try_get("id")?,
            description: row.try_get("description")?,
            redirect: row.try_get("redirect")?,
            is_read: row.try_get("is_read")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?.and_utc(),
            facility_id: row.try_get("facility_id")?,
        })
    }
}

#[derive(Deserialize)]
pub struct FacilityNotificationForCreate {
    pub description: String,
    pub redirect: Option<String>,
    pub facility_id: i64,
}

// endregion:    --- Facility Notification Types

// region:    --- Facility Notification Model Controller
pub struct FacilityNotificationModelController;

impl FacilityNotificationModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        notification_created: FacilityNotificationForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO facility_notifications (description, redirect, facility_id) values ($1, $2, $3) returning id",
        )
        .bind(notification_created.description)
        .bind(notification_created.redirect)
        .bind(notification_created.facility_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<FacilityNotification> {
        let db = model_manager.db();

        let notification = sqlx::query_as("SELECT * FROM facility_notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "facility_notification",
                field: I64Error(id),
            })?;

        Ok(notification)
    }

    pub async fn list(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<FacilityNotification>> {
        let db = model_manager.db();

        let notifications = sqlx::query_as("SELECT * FROM facility_notifications ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(notifications)
    }

    pub async fn list_by_facility_id(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<FacilityNotification>> {
        let db = model_manager.db();

        let notifications =
            sqlx::query_as("SELECT * FROM facility_notifications WHERE facility_id = $1 ORDER BY id")
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

        let count = sqlx::query("UPDATE facility_notifications SET is_read = true WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "facility_notification",
                field: I64Error(id),
            });
        }

        Ok(())
    }
}
// endregion: --- Facility Notification Model Controller

// Backend/src/model/Facility.rs
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
        let notification_created = FacilityNotificationForCreate {
            description: "test_description".to_string(),
            redirect: None,
            facility_id: 1,
        };

        // -- Exec
        let id =
            FacilityNotificationModelController::create(&context, &model_manager, notification_created)
                .await?;

        // -- Check
        let notification =
            FacilityNotificationModelController::get(&context, &model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.redirect, None);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.facility_id, 1);
        assert_eq!(notification.is_read, false);

        println!("\n\nnotification: {:?}", notification);

        // Clean
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
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
        let res = FacilityNotificationModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility_notification",
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

        let notification_created1 = FacilityNotificationForCreate {
            description: "test_description1".to_string(),
            redirect: None,
            facility_id: 1,
        };
        let notification_created2 = FacilityNotificationForCreate {
            description: "test_description2".to_string(),
            redirect: Some(String::from("event")),
            facility_id: 2,
        };

        let id1 = FacilityNotificationModelController::create(
            &context,
            &model_manager,
            notification_created1,
        )
        .await?;
        let id2 = FacilityNotificationModelController::create(
            &context,
            &model_manager,
            notification_created2,
        )
        .await?;
        let notifications: Vec<FacilityNotification> =
            FacilityNotificationModelController::list(&context, &model_manager).await?;

        // Check
        assert_eq!(notifications.len(), 2);
        assert_eq!(notifications[0].id, id1);
        assert_eq!(notifications[1].id, id2);
        assert_eq!(notifications[0].description, "test_description1");
        assert_eq!(notifications[1].description, "test_description2");
        assert_eq!(notifications[0].redirect, None);
        assert_eq!(notifications[1].redirect, Some(String::from("event")));
        assert_eq!(notifications[0].facility_id, 1);
        assert_eq!(notifications[1].facility_id, 2);
        assert_eq!(notifications[0].is_read, false);
        assert_eq!(notifications[1].is_read, false);

        for notification in notifications.iter() {
            println!("notification: {:?}", notification);
        }

        // Clean
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_facility_id() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::new(1, Role::BloodCollectionFacility, Uuid::new_v4());

        let notification_created1 = FacilityNotificationForCreate {
            description: "test_description1".to_string(),
            redirect: None,
            facility_id: 1,
        };
        let notification_created2 = FacilityNotificationForCreate {
            description: "test_description2".to_string(),
            redirect: Some(String::from("event")),
            facility_id: 2,
        };
        let notification_created3 = FacilityNotificationForCreate {
            description: "test_description3".to_string(),
            redirect: Some(String::from("event")),
            facility_id: 1,
        };

        let id1 = FacilityNotificationModelController::create(
            &context,
            &model_manager,
            notification_created1,
        )
        .await?;
        let id2 = FacilityNotificationModelController::create(
            &context,
            &model_manager,
            notification_created2,
        )
        .await?;
        let id3 = FacilityNotificationModelController::create(
            &context,
            &model_manager,
            notification_created3,
        )
        .await?;
        let notifications: Vec<FacilityNotification> =
            FacilityNotificationModelController::list_by_facility_id(&context, &model_manager).await?;

        // Check
        assert_eq!(notifications.len(), 2);
        assert_eq!(notifications[0].id, id1);
        assert_eq!(notifications[1].id, id3);
        assert_eq!(notifications[0].description, "test_description1");
        assert_eq!(notifications[1].description, "test_description3");
        assert_eq!(notifications[0].redirect, None);
        assert_eq!(notifications[1].redirect, Some(String::from("event")));
        assert_eq!(notifications[0].facility_id, 1);
        assert_eq!(notifications[1].facility_id, 1);
        assert_eq!(notifications[0].is_read, false);
        assert_eq!(notifications[1].is_read, false);

        for notification in notifications.iter() {
            println!("notification: {:?}", notification);
        }

        // Clean
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
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

        let notification_created = FacilityNotificationForCreate {
            description: "test_description".to_string(),
            redirect: Some(String::from("event")),
            facility_id: 1,
        };

        // -- Exec
        let id =
            FacilityNotificationModelController::create(&context, &model_manager, notification_created)
                .await?;

        FacilityNotificationModelController::read_notification(&context, &model_manager, id).await?;

        // -- Check
        let notification = FacilityNotificationModelController::get(&context, &model_manager, id).await?;
        assert_eq!(notification.id, id);
        assert_eq!(notification.description, "test_description");
        assert_eq!(notification.redirect, Some(String::from("event")));
        assert_eq!(notification.facility_id, 1);
        assert_eq!(notification.is_read, true);

        println!("\n\nnotification: {:?}", notification);

        // Clean
        sqlx::query("DELETE FROM facility_notifications WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
