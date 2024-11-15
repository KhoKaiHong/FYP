use crate::context::Context;
use crate::model::{ModelManager, Result};
use serde::Serialize;
use sqlx::FromRow;

// region:    --- State Types
#[derive(Debug, FromRow)]
pub struct District {
    pub id: i32,
    pub name: String,
    pub state_id: i32,
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistrictWithState {
    pub district_id: i32,
    pub district_name: String,
    pub state_id: i32,
    pub state_name: String,
}
// endregion:    --- State Types

pub struct DistrictModelController;

impl DistrictModelController {
    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<District>> {
        let db = model_manager.db();

        let districts = sqlx::query_as("SELECT * from districts ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(districts)
    }

    pub async fn list_with_state(
        context: &Context,
        model_manager: &ModelManager,
    ) -> Result<Vec<DistrictWithState>> {
        let db = model_manager.db();

        let districts = sqlx::query_as("SELECT districts.id AS district_id, districts.name AS district_name, districts.state_id, states.name AS state_name FROM districts JOIN states ON districts.state_id = states.id ORDER BY district_id")
            .fetch_all(db)
            .await?;

        Ok(districts)
    }
}
