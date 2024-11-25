mod auth;
mod config;
mod context;
mod error;
mod log;
mod model;
mod state;
mod utils;
mod web;
mod application;

pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use application::Application;
use state::AppState;
use tracing::info;
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

    let application = Application::build_router(&app_state);
    let application_task = tokio::spawn(application.run());

    tokio::select! {
        o = application_task => info!("Application task exited: {:?}", o),
    };


    Ok(())
}
