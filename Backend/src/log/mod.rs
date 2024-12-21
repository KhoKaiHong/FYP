// Modules
use crate::context::Context;
use crate::utils::{format_time, parse_utc_from_timestamp};
use crate::web::{self, ClientError};
use crate::{Error, Result};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

// Function that logs a request.
pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    context: Option<Context>,
    web_error: Option<&web::Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let timestamp = if timestamp <= i64::MAX as u64 {
        i64::try_from(timestamp).expect("Timestamp overflow")
    } else {
        return Err(Error::LoggingError);
    };

    let error_type = web_error.map(|err| err.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Creates the request log and logs it.
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        time: format_time(
            parse_utc_from_timestamp(timestamp).map_err(|_| Error::LoggingError)?,
        ),

        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        user_id: context.as_ref().map(|c| c.user_id()),
        role: context.as_ref().map(|c| c.role().to_string()),
        token_id: context.as_ref().map(|c| c.token_id().to_string()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));

    Ok(())
}

// Request Log Line
#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    // Request attributes.
    uuid: String,
    time: String,

    // Context attributes.
    user_id: Option<i64>,
    role: Option<String>,
    token_id: Option<String>,

    // HTTP request attributes.
    http_path: String,
    http_method: String,

    // Error attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
