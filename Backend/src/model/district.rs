// Modules
use crate::model::{ModelManager, Result};

use serde::Serialize;
use sqlx::FromRow;

// District
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub district_id: i32,
    pub district_name: String,
    pub state_id: i32,
    pub state_name: String,
}

pub struct DistrictModelController;

impl DistrictModelController {
    // Lists all districts
    pub async fn list(
        model_manager: &ModelManager,
    ) -> Result<Vec<District>> {
        let db = model_manager.db();

        let districts = sqlx::query_as("SELECT districts.id AS district_id, districts.name AS district_name, districts.state_id, states.name AS state_name FROM districts JOIN states ON districts.state_id = states.id ORDER BY district_id")
            .fetch_all(db)
            .await?;

        Ok(districts)
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
    async fn test_list_ok() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Execute
        let districts = DistrictModelController::list(&model_manager).await?;

        // Check
        assert_eq!(districts.len(), 160, "number of districts.");

        Ok(())
    }
}
