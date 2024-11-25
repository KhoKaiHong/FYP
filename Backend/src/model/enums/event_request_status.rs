use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString)]
#[sqlx(type_name = "event_request_status")]
pub enum EventRequestStatus {
    Pending,
    Approved,
    Rejected,
}