use anyhow::Context;
use pulsemetrics_backend::{
    config::Config,
    db::{create_pool, run_migrations},
    routes::create_router,
    AppState,
};
use std::time::Duration;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init_tracing();

    tracing::info!("Starting PulseMetrics Backend");

    // Load configuration
    let config = Config::from_env().context("Failed to load configuration")?;
    tracing::info!("Configuration loaded successfully");
    tracing::debug!("Environment: {:?}", config.app.environment);

    // Create database connection pool
    let pool = create_pool(&config.database)
        .await
        .context("Failed to create database pool")?;

    // Run migrations
    run_migrations(&pool)
        .await
        .context("Failed to run database migrations")?;

    // Create application state
    let state = AppState::new(pool, config.clone());

    // Build router
    let app = create_router(state);

    // Get socket address
    let addr = config
        .server
        .socket_addr()
        .context("Failed to parse socket address")?;

    tracing::info!("Server listening on {}", addr);

    // Create server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind to address")?;

    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server error")?;

    tracing::info!("Server shut down gracefully");

    Ok(())
}

/// Initialize tracing/logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,pulsemetrics_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, initiating shutdown");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, initiating shutdown");
        },
    }

    // Give connections time to close
    tokio::time::sleep(Duration::from_secs(1)).await;
}