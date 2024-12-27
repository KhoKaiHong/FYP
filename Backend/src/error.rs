// Modules
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

// Top level Errors
#[derive(Debug)]
pub enum Error {
    // Config Error
    FailToBuildConfig,

    // Model Error
    Model(model::Error),

    // Scheduler Error
    SchedulerError,

    // Application Error
    ApplicationError,

    // Logging Error
    LoggingError,
}

// Conversion from model errors to top level Errors
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}

// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
