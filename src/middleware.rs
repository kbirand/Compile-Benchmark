use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{auth::AuthService, AppState};

pub async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.headers_mut().insert(
        header::HeaderName::from_static("x-request-id"),
        request_id.parse().unwrap(),
    );
    
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::HeaderName::from_static("x-request-id"),
        request_id.parse().unwrap(),
    );
    
    response
}

pub async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    if status.is_server_error() {
        warn!(
            method = %method,
            uri = %uri,
            version = ?version,
            status = %status,
            duration_ms = %duration.as_millis(),
            "Request completed with error"
        );
    } else {
        info!(
            method = %method,
            uri = %uri,
            version = ?version,
            status = %status,
            duration_ms = %duration.as_millis(),
            "Request completed"
        );
    }
    
    response
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        let auth_service = AuthService::new(
            state.config.auth.jwt_secret.clone(),
            state.config.auth.token_expiry_seconds,
            state.config.auth.refresh_token_expiry_seconds,
        );

        if let Ok(token) = AuthService::extract_token_from_header(auth_header) {
            if auth_service.validate_access_token(token).is_ok() {
                return Ok(next.run(request).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Simple rate limiting placeholder
    // In production, use governor or similar
    Ok(next.run(request).await)
}

pub async fn cors_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        "*".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, Authorization".parse().unwrap(),
    );
    
    response
}
