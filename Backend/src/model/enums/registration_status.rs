use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString)]
#[sqlx(type_name = "registration_status")]
pub enum RegistrationStatus {
    Registered,
    Absent,
    Attended,
}