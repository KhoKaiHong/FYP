// Modules
use crate::model::enums::BloodType;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::Serialize;
use serde_with::skip_serializing_none;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// Donation History
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationHistory {
    pub id: i64,
    pub user_id: i64,
    pub user_ic_number: String,
    pub user_name: String,
    pub user_email: String,
    pub user_phone_number: String,
    pub user_blood_type: BloodType,
    pub event_id: Option<i64>,
    pub event_location: Option<String>,
    pub event_address: Option<String>,
    pub event_start_time: Option<DateTime<Utc>>,
    pub event_end_time: Option<DateTime<Utc>>,
    pub event_latitude: Option<f64>,
    pub event_longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
}

// Defines how to convert a row from the database into a Donation History struct.
impl<'r> FromRow<'r, PgRow> for DonationHistory {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(DonationHistory {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            user_ic_number: row.try_get("user_ic_number")?,
            user_name: row.try_get("user_name")?,
            user_email: row.try_get("user_email")?,
            user_phone_number: row.try_get("user_phone_number")?,
            user_blood_type: row.try_get("user_blood_type")?,
            event_id: row.try_get("event_id")?,
            event_location: row.try_get("event_location")?,
            event_address: row.try_get("event_address")?,
            event_start_time: row
                .try_get::<Option<NaiveDateTime>, _>("event_start_time")?
                .map(|dt| dt.and_utc()),
            event_end_time: row
                .try_get::<Option<NaiveDateTime>, _>("event_end_time")?
                .map(|dt| dt.and_utc()),
            event_latitude: row.try_get("event_latitude")?,
            event_longitude: row.try_get("event_longitude")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?.and_utc(),
        })
    }
}

// Fields used to create a Donation History.
pub struct DonationHistoryForCreate {
    pub user_id: i64,
    pub event_id: Option<i64>,
}

// Donation History Model Controller
pub struct DonationHistoryModelController;

impl DonationHistoryModelController {
    // Creates a donation history
    pub async fn create(
        model_manager: &ModelManager,
        donation_history_created: DonationHistoryForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO donation_history (user_id, event_id) values ($1, $2) returning id",
        )
        .bind(donation_history_created.user_id)
        .bind(donation_history_created.event_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    // Gets a donation history by its id
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<DonationHistory> {
        let db = model_manager.db();

        let donation_history: DonationHistory = sqlx::query_as(
            "SELECT donation_history.*, users.ic_number AS user_ic_number, users.name AS user_name, users.email AS user_email, users.phone_number AS user_phone_number, users.blood_type AS user_blood_type, blood_donation_events.location AS event_location, blood_donation_events.address AS event_address, blood_donation_events.start_time AS event_start_time, blood_donation_events.end_time AS event_end_time, blood_donation_events.latitude as event_latitude, blood_donation_events.longitude as event_longitude FROM donation_history JOIN users ON donation_history.user_id = users.id LEFT JOIN blood_donation_events ON donation_history.event_id = blood_donation_events.id WHERE donation_history.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "donation_history",
            field: I64Error(id),
        })?;

        Ok(donation_history)
    }

    // Lists all donation histories for a user
    pub async fn list_by_user_id(
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<DonationHistory>> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user",
                field: I64Error(user_id),
            })?;

        let donation_histories = sqlx::query_as(
            "SELECT donation_history.*, users.ic_number AS user_ic_number, users.name AS user_name, users.email AS user_email, users.phone_number AS user_phone_number, users.blood_type AS user_blood_type, blood_donation_events.location AS event_location, blood_donation_events.address AS event_address, blood_donation_events.start_time AS event_start_time, blood_donation_events.end_time AS event_end_time, blood_donation_events.latitude as event_latitude, blood_donation_events.longitude as event_longitude FROM donation_history JOIN users ON donation_history.user_id = users.id LEFT JOIN blood_donation_events ON donation_history.event_id = blood_donation_events.id WHERE users.id = $1 ORDER BY id",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        Ok(donation_histories)
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
        let donation_history_created = DonationHistoryForCreate {
            user_id: 1000,
            event_id: Some(1),
        };

        // Execute
        let id = DonationHistoryModelController::create(&model_manager, donation_history_created)
            .await?;

        // Check
        let donation_history = DonationHistoryModelController::get(&model_manager, id).await?;

        assert_eq!(donation_history.event_id, Some(1));
        assert_eq!(donation_history.user_id, 1000);

        // Clean
        sqlx::query("DELETE FROM donation_history WHERE id = $1")
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
        let res = DonationHistoryModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "donation_history",
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
    async fn test_list_by_user_id() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let donation_history_created1 = DonationHistoryForCreate {
            user_id: 1000,
            event_id: Some(1),
        };
        let donation_history_created2 = DonationHistoryForCreate {
            user_id: 1000,
            event_id: None,
        };

        // Execute
        let id1 = DonationHistoryModelController::create(&model_manager, donation_history_created1)
            .await?;
        let id2 = DonationHistoryModelController::create(&model_manager, donation_history_created2)
            .await?;

        // Check
        let donation_histories =
            DonationHistoryModelController::list_by_user_id(&model_manager, 1000).await?;

        assert_eq!(
            donation_histories.len(),
            2,
            "Testing list donation history by user id"
        );

        // Clean
        sqlx::query("DELETE FROM donation_history WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM donation_history WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_user_id_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let user_id = 100;

        // Execute
        let res = DonationHistoryModelController::list_by_user_id(&model_manager, user_id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user",
                    field: I64Error(100)
                })
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }
}
