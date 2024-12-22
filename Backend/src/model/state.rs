// Modules
use crate::model::{ModelManager, Result};

use serde::Serialize;
use sqlx::FromRow;

// State
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub id: i32,
    pub name: String,
}

// State Model Controller
pub struct StateModelController;

impl StateModelController {
    // Lists all states
    pub async fn list(model_manager: &ModelManager) -> Result<Vec<State>> {
        let db = model_manager.db();

        let states = sqlx::query_as("SELECT * from states ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(states)
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
        let states = StateModelController::list(&model_manager).await?;

        // Check
        assert_eq!(states.len(), 3);

        Ok(())
    }
}