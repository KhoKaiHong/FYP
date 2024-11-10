mod auth;
mod config;
mod context;
mod error;
mod log;
mod model;
mod state;
mod utils;
mod web;

pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use axum::http::header::{
    ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_LANGUAGE, CONTENT_TYPE, RANGE,
};
use axum::http::Method;
use axum::{middleware, Router};
use model::ModelManager;
use resend_rs::Resend;
use state::AppState;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
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

    // Initialize CORS
    let cors_layer = CorsLayer::new()
        //Allow auth headers and CORS-safelisted request headers
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            ACCEPT_LANGUAGE,
            CONTENT_LANGUAGE,
            CONTENT_TYPE,
            RANGE,
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([config().FRONTEND_URL.parse().expect("Invalid frontend URL")])
        .expose_headers(Any);
    
    // Initialize routes
    let routes_all = Router::new()
        .merge(web::routes::login::routes(app_state.clone()))
        .merge(web::routes::register::routes(app_state.clone()))
        .merge(web::routes::refresh::routes(app_state.clone()))
        .nest(
            "/api",
            Router::new()
                .merge(web::routes::hello::routes())
                .merge(web::routes::logout::routes(app_state.clone()))
                .merge(web::routes::get_credentials::routes(app_state.clone()))
                .merge(web::routes::logout_all::routes(app_state.clone()))
                // Add other protected routes here
                .layer(middleware::from_fn(web::middleware::auth::require_auth)),
        )
        .layer(middleware::map_response(web::middleware::response_mapper))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::middleware::context_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(cors_layer)
        .fallback_service(web::routes::fallback::serve_dir());

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
