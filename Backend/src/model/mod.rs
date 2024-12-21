// Modules
mod error;
mod store;
pub mod user;
pub mod state;
pub mod district;
pub mod facility;
pub mod organiser;
pub mod admin;
pub mod event;
pub mod new_event_request;
pub mod change_event_request;
pub mod registration;
pub mod donation_history;
pub mod user_session;
pub mod facility_session;
pub mod organiser_session;
pub mod admin_session;
pub mod enums;
pub mod user_notification;
pub mod facility_notification;
pub mod organiser_notification;
pub mod admin_notification;

// Re-exports
pub use self::error::{EntityErrorField, Error, Result};

use store::{new_db_pool, Db};

// Model Manager
#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    // Model Manager created by establlishing a database connection pool.
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // Returns the sqlx db pool reference.
    pub fn db(&self) -> &Db {
        &self.db
    }
}
