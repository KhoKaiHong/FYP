// Modules
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{EnumString, EnumIter};
use strum::IntoEnumIterator;

// Event Request Status
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString, EnumIter)]
#[sqlx(type_name = "event_request_status")]
pub enum EventRequestStatus {
    Pending,
    Approved,
    Rejected,
}

pub struct EventRequestStatusModelController;

impl EventRequestStatusModelController {
    pub fn list() -> Vec<EventRequestStatus> {
        EventRequestStatus::iter().collect()
    }
}