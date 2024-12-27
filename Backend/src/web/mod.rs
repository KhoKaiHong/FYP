// Modules
mod error;

pub mod middleware;
pub mod routes;

// Re-exports
pub use self::error::ClientError;
pub use self::error::{Error, Result};
