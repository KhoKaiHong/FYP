use crate::model::enums::blood_type::BloodTypeModelController;
use crate::web::Result;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes() -> Router {
    Router::new()
        .route("/api/bloodtypes", get(list_blood_types_handler))
}

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