use axum::extract::FromRef;
use resend_rs::Resend;

use crate::model::ModelManager;


#[derive(Clone)]
pub struct AppState {
    // Cloning the ModelManager is cheap as the internal DB pool is not cloned.
    pub model_manager: ModelManager,
    // Cloning the Resend client is cheap as the internal HTTP client is not cloned.
    pub email_manager: Resend,
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