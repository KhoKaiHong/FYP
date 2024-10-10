#[derive(Debug, strum_macros::Display)]
pub enum Role {
    User,
    Admin,
    #[strum(to_string = "Blood Collection Facility")]
    BloodCollectionFacility,
    Organiser,
}

