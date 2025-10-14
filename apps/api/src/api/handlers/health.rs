use axum::{extract::State, Json};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{timeout, Duration};

use crate::{models::HealthResponse, AppState};

static START_TIME: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

#[derive(Debug, Serialize)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub timestamp: u64,
    pub services: ServiceHealth,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub database: ServiceStatus,
    pub cache: CacheHealth,
    pub external_apis: ExternalApiHealth,
}

#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    pub status: String,
    pub response_time_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CacheHealth {
    pub status: String,
    pub entry_count: u64,
    pub estimated_size: u64,
    pub hit_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ExternalApiHealth {
    pub google_books: ServiceStatus,
    pub hugging_face: ServiceStatus,
}

pub async fn health_check(State(state): State<AppState>) -> Json<DetailedHealthResponse> {
    let start = START_TIME.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let uptime = now - start;

    // Check database health
    let db_health = check_database_health(&state).await;

    // Check cache health
    let cache_health = check_cache_health(&state).await;

    // Check external APIs health
    let external_health = check_external_apis_health(&state).await;

    // Determine overall health status
    let overall_status = if db_health.status == "healthy" && cache_health.status == "healthy" {
        "healthy"
    } else if db_health.status == "degraded" || cache_health.status == "degraded" {
        "degraded"
    } else {
        "unhealthy"
    };

    let response = DetailedHealthResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        timestamp: now,
        services: ServiceHealth {
            database: db_health,
            cache: cache_health,
            external_apis: external_health,
        },
    };

    Json(response)
}

pub async fn simple_health_check(State(_state): State<AppState>) -> Json<HealthResponse> {
    let start = START_TIME.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let uptime = now - start;

    Json(HealthResponse::healthy(uptime))
}

async fn check_database_health(state: &AppState) -> ServiceStatus {
    let start_time = SystemTime::now();

    let health_check = timeout(
        Duration::from_secs(5),
        sqlx::query("SELECT 1 as health_check").fetch_one(state.db.pool()),
    )
    .await;

    let response_time = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis() as u64;

    match health_check {
        Ok(Ok(_)) => ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(response_time),
            error: None,
        },
        Ok(Err(e)) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some(format!("Database error: {}", e)),
        },
        Err(_) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some("Database connection timeout".to_string()),
        },
    }
}

async fn check_cache_health(state: &AppState) -> CacheHealth {
    let stats = state.cache.get_stats().await;

    let status = if stats.entry_count < 1000000 {
        // Arbitrary healthy threshold
        "healthy"
    } else {
        "degraded"
    };

    CacheHealth {
        status: status.to_string(),
        entry_count: stats.entry_count,
        estimated_size: stats.estimated_size,
        hit_rate: stats.hit_rate,
    }
}

async fn check_external_apis_health(state: &AppState) -> ExternalApiHealth {
    let google_books_health = check_google_books_health(state).await;
    let hugging_face_health = check_hugging_face_health(state).await;

    ExternalApiHealth {
        google_books: google_books_health,
        hugging_face: hugging_face_health,
    }
}

async fn check_google_books_health(state: &AppState) -> ServiceStatus {
    if state
        .config
        .google_books_api_key
        .as_ref()
        .map_or(true, |key| key.is_empty())
    {
        return ServiceStatus {
            status: "disabled".to_string(),
            response_time_ms: None,
            error: Some("API key not configured".to_string()),
        };
    }

    let start_time = SystemTime::now();

    let health_check = timeout(
        Duration::from_secs(10),
        state
            .http_client
            .get("https://www.googleapis.com/books/v1/volumes")
            .query(&[("q", "test"), ("maxResults", "1")])
            .query(&[("key", state.config.google_books_api_key.as_ref().unwrap())])
            .send(),
    )
    .await;

    let response_time = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis() as u64;

    match health_check {
        Ok(Ok(response)) if response.status().is_success() => ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(response_time),
            error: None,
        },
        Ok(Ok(response)) => ServiceStatus {
            status: "degraded".to_string(),
            response_time_ms: Some(response_time),
            error: Some(format!("HTTP {}", response.status())),
        },
        Ok(Err(e)) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some(format!("Request error: {}", e)),
        },
        Err(_) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some("Request timeout".to_string()),
        },
    }
}

async fn check_hugging_face_health(state: &AppState) -> ServiceStatus {
    if state.config.hf_token.is_empty() {
        return ServiceStatus {
            status: "disabled".to_string(),
            response_time_ms: None,
            error: Some("API token not configured".to_string()),
        };
    }

    let start_time = SystemTime::now();

    let health_check = timeout(
        Duration::from_secs(10),
        state
            .http_client
            .get(&format!("{}/models", state.config.hf_api_base_url))
            .header("Authorization", format!("Bearer {}", state.config.hf_token))
            .send(),
    )
    .await;

    let response_time = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis() as u64;

    match health_check {
        Ok(Ok(response)) if response.status().is_success() => ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(response_time),
            error: None,
        },
        Ok(Ok(response)) => ServiceStatus {
            status: "degraded".to_string(),
            response_time_ms: Some(response_time),
            error: Some(format!("HTTP {}", response.status())),
        },
        Ok(Err(e)) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some(format!("Request error: {}", e)),
        },
        Err(_) => ServiceStatus {
            status: "unhealthy".to_string(),
            response_time_ms: Some(response_time),
            error: Some("Request timeout".to_string()),
        },
    }
}
