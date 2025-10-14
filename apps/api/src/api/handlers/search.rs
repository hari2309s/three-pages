use axum::{extract::State, Json};
use std::time::Duration;
use tokio::time::timeout;

use crate::{
    models::{SearchRequest, SearchResponse},
    services::{
        books::{BookAggregatorService, GoogleBooksService, GutenbergService, OpenLibraryService},
        huggingface::{HuggingFaceClient, NLPService},
    },
    utils::{
        errors::{AppError, Result},
        validators,
    },
    AppState,
};

pub async fn search_books(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResponse>> {
    // Validate input with detailed error messages
    validators::validate_query(&payload.query)
        .map_err(|_| AppError::InvalidInput(format!("Invalid search query: {}", payload.query)))?;

    if payload.limit > 100 {
        return Err(AppError::InvalidInput(
            "Search limit cannot exceed 100 results".to_string(),
        ));
    }

    let cache_key = format!("search:{}:{}", payload.query, payload.limit);

    tracing::info!(
        "Processing search request - Query: '{}', Limit: {}",
        payload.query,
        payload.limit
    );

    // Check cache with timeout
    let cache_result = timeout(
        Duration::from_millis(100),
        state.cache.get_json::<SearchResponse>(&cache_key),
    )
    .await;

    if let Ok(Some(cached)) = cache_result {
        tracing::info!(
            "Returning cached search results for: '{}' ({} results)",
            payload.query,
            cached.total_results
        );
        return Ok(Json(cached));
    }

    tracing::info!("No cached results found, performing new search");

    // Initialize NLP service for query understanding with timeout protection
    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let nlp_service = NLPService::new(hf_client);

    // Try to understand query with timeout and fallback
    let query_intent = timeout(
        Duration::from_secs(5), // Quick timeout for NLP processing
        nlp_service.understand_query(&payload.query),
    )
    .await
    .unwrap_or_else(|_| {
        tracing::warn!("NLP service timed out, using simple query");
        Err(AppError::ServiceTimeout("NLP service timeout".to_string()))
    })
    .unwrap_or_else(|e| {
        tracing::warn!("NLP service failed: {}, using simple query", e);
        crate::models::QueryIntent::simple(payload.query.clone())
    });

    tracing::debug!(
        "Query understanding result - Original: '{}', Processed: '{}'",
        payload.query,
        query_intent.search_query
    );

    // Initialize book services
    let google_books = GoogleBooksService::new(
        state.http_client.clone(),
        state.config.google_books_api_key.clone(),
    );

    let open_library = OpenLibraryService::new(state.http_client.clone());

    let gutenberg = GutenbergService::new(
        state.http_client.clone(),
        state.config.gutenberg_api_base_url.clone(),
    );

    let aggregator = BookAggregatorService::new(google_books, open_library, gutenberg);

    // Perform search with timeout protection
    let results = timeout(
        Duration::from_secs(30), // Generous timeout for book search
        aggregator.search(&query_intent.search_query, payload.limit),
    )
    .await
    .map_err(|_| AppError::ServiceTimeout("Book search timed out".to_string()))?
    .map_err(|e| {
        tracing::error!("Book search failed: {}", e);
        AppError::ServiceError(format!("Search operation failed: {}", e))
    })?;

    if results.is_empty() {
        tracing::info!("No books found for query: '{}'", payload.query);
    } else {
        tracing::info!(
            "Found {} books for query: '{}' from {} sources",
            results.len(),
            payload.query,
            results
                .iter()
                .map(|b| format!("{:?}", b.source))
                .collect::<std::collections::HashSet<_>>()
                .len()
        );
    }

    let response = SearchResponse {
        total_results: results.len(),
        results,
        query_understood: query_intent,
    };

    // Cache the result asynchronously (don't block response)
    let cache_key_clone = cache_key.clone();
    let response_clone = response.clone();
    let cache_service = state.cache.clone();
    tokio::spawn(async move {
        cache_service
            .set_json(cache_key_clone, &response_clone)
            .await;
        tracing::debug!("Search results cached successfully");
    });

    tracing::info!(
        "Search completed for query: '{}' - {} results returned",
        payload.query,
        response.total_results
    );

    Ok(Json(response))
}
