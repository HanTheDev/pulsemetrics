use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::time::Instant;

use crate::{db, models::AppResult, AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: DatabaseHealth,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub latency_ms: u64,
}

/// Health check endpoint
/// 
/// Returns 200 if all systems are operational
/// Returns 503 if database is unavailable
pub async fn health_check(
    State(state): State<AppState>,
) -> AppResult<(StatusCode, Json<HealthResponse>)> {
    let start = Instant::now();

    // Check database connection
    let db_status = match db::health_check(&state.db).await {
        Ok(_) => {
            let latency = start.elapsed().as_millis() as u64;
            (
                StatusCode::OK,
                DatabaseHealth {
                    status: "healthy".to_string(),
                    latency_ms: latency,
                },
            )
        }
        Err(e) => {
            tracing::error!("Database health check failed: {:?}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                DatabaseHealth {
                    status: "unhealthy".to_string(),
                    latency_ms: 0,
                },
            )
        }
    };

    let response = HealthResponse {
        status: if db_status.0 == StatusCode::OK {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        database: db_status.1,
        uptime_seconds: 0, // TODO: Track actual uptime
    };

    Ok((db_status.0, Json(response)))
}

/// Readiness check (for Kubernetes)
pub async fn readiness(State(state): State<AppState>) -> AppResult<StatusCode> {
    db::health_check(&state.db).await?;
    Ok(StatusCode::OK)
}

/// Liveness check (for Kubernetes)
pub async fn liveness() -> StatusCode {
    StatusCode::OK
}