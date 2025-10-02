use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    models::{Book, BookDetail},
    services::books::{
        BookAggregatorService, GoogleBooksService, GutenbergService, OpenLibraryService,
    },
    utils::errors::Result,
    AppState,
};

pub async fn get_book(
    State(state): State<AppState>,
    Path(book_id): Path<String>,
) -> Result<Json<BookDetail>> {
    let cache_key = format!("book:{}", book_id);

    // Check cache first
    if let Some(cached) = state.cache.get_json::<BookDetail>(&cache_key).await {
        tracing::info!("Returning cached book details for ID: {}", book_id);
        return Ok(Json(cached));
    }

    // Initialize services
    let google_books = GoogleBooksService::new(
        state.http_client.clone(),
        state.config.google_books_api_key.clone(),
    );

    let open_library = OpenLibraryService::new(state.http_client.clone());

    let gutenberg = GutenbergService::new(
        state.http_client.clone(),
        state.config.gutenberg_api_base.clone(),
    );

    let aggregator = BookAggregatorService::new(google_books, open_library, gutenberg);

    // Try to get book details from aggregator
    let book_detail = aggregator.get_book_details(&book_id).await?;

    // Cache the result
    state.cache.set_json(cache_key, &book_detail).await;

    tracing::info!("Retrieved book details for ID: {}", book_id);
    Ok(Json(book_detail))
}
