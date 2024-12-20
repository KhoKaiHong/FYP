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

pub use self::error::{Error, Result};
pub use config::config;

use application::Application;
use scheduler::CronJobScheduler;
use state::AppState;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize AppState.
    let app_state = AppState::new().await?;

    CronJobScheduler::run(app_state.clone()).await?;

    let application = Application::build_router(&app_state);
    application.run().await?;
    
    Ok(())
}
