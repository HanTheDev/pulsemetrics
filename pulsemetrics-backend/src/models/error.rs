use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application-wide error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// Get HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get user-facing error message
    fn message(&self) -> String {
        match self {
            // For production, don't expose internal error details
            AppError::Database(_) | AppError::Internal(_) => {
                "An internal error occurred".to_string()
            }
            _ => self.to_string(),
        }
    }

    /// Get error code for client-side handling
    fn error_code(&self) -> &str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            AppError::RateLimited => "RATE_LIMITED",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // Log internal errors
        if matches!(
            self,
            AppError::Database(_) | AppError::Internal(_)
        ) {
            tracing::error!("Internal error: {:?}", self);
        }

        let body = Json(json!({
            "error": {
                "code": self.error_code(),
                "message": self.message(),
            }
        }));

        (status, body).into_response()
    }
}

/// Result type alias for handlers
pub type AppResult<T> = Result<T, AppError>;