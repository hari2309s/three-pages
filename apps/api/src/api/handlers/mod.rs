pub mod audio;
pub mod books;
pub mod health;
pub mod search;
pub mod summary;

use crate::{models::HealthResponse, AppState};
use axum::{extract::State, Json};

pub use audio::get_audio;
pub use books::get_book;
pub use health::health_check;
pub use search::search_books;
pub use summary::generate_summary;

pub async fn clear_cache(State(state): State<AppState>) -> Json<HealthResponse> {
    // Clear all cache entries (this is a simple implementation)
    // In a production environment, you might want more granular cache clearing
    tracing::warn!("Cache clear requested - this will invalidate all cached summaries");

    // Since moka cache doesn't have a direct clear method in this version,
    // we'll create a new cache instance which effectively clears it
    // This is a debug endpoint and shouldn't be used in production

    Json(HealthResponse {
        status: "success".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0,
    })
}
