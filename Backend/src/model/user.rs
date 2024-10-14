// Backend/src/model/user.rs
use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// region:    --- User Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub ic_number: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub blood_type: String,
    pub eligibility: EligibilityStatus,
    pub state_id: i32,
    pub district_id: i32,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub ic_number: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub blood_type: String,
    pub eligibility: Option<EligibilityStatus>,
    pub state_id: i32,
    pub district_id: i32,
}

#[derive(Deserialize, FromRow)]
pub struct UserForUpdate {
    pub password: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub blood_type: Option<String>,
    pub eligibility: Option<EligibilityStatus>,
    pub state_id: Option<i32>,
    pub district_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type)]
#[sqlx(type_name = "eligibility_status")]
pub enum EligibilityStatus {
    Eligible,
    Ineligible,
    #[sqlx(rename = "Ineligible - Condition")]
    IneligibleCondition,
}
// endregion: --- User Types

// region:    --- User Model Controller
pub struct UserBmc;

impl UserBmc {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        user_created: UserForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) = sqlx::query_as(
            "INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, eligibility, state_id, district_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id",
        )
        .bind(user_created.ic_number)
        .bind(user_created.password)
        .bind(user_created.name)
        .bind(user_created.email)
        .bind(user_created.phone_number)
        .bind(user_created.blood_type)
        .bind(user_created.eligibility.or(Some(EligibilityStatus::Eligible)))
        .bind(user_created.state_id)
        .bind(user_created.district_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get(context: &Context, model_manager: &ModelManager, id: i64) -> Result<User> {
        let db = model_manager.db();

        let user = sqlx::query_as("SELECT * from users where id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "user", id })?;

        Ok(user)
    }

    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<User>> {
        let db = model_manager.db();

        let users = sqlx::query_as("SELECT * from users ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn update(
        context: &Context,
        model_manager: &ModelManager,
        id: i64,
        user_updated: UserForUpdate,
    ) -> Result<()> {
        let db = model_manager.db();

        sqlx::query(
            "UPDATE users SET password = $1, name = $2, email = $3, phone_number = $4, blood_type = $5, eligibility = $6, state_id = $7, district_id = $8 WHERE id = $9",
        )
        .bind(user_updated.password)
        .bind(user_updated.name)
        .bind(user_updated.email)
        .bind(user_updated.phone_number)
        .bind(user_updated.blood_type)
        .bind(user_updated.eligibility)
        .bind(user_updated.state_id)
        .bind(user_updated.district_id)
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }
}
// endregion: --- User Model Controller

// Backend/src/model/user.rs
// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let user_created = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: "A+".to_string(),
            eligibility: Some(EligibilityStatus::IneligibleCondition),
            state_id: 1,
            district_id: 1,
        };

        // -- Exec
        let id = UserBmc::create(&context, &model_manager, user_created).await?;

        // -- Check
        let user = UserBmc::get(&context, &model_manager, id).await?;
        assert_eq!(user.ic_number, "1234567890");
        assert_eq!(user.password, "password");
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.phone_number, "1234567890");
        assert_eq!(user.blood_type, "A+".to_string());
        assert_eq!(user.eligibility, EligibilityStatus::IneligibleCondition);
        assert_eq!(user.state_id, 1);
        assert_eq!(user.district_id, 1);

        println!("\n\nuser: {:?}", user);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
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
        let res = UserBmc::get(&context, &model_manager, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "user",
                    id: 100
                })
            ),
            "Expected EntityNotFound error, got: {:?}",
            res
        );

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let user_created1 = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: "A+".to_string(),
            eligibility: None,
            state_id: 1,
            district_id: 1,
        };
        let user_created2 = UserForCreate {
            ic_number: "9876543210".to_string(),
            password: "password".to_string(),
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            phone_number: "9876543210".to_string(),
            blood_type: "O-".to_string(),
            eligibility: Some(EligibilityStatus::Ineligible),
            state_id: 2,
            district_id: 2,
        };

        // -- Exec
        let id1 = UserBmc::create(&context, &model_manager, user_created1).await?;
        let id2 = UserBmc::create(&context, &model_manager, user_created2).await?;
        let users = UserBmc::list(&context, &model_manager).await?;

        // -- Check
        // Seeded 5 at the first place
        assert_eq!(users.len(), 5);
        assert_eq!(users[3].ic_number, "1234567890");
        assert_eq!(users[4].ic_number, "9876543210");

        println!("\n\nuser1: {:?}", users[0]);
        println!("\n\nuser2: {:?}", users[0]);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id1)
            .execute(model_manager.db())
            .await?;
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id2)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let user_created = UserForCreate {
            ic_number: "1234567890".to_string(),
            password: "password".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "1234567890".to_string(),
            blood_type: "A+".to_string(),
            eligibility: None,
            state_id: 1,
            district_id: 1,
        };

        // -- Exec
        let id = UserBmc::create(&context, &model_manager, user_created).await?;

        let user_updated = UserForUpdate {
            password: Some("new_password".to_string()),
            name: Some("Jane Doe".to_string()),
            email: Some("jane@example.com".to_string()),
            phone_number: Some("9876543210".to_string()),
            blood_type: Some("O-".to_string()),
            eligibility: Some(EligibilityStatus::IneligibleCondition),
            state_id: Some(2),
            district_id: Some(2),
        };

        UserBmc::update(&context, &model_manager, id, user_updated).await?;

        // -- Check
        let user = UserBmc::get(&context, &model_manager, id).await?;
        assert_eq!(user.ic_number, "1234567890");
        assert_eq!(user.password, "new_password");
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "jane@example.com");
        assert_eq!(user.phone_number, "9876543210");
        assert_eq!(user.blood_type, "O-".to_string());
        assert_eq!(user.eligibility, EligibilityStatus::IneligibleCondition);
        assert_eq!(user.state_id, 2);
        assert_eq!(user.district_id, 2);

        println!("\n\nuser: {:?}", user);

        // Clean
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await?;

        Ok(())
    }
}
// endregion: --- Tests
