use crate::model::store;
use crate::model::registration::RegistrationError;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum EntityErrorField {
    I64Error(i64),
    StringError(String),
    UuidError(Uuid),
}

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    // -- Entity Not Found
    EntityNotFound {
        entity: &'static str,
        field: EntityErrorField,
    },

    // -- Duplicate Errors
    DuplicateKey {
        table: &'static str,
        column: &'static str,
    },

    // -- Event Registration Errors
    EventRegistration(RegistrationError),

    // -- Modules
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Froms
impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}

impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}
// endregion: --- Froms
