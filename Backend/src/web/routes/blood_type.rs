// Modules
use crate::model::enums::blood_type::BloodTypeModelController;
use crate::web::Result;

use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

// Routes
pub fn routes() -> Router {
    Router::new().route("/blood-types", get(list_blood_types_handler))
}

// Handler that lists all blood types
async fn list_blood_types_handler() -> Result<Json<Value>> {
    debug!("{:<12} - list_blood_types_api", "HANDLER");

    let blood_types = BloodTypeModelController::list();

    let body = Json(json!({
        "data": {
            "bloodTypes": blood_types,
        }
    }));

    Ok(body)
}
