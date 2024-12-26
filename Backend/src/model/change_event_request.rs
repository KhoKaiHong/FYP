// Modules
use crate::model::enums::EventRequestStatus;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// Change Event Request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEventRequest {
    pub id: i64,
    pub location: String,
    pub address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub status: EventRequestStatus,
    pub change_reason: String,
    pub rejection_reason: Option<String>,
    pub event_id: i64,
    pub facility_id: i64,
    pub facility_email: String,
    pub facility_name: String,
    pub facility_address: String,
    pub facility_phone_number: String,
    pub organiser_id: i64,
    pub organiser_email: String,
    pub organiser_name: String,
    pub organiser_phone_number: String,
    pub state_id: i32,
    pub state_name: String,
    pub district_id: i32,
    pub district_name: String,
}

// Defines how to convert a row from the database into a Change Event Request struct.
impl<'r> FromRow<'r, PgRow> for ChangeEventRequest {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(ChangeEventRequest {
            id: row.try_get("id")?,
            location: row.try_get("location")?,
            address: row.try_get("address")?,
            start_time: row.try_get::<NaiveDateTime, _>("start_time")?.and_utc(),
            end_time: row.try_get::<NaiveDateTime, _>("end_time")?.and_utc(),
            max_attendees: row.try_get("max_attendees")?,
            latitude: row.try_get("latitude")?,
            longitude: row.try_get("longitude")?,
            status: row.try_get("status")?,
            change_reason: row.try_get("change_reason")?,
            rejection_reason: row.try_get("rejection_reason")?,
            event_id: row.try_get("event_id")?,
            facility_id: row.try_get("facility_id")?,
            facility_email: row.try_get("facility_email")?,
            facility_name: row.try_get("facility_name")?,
            facility_address: row.try_get("facility_address")?,
            facility_phone_number: row.try_get("facility_phone_number")?,
            organiser_id: row.try_get("organiser_id")?,
            organiser_email: row.try_get("organiser_email")?,
            organiser_name: row.try_get("organiser_name")?,
            organiser_phone_number: row.try_get("organiser_phone_number")?,
            state_id: row.try_get("state_id")?,
            state_name: row.try_get("state_name")?,
            district_id: row.try_get("district_id")?,
            district_name: row.try_get("district_name")?,
        })
    }
}

// Fields used to create a Change Event Request.
pub struct ChangeEventRequestForCreate {
    pub location: String,
    pub address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub change_reason: String,
    pub event_id: i64,
    pub facility_id: i64,
    pub organiser_id: i64,
    pub state_id: i32,
    pub district_id: i32,
}

// Fields used to update a Change Event Request
pub struct ChangeEventRequestForUpdate {
    pub status: EventRequestStatus,
    pub rejection_reason: Option<String>,
}

// Change Event Request Model Controller
pub struct ChangeEventRequestModelController;

impl ChangeEventRequestModelController {
    // Creates a Change Event Request, provided that there is no existing pending request from the same event and organiser.
    pub async fn create(
        model_manager: &ModelManager,
        request_created: ChangeEventRequestForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let mut transaction = db.begin().await?;

        let change_request_exists = sqlx::query_as::<_, (i32,)>("SELECT 1 FROM change_blood_donation_events_requests WHERE organiser_id = $1 AND event_id = $2 AND status = 'Pending'")
            .bind(request_created.organiser_id)
            .bind(request_created.event_id)
            .fetch_optional(&mut *transaction)
            .await?;

        if change_request_exists.is_some() {
            transaction.rollback().await?;
            return Err(Error::ExistingChangeEventRequest);
        }

        let (id,) = sqlx::query_as(
            "INSERT INTO change_blood_donation_events_requests (location, address, start_time, end_time, max_attendees, latitude, longitude, change_reason, event_id, facility_id, organiser_id, state_id, district_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) returning id",
        )
        .bind(request_created.location)
        .bind(request_created.address)
        .bind(request_created.start_time.naive_utc())
        .bind(request_created.end_time.naive_utc())
        .bind(request_created.max_attendees)
        .bind(request_created.latitude)
        .bind(request_created.longitude)
        .bind(request_created.change_reason)
        .bind(request_created.event_id)
        .bind(request_created.facility_id)
        .bind(request_created.organiser_id)
        .bind(request_created.state_id)
        .bind(request_created.district_id)
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(id)
    }

    // Gets a Change Event Request by its id
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<ChangeEventRequest> {
        let db = model_manager.db();

        let event = sqlx::query_as(
            "SELECT change_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM change_blood_donation_events_requests JOIN blood_collection_facilities ON change_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON change_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON change_blood_donation_events_requests.state_id = states.id JOIN districts ON change_blood_donation_events_requests.district_id = districts.id WHERE change_blood_donation_events_requests.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "change_event_request",
            field: I64Error(id),
        })?;

        Ok(event)
    }

    // Lists all change events for an organiser
    pub async fn list_by_organiser(
        model_manager: &ModelManager,
        organiser_id: i64,
    ) -> Result<Vec<ChangeEventRequest>> {
        let db = model_manager.db();

        let events = sqlx::query_as("SELECT change_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM change_blood_donation_events_requests JOIN blood_collection_facilities ON change_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON change_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON change_blood_donation_events_requests.state_id = states.id JOIN districts ON change_blood_donation_events_requests.district_id = districts.id WHERE organiser_id = $1 ORDER BY id")
            .bind(organiser_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Lists all change events for a facility
    pub async fn list_by_facility(
        model_manager: &ModelManager,
        facility_id: i64,
    ) -> Result<Vec<ChangeEventRequest>> {
        let db = model_manager.db();

        let events = sqlx::query_as("SELECT change_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM change_blood_donation_events_requests JOIN blood_collection_facilities ON change_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON change_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON change_blood_donation_events_requests.state_id = states.id JOIN districts ON change_blood_donation_events_requests.district_id = districts.id WHERE facility_id = $1 ORDER BY id")
            .bind(facility_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Updates a change event request
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        updated_request: ChangeEventRequestForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder =
            sqlx::QueryBuilder::new("UPDATE change_blood_donation_events_requests SET ");

        query_builder.push("status  = ");
        query_builder.push_bind(updated_request.status);

        if let Some(reason) = updated_request.rejection_reason {
            query_builder.push(", ");
            query_builder.push("rejection_reason = ").push_bind(reason);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();
        query.execute(db).await?;

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use chrono::{DurationRound, TimeDelta};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let test_time = Utc::now()
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();
        let change_request = ChangeEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_create_ok@example.com".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason".to_string(),
            event_id: 1,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = ChangeEventRequestModelController::create(&model_manager, change_request).await?;

        // Check
        let event = ChangeEventRequestModelController::get(&model_manager, id).await?;

        assert_eq!(event.address, "test_create_ok@example.com");
        assert_eq!(event.start_time, test_time);
        assert_eq!(event.end_time, test_time);
        assert_eq!(event.max_attendees, 10);
        assert_eq!(event.status, EventRequestStatus::Pending);
        assert_eq!(event.change_reason, "test_change_reason");
        assert_eq!(event.rejection_reason, None);
        assert_eq!(event.facility_id, 1);
        assert_eq!(event.organiser_id, 1);
        assert_eq!(event.state_id, 1);
        assert_eq!(event.district_id, 1);

        // Clean
        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
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
        let res = ChangeEventRequestModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "change_event_request",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_organiser_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let test_time = Utc::now()
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();
        let change_request_1 = ChangeEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_list_ok-event 01".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_1".to_string(),
            event_id: 1,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };
        let change_request_2 = ChangeEventRequestForCreate {
            location: "test location 2".to_string(),
            address: "test_list_ok-event 02".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 20,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_2".to_string(),
            event_id: 2,
            facility_id: 1,
            organiser_id: 2,
            state_id: 2,
            district_id: 2,
        };
        let change_request_3 = ChangeEventRequestForCreate {
            location: "test location 3".to_string(),
            address: "test_list_ok-event 03".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 20,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_3".to_string(),
            event_id: 2,
            facility_id: 1,
            organiser_id: 1,
            state_id: 2,
            district_id: 2,
        };

        // Execute
        let id1 =
            ChangeEventRequestModelController::create(&model_manager, change_request_1).await?;
        let id2 =
            ChangeEventRequestModelController::create(&model_manager, change_request_2).await?;
        let id3 =
            ChangeEventRequestModelController::create(&model_manager, change_request_3).await?;

        // Check
        let events =
            ChangeEventRequestModelController::list_by_organiser(&model_manager, 1).await?;
        assert_eq!(events.len(), 2, "testing list by organiser - 1");
        let events =
            ChangeEventRequestModelController::list_by_organiser(&model_manager, 2).await?;
        assert_eq!(events.len(), 1, "testing list by organiser - 2");

        // Clean
        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;

        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id3)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_facility_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let test_time = Utc::now()
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();
        let change_request_1 = ChangeEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_list_ok-event 01".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_1".to_string(),
            event_id: 1,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };
        let change_request_2 = ChangeEventRequestForCreate {
            location: "test location 2".to_string(),
            address: "test_list_ok-event 02".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 20,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_2".to_string(),
            event_id: 2,
            facility_id: 2,
            organiser_id: 1,
            state_id: 2,
            district_id: 2,
        };
        let change_request_3 = ChangeEventRequestForCreate {
            location: "test location 3".to_string(),
            address: "test_list_ok-event 03".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 20,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason_3".to_string(),
            event_id: 2,
            facility_id: 1,
            organiser_id: 1,
            state_id: 2,
            district_id: 2,
        };

        // Execute
        let id1 =
            ChangeEventRequestModelController::create(&model_manager, change_request_1).await?;
        let id2 =
            ChangeEventRequestModelController::create(&model_manager, change_request_2).await?;
        let id3 =
            ChangeEventRequestModelController::create(&model_manager, change_request_3).await?;

        // Check
        let events = ChangeEventRequestModelController::list_by_facility(&model_manager, 1).await?;
        assert_eq!(events.len(), 2, "testing list by facility - 1");
        let events = ChangeEventRequestModelController::list_by_facility(&model_manager, 1).await?;
        assert_eq!(events.len(), 1, "testing list by facility - 2");

        // Clean
        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;

        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id3)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let non_updated_time = Utc::now()
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();
        let change_request = ChangeEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_create_ok@example.com".to_string(),
            start_time: non_updated_time,
            end_time: non_updated_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            change_reason: "test_change_reason".to_string(),
            event_id: 1,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = ChangeEventRequestModelController::create(&model_manager, change_request).await?;

        let updated_request = ChangeEventRequestForUpdate {
            status: EventRequestStatus::Rejected,
            rejection_reason: Some("Rejected".to_string()),
        };

        ChangeEventRequestModelController::update(&model_manager, id, updated_request).await?;

        // Check
        let event = ChangeEventRequestModelController::get(&model_manager, id).await?;
        assert_eq!(event.status, EventRequestStatus::Rejected);
        assert_eq!(event.rejection_reason, Some(String::from("Rejected")));

        // Clean
        sqlx::query("DELETE FROM change_blood_donation_events_requests WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
