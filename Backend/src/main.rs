mod config;
mod context;
mod error;
mod log;
mod model;
mod state;
mod web;
mod auth;
mod utils;

pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use axum::{middleware, Router};
use model::ModelManager;
use resend_rs::Resend;
use state::AppState;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize AppState.
    let app_state = AppState {
        model_manager: ModelManager::new().await?,
        email_manager: Resend::new(config().RESEND_API_KEY.as_str()),
    };

    let routes_all = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(
            web::response_map::main_response_mapper,
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::middleware_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(web::routes_static::serve_dir());

    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    info!(
        "{:<12} - {:?}\n",
        "LISTENING ON",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
