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
        .test_before_acquire(true)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET statement_timeout = '30s'")
                    .execute(&mut *conn)
                    .await?;

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

/// Run database migrations at runtime by reading SQL file
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations");

    // Include the SQL file at compile time
    let migration_sql = include_str!("../../migrations/001_initial_schema.sql");
    
    // Execute the entire migration as a single transaction
    let mut tx = pool.begin().await?;
    
    // Split by semicolon and execute each statement
    let statements: Vec<&str> = migration_sql
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("--"))
        .collect();

    for (idx, statement) in statements.iter().enumerate() {
        tracing::debug!("Executing migration statement {}/{}", idx + 1, statements.len());
        
        sqlx::query(statement)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute statement: {}", statement);
                anyhow::anyhow!("Migration failed at statement {}: {}", idx + 1, e)
            })?;
    }

    tx.commit().await?;

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