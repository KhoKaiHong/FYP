// Modules
use crate::context::Context;
use crate::log::log_request;
use crate::web;

use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, to_value, Value};
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

// Error data to be sent to the client if error occurs
#[derive(Serialize)]
struct ErrorData<'a> {
    req_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<&'a Value>,
}

// Function that maps server response to client response
pub async fn response_mapper(
    ctx: Option<Context>,
    uri: Uri,
    req_method: Method,
    response: Response,
) -> Response {
    debug!("{:<12} - response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // Get the client error.
    let web_error = response
        .extensions()
        .get::<Arc<web::Error>>()
        .map(Arc::as_ref);
    let client_status_error = web_error.map(|err| err.client_status_and_error());

    // Build the error response if there is a client error
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error = to_value(client_error).ok();

            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let detail = client_error.as_ref().and_then(|v| v.get("detail"));

            let error_data = ErrorData {
                req_uuid: uuid.to_string(),
                detail: detail,
            };

            // Build the client error JSON
            let client_error_body = json!({
                "error": {
                    "message": message,
                    "data": error_data,
                }
            });

            debug!("CLIENT ERROR BODY: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    // Log the client error
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, web_error, client_error).await.expect("Logging should not fail");

    debug!("\n");
    // If no error response, return the normalresponse
    error_response.unwrap_or(response)
}
