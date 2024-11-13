use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type, EnumString)]
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