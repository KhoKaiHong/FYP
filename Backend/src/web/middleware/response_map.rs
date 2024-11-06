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

#[derive(Serialize)]
struct ErrorData<'a> {
    req_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<&'a Value>,
}

pub async fn response_mapper(
    ctx: Option<Context>,
    uri: Uri,
    req_method: Method,
    response: Response,
) -> Response {
    debug!("{:<12} - response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let web_error = response
        .extensions()
        .get::<Arc<web::Error>>()
        .map(Arc::as_ref);
    let client_status_error = web_error.map(|err| err.client_status_and_error());

    // -- If client error, build the new reponse.
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

            let client_error_body = json!({
                "error": {
                    "message": message,
                    "data": error_data,
                }
            });

            debug!("CLIENT ERROR BODY: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, web_error, client_error).await;

    debug!("\n");
    error_response.unwrap_or(response)
}
