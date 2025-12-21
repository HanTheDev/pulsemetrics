use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{models::AppError, AppState};

/// Simple API key authentication middleware
pub async fn auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract API key from Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    // Expected format: "Bearer <api_key>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

    // Validate API key
    if token != state.config.app.api_key {
        return Err(AppError::Unauthorized("Invalid API key".to_string()));
    }

    Ok(next.run(req).await)
}