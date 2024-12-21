// Modules
use crate::model::registration::RegistrationError;
use crate::model::store;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;

// Entity Error Fields
#[derive(Debug, Serialize)]
pub enum EntityErrorField {
    I64Error(i64),
    StringError(String),
    UuidError(Uuid),
}

// Store Errors
#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        field: EntityErrorField,
    },

    DuplicateKey {
        table: &'static str,
        column: &'static str,
    },

    EventRegistration(RegistrationError),

    ExistingNewEventRequest,

    ExistingChangeEventRequest,

    Store(store::Error),

    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Conversion from other errors to Store Errors
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
