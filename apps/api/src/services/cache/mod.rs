use moka::future::Cache;
use std::time::Duration;

use crate::api::handlers::CacheStats;

#[derive(Clone)]
pub struct CacheService {
    cache: Cache<String, Vec<u8>>,
}

impl CacheService {
    pub fn new(max_capacity: u64, ttl_seconds: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self { cache }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: Vec<u8>) {
        self.cache.insert(key, value).await;
    }

    pub async fn remove(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub fn generate_key(prefix: &str, identifier: &str) -> String {
        format!("{}:{}", prefix, identifier)
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let data = self.get(key).await?;
        serde_json::from_slice(&data).ok()
    }

    pub async fn set_json<T: serde::Serialize>(&self, key: String, value: &T) {
        if let Ok(data) = serde_json::to_vec(value) {
            self.set(key, data).await;
        }
    }

    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
        // Force immediate cleanup
        self.cache.run_pending_tasks().await;
    }

    pub async fn get_stats(&self) -> CacheStats {
        // Run pending tasks to get accurate stats
        self.cache.run_pending_tasks().await;

        let estimated_size = self.cache.weighted_size();
        let entry_count = self.cache.entry_count();

        // For this version of moka, hit rate stats are not available
        // Return basic stats with 0.0 hit rate as placeholder
        let hit_rate = 0.0;

        CacheStats {
            estimated_size,
            entry_count,
            hit_rate,
        }
    }
}
