// Modules
use crate::config::config;
use crate::state::AppState;
use crate::{web, Error, Result};

use axum::http::header::{
    ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_LANGUAGE, CONTENT_TYPE, RANGE,
};
use axum::http::Method;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub struct Application {
    router: Router,
}

impl Application {
    pub fn build_router(app_state: &AppState) -> Self {
        // Initialize CORS
        let cors_layer = CorsLayer::new()
            .allow_headers([
                AUTHORIZATION,
                ACCEPT,
                ACCEPT_LANGUAGE,
                CONTENT_LANGUAGE,
                CONTENT_TYPE,
                RANGE,
            ])
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PATCH,
                Method::PUT,
            ])
            .allow_origin([config()
                .frontend
                .frontend_url
                .parse()
                .expect("Invalid frontend URL")])
            .expose_headers(Any);

        // Public routes
        let routes_public = Router::new()
            .merge(web::routes::login::routes(app_state.clone()))
            .merge(web::routes::register::routes_public(app_state.clone()))
            .merge(web::routes::refresh::routes(app_state.clone()))
            .merge(web::routes::district::routes(app_state.clone()))
            .merge(web::routes::state::routes(app_state.clone()))
            .merge(web::routes::event::list_routes(app_state.clone()))
            .merge(web::routes::facility::list_route(app_state.clone()))
            .merge(web::routes::blood_type::routes());

        // User-specific routes
        let routes_require_user: Router = Router::new()
            .merge(web::routes::event_registration::register_route(
                app_state.clone(),
            ))
            .merge(web::routes::user::user_routes(app_state.clone()))
            .merge(web::routes::event_registration::list_by_user_id_route(
                app_state.clone(),
            ))
            .merge(web::routes::donation_history::list_by_user_id(
                app_state.clone(),
            ))
            .merge(web::routes::user_notifications::routes(app_state.clone()))
            .layer(middleware::from_fn(web::middleware::auth::require_user));

        // Facility-specific routes
        let routes_require_facility: Router = Router::new()
            .merge(web::routes::facility::update_route(app_state.clone()))
            .merge(web::routes::new_event_request::list_by_facility_route(
                app_state.clone(),
            ))
            .merge(web::routes::new_event_request::update_by_facility_route(
                app_state.clone(),
            ))
            .merge(web::routes::event::list_routes_facility(app_state.clone()))
            .merge(web::routes::change_event_request::list_by_facility_route(
                app_state.clone(),
            ))
            .merge(web::routes::change_event_request::update_by_facility_route(
                app_state.clone(),
            ))
            .merge(web::routes::event_registration::update_route(
                app_state.clone(),
            ))
            .merge(web::routes::event_registration::list_by_event_id_route(
                app_state.clone(),
            ))
            .merge(web::routes::facility_notifications::routes(
                app_state.clone(),
            ))
            .layer(middleware::from_fn(web::middleware::auth::require_facility));

        // Organiser-specific routes
        let routes_require_organiser: Router = Router::new()
            .merge(web::routes::organiser::routes(app_state.clone()))
            .merge(web::routes::new_event_request::list_by_organiser_route(
                app_state.clone(),
            ))
            .merge(web::routes::new_event_request::post_route(
                app_state.clone(),
            ))
            .merge(web::routes::event::list_routes_organiser(app_state.clone()))
            .merge(web::routes::change_event_request::list_by_organiser_route(
                app_state.clone(),
            ))
            .merge(web::routes::change_event_request::post_route(
                app_state.clone(),
            ))
            .merge(web::routes::organiser_notifications::routes(
                app_state.clone(),
            ))
            .layer(middleware::from_fn(
                web::middleware::auth::require_organiser,
            ));
        
        // Admin-specific routes
        let routes_require_admin: Router = Router::new()
            .merge(web::routes::admin::routes(app_state.clone()))
            .merge(web::routes::register::routes_admin(app_state.clone()))
            .merge(web::routes::admin_notifications::routes(app_state.clone()))
            .layer(middleware::from_fn(web::middleware::auth::require_admin));

        // Authorised routes
        let routes_require_auth = Router::new()
            .merge(web::routes::logout::routes(app_state.clone()))
            .merge(web::routes::get_credentials::routes(app_state.clone()))
            .merge(web::routes::logout_all::routes(app_state.clone()))
            .merge(routes_require_user)
            .merge(routes_require_facility)
            .merge(routes_require_organiser)
            .merge(routes_require_admin)
            .layer(middleware::from_fn(web::middleware::auth::require_auth));

        // Initialize routes
        let router = Router::new()
            // Nest all routes with an /api prefix
            .nest(
                "/api",
                Router::new()
                    .merge(routes_public)
                    .merge(routes_require_auth),
            )
            // Middlewares
            .layer(middleware::map_response(web::middleware::response_mapper))
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                web::middleware::context_resolver,
            ))
            .layer(CookieManagerLayer::new())
            .layer(cors_layer)
            // Fallback
            .fallback_service(web::routes::fallback::serve_dir());

        Self { router }
    }

    pub async fn run(self) -> Result<()> {
        // Start server
        let address = format!(
            "{}:{}",
            config().application.host,
            config().application.port
        );

        let listener = TcpListener::bind(address)
            .await
            .map_err(|_| Error::ApplicationError)?;

        info!(
            "{:<12} - {:?}\n",
            "LISTENING ON",
            listener.local_addr().unwrap()
        );

        axum::serve(listener, self.router.into_make_service())
            .await
            .map_err(|_| Error::ApplicationError)?;

        Ok(())
    }
}
