use axum::{extract::State, Json};
use chrono::Utc;

use crate::{
    error::Result,
    models::{HealthCheck, HealthCheckResponse, HealthStatus},
    AppState,
};

pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthCheckResponse>> {
    let mut checks = Vec::new();

    // Database health check
    let db_check = check_database(&state).await;
    checks.push(db_check);

    // Cache health check
    let cache_check = check_cache(&state).await;
    checks.push(cache_check);

    // External API health check
    let api_check = check_external_api(&state).await;
    checks.push(api_check);

    let overall_status = if checks.iter().all(|c| c.status == HealthStatus::Healthy) {
        "healthy"
    } else if checks.iter().any(|c| c.status == HealthStatus::Unhealthy) {
        "unhealthy"
    } else {
        "degraded"
    };

    let response = HealthCheckResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
        uptime_seconds: 0, // Would track actual uptime in production
        checks,
    };

    Ok(Json(response))
}

async fn check_database(state: &AppState) -> HealthCheck {
    let start = std::time::Instant::now();
    
    match sqlx::query("SELECT 1").execute(&state.db.pool).await {
        Ok(_) => HealthCheck {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Connected".to_string()),
            latency_ms: Some(start.elapsed().as_millis() as u64),
        },
        Err(e) => HealthCheck {
            name: "database".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some(format!("Error: {}", e)),
            latency_ms: Some(start.elapsed().as_millis() as u64),
        },
    }
}

async fn check_cache(state: &AppState) -> HealthCheck {
    let entry_count = state.cache.entry_count();
    
    HealthCheck {
        name: "cache".to_string(),
        status: HealthStatus::Healthy,
        message: Some(format!("{} entries cached", entry_count)),
        latency_ms: Some(0),
    }
}

async fn check_external_api(_state: &AppState) -> HealthCheck {
    // Placeholder for external API health check
    HealthCheck {
        name: "external_api".to_string(),
        status: HealthStatus::Healthy,
        message: Some("Configured".to_string()),
        latency_ms: None,
    }
}
