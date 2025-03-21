// Modules
mod error;

pub mod password;
pub mod token;
pub mod role;

// Re-exports
pub use self::error::{Error, Result};
pub use self::role::Role;