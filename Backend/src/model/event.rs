// Modules
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// Event
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub location: String,
    pub address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub current_attendees: i32,
    pub max_attendees: i32,
    pub latitude: f64,
    pub longitude: f64,
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

// Defines how to convert a row from the database into an Event struct.
impl<'r> FromRow<'r, PgRow> for Event {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(Event {
            id: row.try_get("id")?,
            location: row.try_get("location")?,
            address: row.try_get("address")?,
            start_time: row.try_get::<NaiveDateTime, _>("start_time")?.and_utc(),
            end_time: row.try_get::<NaiveDateTime, _>("end_time")?.and_utc(),
            current_attendees: row.try_get("current_attendees")?,
            max_attendees: row.try_get("max_attendees")?,
            latitude: row.try_get("latitude")?,
            longitude: row.try_get("longitude")?,
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

// Fields used to create an Event.
pub struct EventForCreate {
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

// Fields used to update an Event
pub struct EventForUpdate {
    pub location: Option<String>,
    pub address: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub max_attendees: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

// Event Model Controller
pub struct EventModelController;

impl EventModelController {
    // Creates an Event
    pub async fn create(
        model_manager: &ModelManager,
        event_created: EventForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO blood_donation_events (location, address, start_time, end_time, max_attendees, latitude, longitude, facility_id, organiser_id, state_id, district_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) returning id",
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
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    // Gets an Event by its id
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<Event> {
        let db = model_manager.db();

        let event = sqlx::query_as(
            "SELECT 
                blood_donation_events.*, 
                blood_collection_facilities.email AS facility_email, 
                blood_collection_facilities.name AS facility_name, 
                blood_collection_facilities.address AS facility_address, 
                blood_collection_facilities.phone_number AS facility_phone_number, 
                event_organisers.email AS organiser_email, 
                event_organisers.name AS organiser_name, 
                event_organisers.phone_number AS organiser_phone_number, 
                states.name AS state_name, 
                districts.name AS district_name, 
                COALESCE(registration_counts.current_attendees, 0) AS current_attendees 
            FROM 
                blood_donation_events 
            JOIN 
                blood_collection_facilities ON blood_donation_events.facility_id = blood_collection_facilities.id 
            JOIN 
                event_organisers ON blood_donation_events.organiser_id = event_organisers.id 
            JOIN 
                states ON blood_donation_events.state_id = states.id 
            JOIN 
                districts ON blood_donation_events.district_id = districts.id
            LEFT JOIN (
                SELECT 
                    event_id, 
                    COUNT(*)::INTEGER AS current_attendees
                FROM 
                    registrations
                GROUP BY 
                    event_id
            ) AS registration_counts ON blood_donation_events.id = registration_counts.event_id 
            WHERE 
                blood_donation_events.id = $1")
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "event",
            field: I64Error(id),
        })?;

        Ok(event)
    }

    // Lists all Events
    pub async fn list(model_manager: &ModelManager) -> Result<Vec<Event>> {
        let db = model_manager.db();

        let events = sqlx::query_as(
            "SELECT 
                blood_donation_events.*, 
                blood_collection_facilities.email AS facility_email, 
                blood_collection_facilities.name AS facility_name, 
                blood_collection_facilities.address AS facility_address, 
                blood_collection_facilities.phone_number AS facility_phone_number, 
                event_organisers.email AS organiser_email, 
                event_organisers.name AS organiser_name, 
                event_organisers.phone_number AS organiser_phone_number, 
                states.name AS state_name, 
                districts.name AS district_name, 
                COALESCE(registration_counts.current_attendees, 0) AS current_attendees 
            FROM 
                blood_donation_events 
            JOIN 
                blood_collection_facilities ON blood_donation_events.facility_id = blood_collection_facilities.id 
            JOIN 
                event_organisers ON blood_donation_events.organiser_id = event_organisers.id 
            JOIN 
                states ON blood_donation_events.state_id = states.id 
            JOIN 
                districts ON blood_donation_events.district_id = districts.id
            LEFT JOIN (
                SELECT 
                    event_id, 
                    COUNT(*)::INTEGER AS current_attendees
                FROM 
                    registrations
                GROUP BY 
                    event_id
            ) AS registration_counts ON blood_donation_events.id = registration_counts.event_id 
            ORDER BY 
                id")
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Lists all future Events
    pub async fn list_future_events(model_manager: &ModelManager) -> Result<Vec<Event>> {
        let db = model_manager.db();

        let events = sqlx::query_as("
            SELECT 
                blood_donation_events.*, 
                blood_collection_facilities.email AS facility_email, 
                blood_collection_facilities.name AS facility_name, 
                blood_collection_facilities.address AS facility_address, 
                blood_collection_facilities.phone_number AS facility_phone_number, 
                event_organisers.email AS organiser_email, 
                event_organisers.name AS organiser_name, 
                event_organisers.phone_number AS organiser_phone_number, 
                states.name AS state_name, 
                districts.name AS district_name, 
                COALESCE(registration_counts.current_attendees, 0) AS current_attendees 
            FROM 
                blood_donation_events 
            JOIN 
                blood_collection_facilities ON blood_donation_events.facility_id = blood_collection_facilities.id 
            JOIN 
                event_organisers ON blood_donation_events.organiser_id = event_organisers.id 
            JOIN 
                states ON blood_donation_events.state_id = states.id 
            JOIN 
                districts ON blood_donation_events.district_id = districts.id 
            LEFT JOIN (
                SELECT 
                    event_id, 
                    COUNT(*)::INTEGER AS current_attendees
                FROM 
                    registrations
                GROUP BY 
                    event_id
            ) AS registration_counts ON blood_donation_events.id = registration_counts.event_id 
            WHERE 
                start_time > CURRENT_TIMESTAMP 
            ORDER BY 
                id")
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Lists all Events for an Organiser
    pub async fn list_by_organiser(
        model_manager: &ModelManager,
        organiser_id: i64,
    ) -> Result<Vec<Event>> {
        let db = model_manager.db();

        let events = sqlx::query_as(
            "SELECT 
                blood_donation_events.*, 
                blood_collection_facilities.email AS facility_email, 
                blood_collection_facilities.name AS facility_name, 
                blood_collection_facilities.address AS facility_address, 
                blood_collection_facilities.phone_number AS facility_phone_number, 
                event_organisers.email AS organiser_email, 
                event_organisers.name AS organiser_name, 
                event_organisers.phone_number AS organiser_phone_number, 
                states.name AS state_name, 
                districts.name AS district_name, 
                COALESCE(registration_counts.current_attendees, 0) AS current_attendees 
            FROM 
                blood_donation_events 
            JOIN 
                blood_collection_facilities ON blood_donation_events.facility_id = blood_collection_facilities.id 
            JOIN 
                event_organisers ON blood_donation_events.organiser_id = event_organisers.id 
            JOIN 
                states ON blood_donation_events.state_id = states.id 
            JOIN 
                districts ON blood_donation_events.district_id = districts.id
            LEFT JOIN (
                SELECT 
                    event_id, 
                    COUNT(*)::INTEGER AS current_attendees
                FROM 
                    registrations
                GROUP BY 
                    event_id
            ) AS registration_counts ON blood_donation_events.id = registration_counts.event_id 
            WHERE 
                organiser_id = $1
            ORDER BY 
                id")
            .bind(organiser_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Lists all Events for a Facility
    pub async fn list_by_facility(
        model_manager: &ModelManager,
        facility_id: i64,
    ) -> Result<Vec<Event>> {
        let db = model_manager.db();

        let events = sqlx::query_as(
            "SELECT 
                blood_donation_events.*, 
                blood_collection_facilities.email AS facility_email, 
                blood_collection_facilities.name AS facility_name, 
                blood_collection_facilities.address AS facility_address, 
                blood_collection_facilities.phone_number AS facility_phone_number, 
                event_organisers.email AS organiser_email, 
                event_organisers.name AS organiser_name, 
                event_organisers.phone_number AS organiser_phone_number, 
                states.name AS state_name, 
                districts.name AS district_name, 
                COALESCE(registration_counts.current_attendees, 0) AS current_attendees 
            FROM 
                blood_donation_events 
            JOIN 
                blood_collection_facilities ON blood_donation_events.facility_id = blood_collection_facilities.id 
            JOIN 
                event_organisers ON blood_donation_events.organiser_id = event_organisers.id 
            JOIN 
                states ON blood_donation_events.state_id = states.id 
            JOIN 
                districts ON blood_donation_events.district_id = districts.id
            LEFT JOIN (
                SELECT 
                    event_id, 
                    COUNT(*)::INTEGER AS current_attendees
                FROM 
                    registrations
                GROUP BY 
                    event_id
            ) AS registration_counts ON blood_donation_events.id = registration_counts.event_id 
            WHERE 
                facility_id = $1
            ORDER BY 
                id")
            .bind(facility_id)
            .fetch_all(db)
            .await?;

        Ok(events)
    }

    // Updates an Event
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        event_updated: EventForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE blood_donation_events SET ");

        let mut first = true;

        if let Some(location) = event_updated.location {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("location = ");
            query_builder.push_bind(location);
            first = false;
        }

        if let Some(address) = event_updated.address {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("address = ");
            query_builder.push_bind(address);
            first = false;
        }

        if let Some(start_time) = event_updated.start_time {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("start_time = ");
            query_builder.push_bind(start_time.naive_utc());
            first = false;
        }

        if let Some(end_time) = event_updated.end_time {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("end_time = ");
            query_builder.push_bind(end_time.naive_utc());
            first = false;
        }

        if let Some(max_attendees) = event_updated.max_attendees {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("max_attendees = ");
            query_builder.push_bind(max_attendees);
            first = false;
        }

        if let Some(latitude) = event_updated.latitude {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("latitude = ");
            query_builder.push_bind(latitude);
            first = false;
        }

        if let Some(longitude) = event_updated.longitude {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("longitude = ");
            query_builder.push_bind(longitude);
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
        let event_created = EventForCreate {
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
        let id = EventModelController::create(&model_manager, event_created).await?;

        // Check
        let event = EventModelController::get(&model_manager, id).await?;

        assert_eq!(event.address, "test_create_ok@example.com");
        assert_eq!(event.start_time, test_time);
        assert_eq!(event.end_time, test_time);
        assert_eq!(event.max_attendees, 10);
        assert_eq!(event.facility_id, 1);
        assert_eq!(event.organiser_id, 1);
        assert_eq!(event.state_id, 1);
        assert_eq!(event.district_id, 1);

        // Clean
        sqlx::query("DELETE from blood_donation_events where id = $1")
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
        let res = EventModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "event",
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
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Check
        let events = EventModelController::list(&model_manager).await?;
        assert_eq!(events.len(), 19, "Testing list events");

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
        let event_created = EventForCreate {
            location: "test 1".to_string(),
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
        let id = EventModelController::create(&model_manager, event_created).await?;

        let updated_time = crate::utils::now_add_sec(300)
            .duration_trunc(TimeDelta::microseconds(1))
            .unwrap();

        let event_updated = EventForUpdate {
            location: Some("test 2".to_string()),
            address: Some("new_address@example.com".to_string()),
            start_time: None,
            end_time: Some(updated_time),
            max_attendees: None,
            latitude: Some(5.1732962387784367),
            longitude: Some(90.70668106095312),
        };

        EventModelController::update(&model_manager, id, event_updated).await?;

        // Check
        let event = EventModelController::get(&model_manager, id).await?;
        assert_eq!(event.location, "test 2");
        assert_eq!(event.address, "new_address@example.com");
        assert_eq!(event.start_time, non_updated_time);
        assert_eq!(event.end_time, updated_time);
        assert_eq!(event.max_attendees, 10);
        assert_eq!(event.latitude, 5.1732962387784367);
        assert_eq!(event.longitude, 90.70668106095312);

        // Clean
        sqlx::query("DELETE from blood_donation_events where id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
