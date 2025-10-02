use axum::{
    routing::{get, post},
    Router,
};

use crate::{api::handlers, middleware::create_cors_layer, AppState};

pub fn create_router(state: AppState) -> Router {
    let cors = create_cors_layer(state.config.allowed_origins.clone());

    Router::new()
        .route("/api/health", get(handlers::health_check))
        .route("/api/search", post(handlers::search_books))
        .route("/api/books/:id", get(handlers::get_book))
        .route("/api/books/:id/summary", post(handlers::generate_summary))
        .route("/api/summary/:id/audio", get(handlers::get_audio))
        .layer(cors)
        .with_state(state)
}
