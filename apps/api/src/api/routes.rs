use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{api::handlers, middleware::create_cors_layer, AppState};

pub fn create_router(state: AppState) -> Router {
    let cors = create_cors_layer(state.config.allowed_origins.clone());

    Router::new()
        .route("/api/health", get(handlers::simple_health_check))
        .route("/api/health/detailed", get(handlers::health_check))
        .route("/api/search", post(handlers::search_books))
        .route("/api/books/:id", get(handlers::get_book))
        .route("/api/books/:id/summary", post(handlers::generate_summary))
        .route("/api/summary/:id/audio", get(handlers::get_audio))
        .route("/api/cache/clear", delete(handlers::clear_cache))
        .layer(cors)
        .with_state(state)
}
