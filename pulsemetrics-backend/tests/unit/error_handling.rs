use pulsemetrics_backend::models::AppError;
use axum::http::StatusCode;

#[test]
fn test_bad_request_status_code() {
    let error = AppError::BadRequest("test".to_string());
    assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_unauthorized_status_code() {
    let error = AppError::Unauthorized("test".to_string());
    assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_not_found_status_code() {
    let error = AppError::NotFound("test".to_string());
    assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn test_rate_limited_status_code() {
    let error = AppError::RateLimited;
    assert_eq!(error.status_code(), StatusCode::TOO_MANY_REQUESTS);
}