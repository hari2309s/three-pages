pub mod audio;
pub mod books;
pub mod health;
pub mod search;
pub mod summary;

use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

pub use audio::get_audio;
pub use books::get_book;
pub use health::{health_check, simple_health_check};
pub use search::search_books;
pub use summary::generate_summary;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheResponse {
    pub status: String,
    pub message: String,
    pub entries_cleared: u64,
    pub cache_stats: CacheStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub estimated_size: u64,
    pub entry_count: u64,
    pub hit_rate: f64,
}

pub async fn clear_cache(State(state): State<AppState>) -> Json<CacheResponse> {
    tracing::warn!("Cache clear requested - this will invalidate all cached entries");

    // Get cache stats before clearing
    let pre_clear_stats = state.cache.get_stats().await;
    let entries_before = pre_clear_stats.entry_count;

    // Actually clear the cache
    state.cache.invalidate_all().await;

    // Get post-clear stats
    let post_clear_stats = state.cache.get_stats().await;

    tracing::info!(
        "Cache cleared successfully. Removed {} entries",
        entries_before
    );

    Json(CacheResponse {
        status: "success".to_string(),
        message: format!(
            "Cache cleared successfully. {} entries were invalidated.",
            entries_before
        ),
        entries_cleared: entries_before,
        cache_stats: post_clear_stats,
    })
}
