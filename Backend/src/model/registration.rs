// Modules
use crate::model::enums::{BloodType, RegistrationStatus};
use crate::model::EntityErrorField::I64Error;
use crate::model::{Error, ModelManager, Result};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// Registration
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub id: i64,
    pub status: RegistrationStatus,
    pub event_id: i64,
    pub event_location: String,
    pub event_address: String,
    pub event_start_time: DateTime<Utc>,
    pub event_end_time: DateTime<Utc>,
    pub event_max_attendees: i32,
    pub event_latitude: f64,
    pub event_longitude: f64,
    pub user_id: i64,
    pub user_ic_number: String,
    pub user_name: String,
    pub user_email: String,
    pub user_phone_number: String,
    pub user_blood_type: BloodType,
    pub registered_at: DateTime<Utc>,
}

// Defines how to convert a row from the database into an Registration struct.
impl<'r> FromRow<'r, PgRow> for Registration {
    fn from_row(row: &'r PgRow) -> core::result::Result<Self, sqlx::Error> {
        Ok(Registration {
            id: row.try_get("id")?,
            status: row.try_get("status")?,
            event_location: row.try_get("event_location")?,
            event_id: row.try_get("event_id")?,
            event_address: row.try_get("event_address")?,
            event_start_time: row
                .try_get::<NaiveDateTime, _>("event_start_time")?
                .and_utc(),
            event_end_time: row.try_get::<NaiveDateTime, _>("event_end_time")?.and_utc(),
            event_max_attendees: row.try_get("event_max_attendees")?,
            event_latitude: row.try_get("event_latitude")?,
            event_longitude: row.try_get("event_longitude")?,
            user_id: row.try_get("user_id")?,
            user_ic_number: row.try_get("user_ic_number")?,
            user_name: row.try_get("user_name")?,
            user_email: row.try_get("user_email")?,
            user_phone_number: row.try_get("user_phone_number")?,
            user_blood_type: row.try_get("user_blood_type")?,
            registered_at: row.try_get::<NaiveDateTime, _>("registered_at")?.and_utc(),
        })
    }
}

// Fields used to create a registration.
#[derive(Deserialize)]
pub struct RegistrationForCreate {
    pub event_id: i64,
    pub user_id: i64,
}

// Fields used to uptdate a registration.
#[derive(Deserialize)]
pub struct RegistrationForUpdate {
    pub status: Option<RegistrationStatus>,
}

// Registration Errors to propagate to client
#[derive(Debug, Serialize)]
pub enum RegistrationError {
    EventAtCapacity,
    ExistingEventRegistration,
}

// Error boilerplate
impl core::fmt::Display for RegistrationError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for RegistrationError {}

// Registration Model Controller
pub struct RegistrationModelController;

impl RegistrationModelController {
    // Creates a new registration
    pub async fn create(
        model_manager: &ModelManager,
        registration_created: RegistrationForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let mut transaction = db.begin().await?;

        let (max_attendees, current_attendees): (i32, i64) = sqlx::query_as(
            "
            SELECT max_attendees, 
                   (SELECT COUNT(*) FROM registrations WHERE event_id = $1) AS current_attendees
            FROM blood_donation_events
            WHERE id = $1
            FOR UPDATE
            ",
        )
        .bind(registration_created.event_id)
        .fetch_one(&mut *transaction)
        .await?;

        let already_registered: Option<i64> = sqlx::query_scalar(
            "
            SELECT id
            FROM registrations
            WHERE user_id = $1 AND status = 'Registered'
            FOR UPDATE
            ",
        )
        .bind(registration_created.user_id)
        .fetch_optional(&mut *transaction)
        .await?;

        if already_registered.is_some() {
            transaction.rollback().await?;
            return Err(Error::EventRegistration(
                RegistrationError::ExistingEventRegistration,
            ));
        }

        if current_attendees >= max_attendees as i64 {
            transaction.rollback().await?;
            return Err(Error::EventRegistration(RegistrationError::EventAtCapacity));
        }

        let (id,) = sqlx::query_as(
            "INSERT INTO registrations (event_id, user_id) values ($1, $2) returning id",
        )
        .bind(registration_created.event_id)
        .bind(registration_created.user_id)
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(id)
    }

    // Gets a new registration
    pub async fn get(model_manager: &ModelManager, id: i64) -> Result<Registration> {
        let db = model_manager.db();

        let registration = sqlx::query_as(
            "SELECT registrations.*, blood_donation_events.location AS event_location, blood_donation_events.address AS event_address, blood_donation_events.start_time AS event_start_time, blood_donation_events.end_time AS event_end_time, blood_donation_events.max_attendees AS event_max_attendees, blood_donation_events.latitude AS event_latitude, blood_donation_events.longitude AS event_longitude, users.ic_number AS user_ic_number, users.name AS user_name, users.email AS user_email, users.phone_number AS user_phone_number, users.blood_type AS user_blood_type FROM registrations JOIN blood_donation_events ON registrations.event_id = blood_donation_events.id JOIN users ON registrations.user_id = users.id WHERE registrations.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: "registration",
            field: I64Error(id),
        })?;

        Ok(registration)
    }

    // Lists all registrations for an event
    pub async fn list_by_event_id(
        model_manager: &ModelManager,
        event_id: i64,
    ) -> Result<Vec<Registration>> {
        let db = model_manager.db();

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM blood_donation_events WHERE id = $1")
            .bind(event_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "event",
                field: I64Error(event_id),
            })?;

        let registrations = sqlx::query_as(
            "SELECT registrations.*, blood_donation_events.location AS event_location, blood_donation_events.address AS event_address, blood_donation_events.start_time AS event_start_time, blood_donation_events.end_time AS event_end_time, blood_donation_events.max_attendees AS event_max_attendees, blood_donation_events.latitude AS event_latitude, blood_donation_events.longitude AS event_longitude, users.ic_number AS user_ic_number, users.name AS user_name, users.email AS user_email, users.phone_number AS user_phone_number, users.blood_type AS user_blood_type FROM registrations JOIN blood_donation_events ON registrations.event_id = blood_donation_events.id JOIN users ON registrations.user_id = users.id WHERE event_id = $1 ORDER BY id",
        )
        .bind(event_id)
        .fetch_all(db)
        .await?;

        Ok(registrations)
    }

    // Lists all registrations for a user
    pub async fn list_by_user_id(
        model_manager: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<Registration>> {
        let db = model_manager.db();

        let registrations = sqlx::query_as(
            "SELECT registrations.*, blood_donation_events.location AS event_location, blood_donation_events.address AS event_address, blood_donation_events.start_time AS event_start_time, blood_donation_events.end_time AS event_end_time, blood_donation_events.max_attendees AS event_max_attendees, blood_donation_events.latitude AS event_latitude, blood_donation_events.longitude AS event_longitude, users.ic_number AS user_ic_number, users.name AS user_name, users.email AS user_email, users.phone_number AS user_phone_number, users.blood_type AS user_blood_type FROM registrations JOIN blood_donation_events ON registrations.event_id = blood_donation_events.id JOIN users ON registrations.user_id = users.id WHERE user_id = $1 ORDER BY id",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        Ok(registrations)
    }

    // Updates a registration
    pub async fn update(
        model_manager: &ModelManager,
        id: i64,
        registration_updated: RegistrationForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE registrations SET ");

        let mut first = true;

        if let Some(status) = registration_updated.status {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("status = ");
            query_builder.push_bind(status);
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

    // Gets number of registrations for an event
    pub async fn get_num_of_registrations(
        model_manager: &ModelManager,
        event_id: i64,
    ) -> Result<i64> {
        let db = model_manager.db();

        // Check if the event exists
        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM blood_donation_events WHERE id = $1")
            .bind(event_id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "event",
                field: I64Error(event_id),
            })?;

        let (count,): (i64,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM registrations WHERE event_id = $1")
                .bind(event_id)
                .fetch_one(db)
                .await?;

        Ok(count)
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
        let registration_created = RegistrationForCreate {
            event_id: 1,
            user_id: 1000,
        };

        // Execute
        let id = RegistrationModelController::create(&model_manager, registration_created).await?;

        // Check
        let registration = RegistrationModelController::get(&model_manager, id).await?;

        println!("registration for test_create: {:?}", registration);

        assert_eq!(registration.event_id, 1);
        assert_eq!(registration.user_id, 1000);
        assert_eq!(registration.status, RegistrationStatus::Registered);

        // Clean
        sqlx::query("DELETE from registrations where id = $1")
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
        let res = RegistrationModelController::get(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "registration",
                    field: I64Error(100),
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_event_id() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let registration_created1 = RegistrationForCreate {
            event_id: 1,
            user_id: 1000,
        };
        let registration_created2 = RegistrationForCreate {
            event_id: 1,
            user_id: 1001,
        };

        // Execute
        let id1 =
            RegistrationModelController::create(&model_manager, registration_created1).await?;
        let id2 =
            RegistrationModelController::create(&model_manager, registration_created2).await?;
        let registrations =
            RegistrationModelController::list_by_event_id(&model_manager, 1).await?;

        assert_eq!(registrations.len(), 3, "number of seeded registrations.");
        assert_eq!(registrations[1].event_id, 1);
        assert_eq!(registrations[2].event_id, 1);

        println!("registration1 for test_update: {:?}", registrations[1]);
        println!("registration2 for test_update: {:?}", registrations[2]);

        // Clean
        sqlx::query("DELETE from registrations where id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE from registrations where id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_by_event_id_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = RegistrationModelController::list_by_event_id(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "event",
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
    async fn test_update_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let registration_created = RegistrationForCreate {
            event_id: 1,
            user_id: 1000,
        };

        // Execute
        let id = RegistrationModelController::create(&model_manager, registration_created).await?;

        let registration_updated = RegistrationForUpdate {
            status: Some(RegistrationStatus::Attended),
        };

        RegistrationModelController::update(&model_manager, id, registration_updated).await?;

        // Check
        let registration = RegistrationModelController::get(&model_manager, id).await?;
        println!("registration for test_update: {:?}", registration);
        assert_eq!(registration.event_id, 1);
        assert_eq!(registration.user_id, 1000);
        assert_eq!(registration.status, RegistrationStatus::Attended);

        // Clean
        sqlx::query("DELETE from registrations where id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_num_of_registrations() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let registration_created1 = RegistrationForCreate {
            event_id: 1,
            user_id: 1000,
        };
        let registration_created2 = RegistrationForCreate {
            event_id: 1,
            user_id: 1001,
        };

        // Execute
        let id1 =
            RegistrationModelController::create(&model_manager, registration_created1).await?;
        let id2 =
            RegistrationModelController::create(&model_manager, registration_created2).await?;
        let num_of_registrations =
            RegistrationModelController::get_num_of_registrations(&model_manager, 1).await?;

        // Check
        assert_eq!(num_of_registrations, 3);

        // Clean
        sqlx::query("DELETE from registrations where id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE from registrations where id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_num_of_registrations_err_not_found() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;
        let id = 100;

        // Execute
        let res = RegistrationModelController::get_num_of_registrations(&model_manager, id).await;

        // Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "event",
                    field: I64Error(100),
                })
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }
}
