use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("External service error: {0}")]
    ExternalServiceError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
    pub request_id: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::AuthenticationError(msg) => {
                (StatusCode::UNAUTHORIZED, "authentication_error", msg.clone())
            }
            AppError::AuthorizationError(msg) => {
                (StatusCode::FORBIDDEN, "authorization_error", msg.clone())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "validation_error", msg.clone())
            }
            AppError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                e.to_string(),
            ),
            AppError::ExternalServiceError(e) => (
                StatusCode::BAD_GATEWAY,
                "external_service_error",
                e.to_string(),
            ),
            AppError::SerializationError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "serialization_error",
                e.to_string(),
            ),
            AppError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg.clone())
            }
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limit_exceeded",
                "Too many requests".to_string(),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            AppError::ServiceUnavailable(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, "service_unavailable", msg.clone())
            }
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message,
            details: None,
            request_id: uuid::Uuid::new_v4().to_string(),
        };

        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
