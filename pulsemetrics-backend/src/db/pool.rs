use crate::config::DatabaseConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Create a PostgreSQL connection pool with optimal settings
pub async fn create_pool(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    tracing::info!(
        "Initializing database connection pool (max: {}, min: {})",
        config.max_connections,
        config.min_connections
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout_seconds))
        .idle_timeout(Duration::from_secs(config.idle_timeout_seconds))
        // Test connections before acquiring
        .test_before_acquire(true)
        // Enable statement caching for performance
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Set statement timeout to 30 seconds
                sqlx::query("SET statement_timeout = '30s'")
                    .execute(&mut *conn)
                    .await?;

                // Set timezone to UTC
                sqlx::query("SET timezone = 'UTC'")
                    .execute(&mut *conn)
                    .await?;

                Ok(())
            })
        })
        .connect(&config.url)
        .await?;

    tracing::info!("Database connection pool initialized successfully");

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    tracing::info!("Database migrations completed successfully");

    Ok(())
}

/// Health check for database connection
pub async fn health_check(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;

    Ok(())
}