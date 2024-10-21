use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// region:    --- Facility Types

// Not needed if state and district name is required
#[derive(Debug, FromRow)]
pub struct Facility {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub state_id: i32,
}

#[derive(Debug, FromRow)]
pub struct FacilityWithLocation {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub state_id: i32,
    pub state_name: String,
}

#[derive(Deserialize)]
pub struct FacilityForCreate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub state_id: i32,
}

#[derive(Deserialize)]
pub struct FacilityForUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

// endregion:    --- Facility Types

// region:    --- Facility Model Controller
pub struct FacilityModelController;

impl FacilityModelController {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        facility_created: FacilityForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO blood_collection_facilities (email, password, name, address, phone_number, state_id) values ($1, $2, $3, $4, $5, $6) returning id",
        )
        .bind(facility_created.email)
        .bind(facility_created.password)
        .bind(facility_created.name)
        .bind(facility_created.address)
        .bind(facility_created.phone_number)
        .bind(facility_created.state_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
    ) -> Result<FacilityWithLocation> {
        let db = model_manager.db();

        let facility = sqlx::query_as(
            "SELECT blood_collection_facilities.*, states.name AS state_name FROM blood_collection_facilities JOIN states ON blood_collection_facilities.state_id = states.id WHERE blood_collection_facilities.id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound { entity: "facility", id })?;

        Ok(facility)
    }

    pub async fn list(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<FacilityWithLocation>> {
        let db = model_manager.db();

        let facilities = sqlx::query_as("SELECT blood_collection_facilities.*, states.name AS state_name FROM blood_collection_facilities JOIN states ON blood_collection_facilities.state_id = states.id ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(facilities)
    }

    pub async fn update(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
        facility_updated: FacilityForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE blood_collection_facilities SET ");

        let mut first = true;

        if let Some(email) = facility_updated.email {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("email = ");
            query_builder.push_bind(email);
            first = false;
        }

        if let Some(password) = facility_updated.password {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("password = ");
            query_builder.push_bind(password);
            first = false;
        }

        if let Some(name) = facility_updated.name {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(name);
            first = false;
        }

        if let Some(address) = facility_updated.address {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("address = ");
            query_builder.push_bind(address);
            first = false;
        }

        if let Some(phone_number) = facility_updated.phone_number {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("phone_number = ");
            query_builder.push_bind(phone_number);
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
// endregion: --- Facility Model Controller

// Backend/src/model/facility.rs
// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let facility_created = FacilityForCreate {
            email: "test_create_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Facility".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };

        // -- Exec
        let id =
            FacilityModelController::create(&context, &model_manager, facility_created).await?;

        // -- Check
        let facility = FacilityModelController::get(&context, &model_manager, id).await?;
        assert_eq!(facility.email, "test_create_ok@example.com");
        assert_eq!(facility.password, "welcome");
        assert_eq!(facility.name, "Test Facility");
        assert_eq!(facility.address, "123 Fake St");
        assert_eq!(facility.phone_number, "1234567890");
        assert_eq!(facility.state_id, 1);

        println!("\n\nfacility: {:?}", facility);

        // Clean
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
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
        let res = FacilityModelController::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "facility",
                    id: 100
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

        let facility_created1 = FacilityForCreate {
            email: "test_email1@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Facility 01".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };
        let facility_created2 = FacilityForCreate {
            email: "test_email2@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Facility 02".to_string(),
            address: "Hello Street".to_string(),
            phone_number: "987654321".to_string(),
            state_id: 2,
        };

        let id1 =
            FacilityModelController::create(&context, &model_manager, facility_created1).await?;
        let id2 =
            FacilityModelController::create(&context, &model_manager, facility_created2).await?;
        let facilities = FacilityModelController::list(&context, &model_manager).await?;

        // Check
        assert_eq!(facilities.len(), 24, "Number of facilities");
        assert_eq!(facilities[22].id, id1);
        assert_eq!(facilities[23].id, id2);
        assert_eq!(facilities[22].name, "Test Facility 01");
        assert_eq!(facilities[23].name, "Test Facility 02");
        assert_eq!(facilities[22].state_id, 1);
        assert_eq!(facilities[23].state_id, 2);
        assert_eq!(facilities[22].state_name, "Johor");
        assert_eq!(facilities[23].state_name, "Kedah");
        assert_eq!(facilities[22].address, "123 Fake St");
        assert_eq!(facilities[23].address, "Hello Street");
        assert_eq!(facilities[22].phone_number, "1234567890");
        assert_eq!(facilities[23].phone_number, "987654321");

        for facility in facilities.iter() {
            println!("facility: {:?}", facility);
        }

        // Clean
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
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
        let facility_created = FacilityForCreate {
            email: "test_list_ok@example.com".to_string(),
            password: "welcome".to_string(),
            name: "Test Facility 01".to_string(),
            address: "123 Fake St".to_string(),
            phone_number: "1234567890".to_string(),
            state_id: 1,
        };

        let id =
            FacilityModelController::create(&context, &model_manager, facility_created).await?;

        let facility_updated = FacilityForUpdate {
            email: Some("new_email@gmail.com".to_string()),
            password: None,
            name: Some("New name".to_string()),
            address: None,
            phone_number: Some("987654321".to_string()),
        };

        FacilityModelController::update(&context, &model_manager, id, facility_updated).await?;

        // -- Check
        let facility = FacilityModelController::get(&context, &model_manager, id).await?;
        assert_eq!(facility.email, "new_email@gmail.com");
        assert_eq!(facility.password, "welcome");
        assert_eq!(facility.name, "New name");
        assert_eq!(facility.address, "123 Fake St");
        assert_eq!(facility.phone_number, "987654321");

        println!("\n\nfacility: {:?}", facility);

       // Clean
       sqlx::query("DELETE FROM blood_collection_facilities WHERE id = $1")
       .bind(id)
       .execute(model_manager.db())
       .await?;

        Ok(())
    }
}
