use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString)]
#[sqlx(type_name = "eligibility_status")]
pub enum EligibilityStatus {
    Eligible,
    Ineligible,
    #[sqlx(rename = "Ineligible - Condition")]
    #[serde(rename = "Ineligible - Condition")]
    #[strum(serialize = "Ineligible - Condition")]
    IneligibleCondition,
}