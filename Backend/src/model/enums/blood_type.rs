// Modules
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

// Blood Types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString, EnumIter)]
#[sqlx(type_name = "blood_type_enum")]
pub enum BloodType {
    #[sqlx(rename = "A+")]
    #[serde(rename = "A+")]
    #[strum(serialize = "A+")]
    APositive,
    #[sqlx(rename = "A-")]
    #[serde(rename = "A-")]
    #[strum(serialize = "A-")]
    ANegative,
    #[sqlx(rename = "B+")]
    #[serde(rename = "B+")]
    #[strum(serialize = "B+")]
    BPositive,
    #[sqlx(rename = "B-")]
    #[serde(rename = "B-")]
    #[strum(serialize = "B-")]
    BNegative,
    #[sqlx(rename = "O+")]
    #[serde(rename = "O+")]
    #[strum(serialize = "O+")]
    OPositive,
    #[sqlx(rename = "O-")]
    #[serde(rename = "O-")]
    #[strum(serialize = "O-")]
    ONegative,
    #[sqlx(rename = "AB+")]
    #[serde(rename = "AB+")]
    #[strum(serialize = "AB+")]
    ABPositive,
    #[sqlx(rename = "AB-")]
    #[serde(rename = "AB-")]
    #[strum(serialize = "AB-")]
    ABNegative,
}

pub struct BloodTypeModelController;

impl BloodTypeModelController {
    pub fn list() -> Vec<BloodType> {
        BloodType::iter().collect()
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
        let blood_types = BloodTypeModelController::list();

        // Check
        assert_eq!(blood_types.len(), 8);

        Ok(())
    }
}
