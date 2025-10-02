use axum::{Json, extract::State};

use crate::{
    AppState,
    models::{SearchRequest, SearchResponse},
    services::{
        books::{BookAggregatorService, GoogleBooksService, GutenbergService, OpenLibraryService},
        huggingface::{HuggingFaceClient, NLPService},
    },
    utils::{errors::Result, validators},
};

pub async fn search_books(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResponse>> {
    validators::validate_query(&payload.query)?;

    let cache_key = format!("search:{}", payload.query);

    if let Some(cached) = state.cache.get_json::<SearchResponse>(&cache_key).await {
        tracing::info!("Returning cached search results for: {}", payload.query);
        return Ok(Json(cached));
    }

    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let nlp_service = NLPService::new(hf_client);

    let query_intent = nlp_service
        .understand_query(&payload.query)
        .await
        .unwrap_or_else(|_| {
            tracing::warn!("NLP service failed, using simple query");
            crate::models::QueryIntent::simple(payload.query.clone())
        });

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

    let results = aggregator
        .search(&query_intent.search_query, payload.limit)
        .await?;

    let response = SearchResponse {
        total_results: results.len(),
        results,
        query_understood: query_intent,
    };

    state.cache.set_json(cache_key, &response).await;

    Ok(Json(response))
}
