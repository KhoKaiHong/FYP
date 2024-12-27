use axum::extract::FromRef;

use crate::model::ModelManager;
use crate::Result;


#[derive(Clone)]
pub struct AppState {
    // Cloning the ModelManager is cheap as the internal DB pool is not cloned
    pub model_manager: ModelManager,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let app_state = AppState {
            model_manager: ModelManager::new().await?,
        };

        Ok(app_state)
    }
}

// Converts AppState to ModelManager for handlers to access substate for model
impl FromRef<AppState> for ModelManager {
    fn from_ref(app_state: &AppState) -> ModelManager {
        app_state.model_manager.clone()
    }
}