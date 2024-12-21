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