// Modules
mod error;

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// Re-exports
pub use self::error::{Error, Result};

pub type Db = Pool<Postgres>;

// Function that creates a new database connection pool.
pub async fn new_db_pool() -> Result<Db> {
    // Set max connections to 5 if not in unit test mode, 1 in unit test mode due to an issue with tokio scheduler.
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(config().database.connect_options())
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}