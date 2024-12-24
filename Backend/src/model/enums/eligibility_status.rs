// Modules
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{EnumString, EnumIter};
use strum::IntoEnumIterator;

// Eligibility Status
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString, EnumIter)]
#[sqlx(type_name = "eligibility_status")]
pub enum EligibilityStatus {
    Eligible,
    Ineligible,
    #[sqlx(rename = "Ineligible - Condition")]
    #[serde(rename = "Ineligible - Condition")]
    #[strum(serialize = "Ineligible - Condition")]
    IneligibleCondition,
}

pub struct EligibilityStatusModelController;

impl EligibilityStatusModelController {
    pub fn list() -> Vec<EligibilityStatus> {
        EligibilityStatus::iter().collect()
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
        let eligibility_statuses = EligibilityStatusModelController::list();

        // Check
        assert_eq!(eligibility_statuses.len(), 3);

        Ok(())
    }
}