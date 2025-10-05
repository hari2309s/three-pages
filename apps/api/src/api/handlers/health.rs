use axum::{extract::State, Json};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{models::HealthResponse, AppState};

static START_TIME: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

pub async fn health_check(State(_state): State<AppState>) -> Json<HealthResponse> {
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
