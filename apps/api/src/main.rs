mod api;
mod config;
mod middleware;
mod models;
mod services;
mod utils;

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
async fn main() -> anyhow::Result<()> {
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
    let config = Settings::new().map_err(|e| {
        tracing::error!("Failed to load configuration: {}", e);
        anyhow::anyhow!("Configuration error: {}", e)
    })?;
    tracing::info!("Configuration loaded successfully");
    tracing::debug!(
        "Database URL configured: {}",
        if config.database_url.starts_with("postgres://") {
            "postgres://***"
        } else {
            "***"
        }
    );

    // Initialize database with retry logic
    let db = initialize_database(&config).await?;

    // Run migrations with better error handling
    tracing::info!("Running database migrations...");
    if let Err(e) = db.run_migrations().await {
        tracing::error!("Database migration failed: {}", e);
        tracing::error!("This is often due to:");
        tracing::error!("1. Database connection issues");
        tracing::error!("2. Missing migration files");
        tracing::error!("3. Database permissions");
        tracing::error!("4. Incorrect DATABASE_URL format");
        return Err(anyhow::anyhow!("Database migration failed: {}", e));
    }
    tracing::info!("Database migrations completed successfully");

    // Initialize cache
    let cache = CacheService::new(config.cache_max_capacity, config.cache_ttl_seconds);
    tracing::info!("Cache service initialized");

    // Create HTTP client
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
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

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address {}: {}", addr, e))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

async fn initialize_database(config: &Settings) -> anyhow::Result<DatabaseService> {
    use std::time::Duration;
    use tokio::time::sleep;

    let max_retries = 5;
    let mut retry_count = 0;

    loop {
        match DatabaseService::new(&config.database_url, config.database_pool_size).await {
            Ok(db) => {
                tracing::info!("Database connection established successfully");
                return Ok(db);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= max_retries {
                    tracing::error!(
                        "Failed to connect to database after {} attempts",
                        max_retries
                    );
                    tracing::error!("Database error: {}", e);
                    tracing::error!("Please check:");
                    tracing::error!("1. DATABASE_URL is correctly formatted");
                    tracing::error!("2. Database server is running and accessible");
                    tracing::error!("3. Network connectivity to database");
                    tracing::error!("4. Database credentials are correct");
                    return Err(anyhow::anyhow!("Database connection failed: {}", e));
                }

                let backoff_seconds = retry_count * 2;
                tracing::warn!(
                    "Database connection attempt {} failed: {}. Retrying in {} seconds...",
                    retry_count,
                    e,
                    backoff_seconds
                );
                sleep(Duration::from_secs(backoff_seconds)).await;
            }
        }
    }
}
