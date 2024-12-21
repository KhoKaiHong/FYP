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