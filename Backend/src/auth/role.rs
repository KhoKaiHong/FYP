#[derive(Debug, strum_macros::Display, strum_macros::EnumString, Clone)]
pub enum Role {
    User,
    Admin,
    #[strum(serialize = "Blood Collection Facility")]
    BloodCollectionFacility,
    Organiser,
}

