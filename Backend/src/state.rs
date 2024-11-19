use axum::extract::FromRef;
use resend_rs::Resend;

use crate::model::ModelManager;
use crate::Result;
use crate::config;


#[derive(Clone)]
pub struct AppState {
    // Cloning the ModelManager is cheap as the internal DB pool is not cloned.
    pub model_manager: ModelManager,
    // Cloning the Resend client is cheap as the internal HTTP client is not cloned.
    pub email_manager: Resend,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let app_state = AppState {
            model_manager: ModelManager::new().await?,
            email_manager: Resend::new(config().email_client.resend_api_key.as_str()),
        };

        Ok(app_state)
    }
}

// Converts AppState to ModelManager for handlers to access substate for model.
impl FromRef<AppState> for ModelManager {
    fn from_ref(app_state: &AppState) -> ModelManager {
        app_state.model_manager.clone()
    }
}

// Converts AppState to Resend for handlers to access substate for email services.
impl FromRef<AppState> for Resend {
    fn from_ref(app_state: &AppState) -> Resend {
        app_state.email_manager.clone()
    }
}