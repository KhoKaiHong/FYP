// Modules
use crate::model;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

// Job Errors
#[serde_as]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
    JobBuildError(String),
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    ModelError(model::Error),
}

// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Conversion from other errors to Job Errors
impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::ModelError(val)
    }
}