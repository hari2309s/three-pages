mod api;
mod config;
mod middleware;
mod models;
mod services;
mod utils;

use anyhow::Result;

use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::routes::create_router,
    config::Settings,
    services::{cache::CacheService, storage::DatabaseService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Settings,
    pub db: DatabaseService,
    pub cache: CacheService,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "book_summarizer_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Settings::new()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database
    let db = DatabaseService::new(&config.database_url, config.database_pool_size).await?;
    tracing::info!("Database connection established");

    // Run migrations
    db.run_migrations().await?;
    tracing::info!("Database migrations completed");

    // Initialize cache
    let cache = CacheService::new(config.cache_max_capacity, config.cache_ttl_seconds);
    tracing::info!("Cache service initialized");

    // Create HTTP client
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Create application state
    let state = AppState {
        config: config.clone(),
        db,
        cache,
        http_client,
    };

    // Build application router
    let app = create_router(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
