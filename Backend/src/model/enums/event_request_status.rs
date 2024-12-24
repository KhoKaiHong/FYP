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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_list() -> Result<()> {
        // Setup
        let model_manager = _dev_utils::init_test().await;

        // Execute
        let event_request_status = EventRequestStatusModelController::list();

        // Check
        assert_eq!(event_request_status.len(), 3);

        Ok(())
    }
}
