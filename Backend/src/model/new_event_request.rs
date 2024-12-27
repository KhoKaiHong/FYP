// Modules
use crate::model::enums::EventRequestStatus;
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// New Event Request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewEventRequest {
    pub id: i64,
    pub location: String,
    pub address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub status: EventRequestStatus,
    pub rejection_reason: Option<String>,
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

// Defines how to convert a row from the database into a New Event Request struct.
impl<'r> FromRow<'r, PgRow> for NewEventRequest {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(NewEventRequest {
            id: row.try_get("id")?,
            location: row.try_get("location")?,
            address: row.try_get("address")?,
            start_time: row.try_get::<NaiveDateTime, _>("start_time")?.and_utc(),
            end_time: row.try_get::<NaiveDateTime, _>("end_time")?.and_utc(),
            max_attendees: row.try_get("max_attendees")?,
            latitude: row.try_get("latitude")?,
            longitude: row.try_get("longitude")?,
            status: row.try_get("status")?,
            rejection_reason: row.try_get("rejection_reason")?,
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

// Fields used to create a New Event Request.
pub struct NewEventRequestForCreate {
    pub location: String,
    pub address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub facility_id: i64,
    pub organiser_id: i64,
    pub state_id: i32,
    pub district_id: i32,
}

// Fields used to update a New Event Request.
pub struct NewEventRequestForUpdate {
    pub status: EventRequestStatus,
    pub rejection_reason: Option<String>,
}

// New Event Request Model Controller
pub struct NewEventRequestModelController;

impl NewEventRequestModelController {
    // Creates a new event request.
    pub async fn create(
        model_manager: &ModelManager,
        event_created: NewEventRequestForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let mut transaction = db.begin().await?;

        let event_exists = sqlx::query_as::<_, (i32,)>("SELECT 1 FROM new_blood_donation_events_requests WHERE organiser_id = $1 AND status = 'Pending'")
            .bind(event_created.organiser_id)
            .fetch_optional(&mut *transaction)
            .await?;

        if event_exists.is_some() {
            transaction.rollback().await?;
            return Err(Error::ExistingNewEventRequest);
        }

        let (id,) = sqlx::query_as(
            "INSERT INTO new_blood_donation_events_requests (location, address, start_time, end_time, max_attendees, latitude, longitude, facility_id, organiser_id, state_id, district_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) returning id",
        )
        .bind(event_created.location)
        .bind(event_created.address)
        .bind(event_created.start_time.naive_utc())
        .bind(event_created.end_time.naive_utc())
        .bind(event_created.max_attendees)
        .bind(event_created.latitude)
        .bind(event_created.longitude)
        .bind(event_created.facility_id)
        .bind(event_created.organiser_id)
        .bind(event_created.state_id)
        .bind(event_created.district_id)
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(id)
    }

    // Gets a new event request by id.
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<NewEventRequest> {
        let db = model_manager.db();

        let event = sqlx::query_as(
            "SELECT new_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM new_blood_donation_events_requests JOIN blood_collection_facilities ON new_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON new_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON new_blood_donation_events_requests.state_id = states.id JOIN districts ON new_blood_donation_events_requests.district_id = districts.id WHERE new_blood_donation_events_requests.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "new_event_request",
            field: I64Error(id),
        })?;

        Ok(event)
    }

    // Lists all new event requests for a given organiser.
    pub async fn list_by_organiser(
        model_manager: &ModelManager,
        organiser_id: i64,
    ) -> Result<Vec<NewEventRequest>> {
        let db = model_manager.db();

        let events = sqlx::query_as("SELECT new_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM new_blood_donation_events_requests JOIN blood_collection_facilities ON new_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON new_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON new_blood_donation_events_requests.state_id = states.id JOIN districts ON new_blood_donation_events_requests.district_id = districts.id WHERE organiser_id = $1 ORDER BY id")
            .bind(organiser_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Lists all new event requests for a given facility.
    pub async fn list_by_facility(
        model_manager: &ModelManager,
        facility_id: i64,
    ) -> Result<Vec<NewEventRequest>> {
        let db = model_manager.db();

        let events = sqlx::query_as("SELECT new_blood_donation_events_requests.*, blood_collection_facilities.email AS facility_email, blood_collection_facilities.name AS facility_name, blood_collection_facilities.address AS facility_address, blood_collection_facilities.phone_number AS facility_phone_number, event_organisers.email AS organiser_email, event_organisers.name AS organiser_name, event_organisers.phone_number AS organiser_phone_number, states.name AS state_name, districts.name AS district_name FROM new_blood_donation_events_requests JOIN blood_collection_facilities ON new_blood_donation_events_requests.facility_id = blood_collection_facilities.id JOIN event_organisers ON new_blood_donation_events_requests.organiser_id = event_organisers.id JOIN states ON new_blood_donation_events_requests.state_id = states.id JOIN districts ON new_blood_donation_events_requests.district_id = districts.id WHERE facility_id = $1 ORDER BY id")
            .bind(facility_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Updates a new event request.
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        updated_request: NewEventRequestForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder =
            sqlx::QueryBuilder::new("UPDATE new_blood_donation_events_requests SET ");

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
        let event_created = NewEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_create_ok@example.com".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = NewEventRequestModelController::create(&model_manager, event_created).await?;

        // Check
        let event = NewEventRequestModelController::get(&model_manager, id).await?;

        assert_eq!(event.address, "test_create_ok@example.com");
        assert_eq!(event.start_time, test_time);
        assert_eq!(event.end_time, test_time);
        assert_eq!(event.max_attendees, 10);
        assert_eq!(event.status, EventRequestStatus::Pending);
        assert_eq!(event.rejection_reason, None);
        assert_eq!(event.facility_id, 1);
        assert_eq!(event.organiser_id, 1);
        assert_eq!(event.state_id, 1);
        assert_eq!(event.district_id, 1);

        // Clean
        sqlx::query("DELETE FROM new_blood_donation_events_requests WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_create_err() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let test_time = Utc::now()
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();
        let event_created = NewEventRequestForCreate {
            location: "test location 1".to_string(),
            address: "test_create_err@example.com".to_string(),
            start_time: test_time,
            end_time: test_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            facility_id: 1,
            organiser_id: 2,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let res = NewEventRequestModelController::create(&model_manager, event_created).await;

        // Check
        assert!(
            matches!(res, Err(Error::ExistingNewEventRequest)),
            "Expected ExistingNewEventRequest error, got: {:?}",
            res
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = NewEventRequestModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "new_event_request",
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

        // Execute
        let events1 = NewEventRequestModelController::list_by_organiser(&model_manager, 1).await?;
        let events2 = NewEventRequestModelController::list_by_organiser(&model_manager, 2).await?;

        // Check
        assert_eq!(events1.len(), 6, "Testing list new event requests by organiser - 1");
        assert_eq!(events2.len(), 5, "Testing list new event requests by organiser - 2");

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_facility_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Execute
        let events1 = NewEventRequestModelController::list_by_facility(&model_manager, 1).await?;
        let events2 = NewEventRequestModelController::list_by_facility(&model_manager, 2).await?;

        assert_eq!(events1.len(), 1, "Testing list new event requests by facility - 1");
        assert_eq!(events2.len(), 0, "Testing list new event requests by facility - 2");

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
        let event_created = NewEventRequestForCreate {
            location: "test".to_string(),
            address: "test@example.com".to_string(),
            start_time: non_updated_time,
            end_time: non_updated_time,
            max_attendees: 10,
            latitude: 3.1732962387784367,
            longitude: 101.70668106095312,
            facility_id: 1,
            organiser_id: 1,
            state_id: 1,
            district_id: 1,
        };

        // Execute
        let id = NewEventRequestModelController::create(&model_manager, event_created).await?;

        let updated_request = NewEventRequestForUpdate {
            status: EventRequestStatus::Rejected,
            rejection_reason: Some("Rejected".to_string()),
        };

        NewEventRequestModelController::update(&model_manager, id, updated_request).await?;

        // Check
        let event = NewEventRequestModelController::get(&model_manager, id).await?;
        assert_eq!(event.status, EventRequestStatus::Rejected);
        assert_eq!(event.rejection_reason, Some(String::from("Rejected")));

        // Clean
        sqlx::query("DELETE FROM new_blood_donation_events_requests WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
