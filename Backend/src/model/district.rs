use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// region:    --- State Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct District {
    pub id: i32,
    pub name: String,
    pub state_id: i32,
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
}
