// Module declarations
mod application;
mod auth;
mod config;
mod context;
mod error;
mod job;
mod log;
mod model;
mod scheduler;
mod state;
mod utils;
mod web;

pub mod _dev_utils;

// Modules
use application::Application;
use scheduler::CronJobScheduler;
use state::AppState;
use tracing_subscriber::{self, EnvFilter};

// Re-exports
pub use self::error::{Error, Result};
pub use config::config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialise development environment
    _dev_utils::init_dev().await;

    // Initialize AppState
    let app_state = AppState::new().await?;

    // Initialise CRON job scheduler
    CronJobScheduler::run(app_state.clone()).await?;

    // Build the application and run it
    let application = Application::build_router(&app_state);
    application.run().await?;
    
    Ok(())
}
