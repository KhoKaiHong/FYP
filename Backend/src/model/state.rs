use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

// region:    --- State Types
#[derive(Debug, Clone, FromRow, Serialize)]
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