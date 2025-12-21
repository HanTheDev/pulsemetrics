use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::{handlers, middleware as mw, AppState};

/// Build the application router
pub fn create_router(state: AppState) -> Router {
    // Health check routes (no auth required)
    let health_routes = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/ready", get(handlers::readiness))
        .route("/live", get(handlers::liveness));

    // API routes (auth required)
    let api_routes = Router::new()
        .route("/ingest", post(handlers::ingest_events))
        .layer(middleware::from_fn_with_state(state.clone(), mw::auth));

    // Combine routes
    Router::new()
        .nest("/api", api_routes)
        .merge(health_routes)
        .layer(middleware::from_fn(mw::log_request))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive()) // Will configure CORS properly for production
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}