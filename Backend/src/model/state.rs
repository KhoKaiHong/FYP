use crate::context::Context;
use crate::model::{ModelManager, Result};
use serde::Serialize;
use sqlx::FromRow;

// region:    --- State Types
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub id: i32,
    pub name: String,
}
// endregion:    --- State Types

pub struct StateModelController;

impl StateModelController {
    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<State>> {
        let db = model_manager.db();

        let states = sqlx::query_as("SELECT * from states ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(states)
    }
}