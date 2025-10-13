use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_environment")]
    pub environment: String,

    pub database_url: String,

    #[serde(default = "default_pool_size")]
    pub database_pool_size: u32,

    pub hf_token: String,

    #[serde(default = "default_hf_base_url")]
    pub hf_api_base_url: String,

    pub google_books_api_key: Option<String>,

    #[serde(default = "default_gutenberg_base_url")]
    pub gutenberg_api_base_url: String,

    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_seconds: u64,

    #[serde(default = "default_cache_capacity")]
    pub cache_max_capacity: u64,

    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,
}

fn default_port() -> u16 {
    10000
}

fn default_environment() -> String {
    "development".to_string()
}

fn default_pool_size() -> u32 {
    5
}

fn default_hf_base_url() -> String {
    "https://api-inference.huggingface.co".to_string()
}

fn default_gutenberg_base_url() -> String {
    "https://gutendex.com".to_string()
}

fn default_cache_ttl() -> u64 {
    3600
}

fn default_cache_capacity() -> u64 {
    1000
}

fn default_allowed_origins() -> Vec<String> {
    vec![
        "http://localhost:5173".to_string(),
        "http://localhost:3000".to_string(),
    ]
}

impl Settings {
    pub fn new() -> Result<Self> {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or_else(default_port);

        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| default_environment());

        let database_url = env::var("APP_SUPABASE_URL").context("APP_SUPABASE_URL must be set")?;

        let database_pool_size = env::var("DATABASE_POOL_SIZE")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or_else(default_pool_size);

        let hf_token =
            env::var("APP_HUGGINGFACE_API_KEY").context("APP_HUGGINGFACE_API_KEY must be set")?;

        let hf_api_base_url =
            env::var("APP_HUGGINGFACE_API_BASE_URL").unwrap_or_else(|_| default_hf_base_url());

        let google_books_api_key = env::var("GOOGLE_BOOKS_API_KEY").ok();

        let gutenberg_api_base_url =
            env::var("GUTENBERG_API_BASE_URL").unwrap_or_else(|_| default_gutenberg_base_url());

        let cache_ttl_seconds = env::var("CACHE_TTL_SECONDS")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or_else(default_cache_ttl);

        let cache_max_capacity = env::var("CACHE_MAX_CAPACITY")
            .ok()
            .and_then(|c| c.parse().ok())
            .unwrap_or_else(default_cache_capacity);

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .ok()
            .map(|o| o.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(default_allowed_origins);

        Ok(Self {
            port,
            environment,
            database_url,
            database_pool_size,
            hf_token,
            hf_api_base_url,
            google_books_api_key,
            gutenberg_api_base_url,
            cache_ttl_seconds,
            cache_max_capacity,
            allowed_origins,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
