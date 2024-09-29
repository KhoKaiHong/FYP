pub use self::error::{Error, Result};

use crate::model::ModelController;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::debug;
use tracing_subscriber::{self, EnvFilter};

mod context;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelController.
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::middleware_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(
            web::response_map::main_response_mapper,
        ))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::middleware_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(web::routes_static::routes_static());

    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    debug!("LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
