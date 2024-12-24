// Modules
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{EnumString, EnumIter};
use strum::IntoEnumIterator;

// Registration Status
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString, EnumIter)]
#[sqlx(type_name = "registration_status")]
pub enum RegistrationStatus {
    Registered,
    Absent,
    Attended,
}

pub struct RegistrationStatusModelController;

impl RegistrationStatusModelController {
    pub fn list() -> Vec<RegistrationStatus> {
        RegistrationStatus::iter().collect()
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
        let registration_status = RegistrationStatusModelController::list();

        // Check
        assert_eq!(registration_status.len(), 3);

        Ok(())
    }
}