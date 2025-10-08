use axum::{extract::State, Json};
use hex;
use sha2::{Digest, Sha256};

use crate::{
    models::{CreateSummary, SummaryRequest, SummaryResponse},
    services::{
        books::{BookAggregatorService, GoogleBooksService, GutenbergService, OpenLibraryService},
        huggingface::{HuggingFaceClient, SummarizerService},
    },
    utils::{
        errors::{AppError, Result},
        validators,
    },
    AppState,
};

pub async fn generate_summary(
    State(state): State<AppState>,
    Json(payload): Json<SummaryRequest>,
) -> Result<Json<SummaryResponse>> {
    validators::validate_query(&payload.book_id)?;

    let cache_key = format!(
        "summary:{}:{}",
        payload.book_id,
        payload.max_pages.unwrap_or(3)
    );

    // Check cache first
    if let Some(cached) = state.cache.get_json::<SummaryResponse>(&cache_key).await {
        tracing::info!("Returning cached summary for book: {}", payload.book_id);
        return Ok(Json(cached));
    }

    // Initialize book services
    let google_books = GoogleBooksService::new(
        state.http_client.clone(),
        state.config.google_books_api_key.clone(),
    );

    let open_library = OpenLibraryService::new(state.http_client.clone());

    let gutenberg = GutenbergService::new(
        state.http_client.clone(),
        state.config.gutenberg_api_base.clone(),
    );

    let book_aggregator = BookAggregatorService::new(google_books, open_library, gutenberg);

    // Get book details
    let book_detail = book_aggregator
        .get_book_details(&payload.book_id)
        .await?
        .ok_or_else(|| {
            AppError::BookNotFound(format!("Book with ID {} not found", payload.book_id))
        })?;

    // Initialize HuggingFace client for summarization
    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let summarizer = SummarizerService::new(hf_client);

    // Get book content if available
    let content = if book_detail.book.has_content() {
        // For Gutenberg books, try to fetch content
        match book_detail.content_url {
            Some(ref url) => match state.http_client.get(url).send().await {
                Ok(response) => response.text().await.ok(),
                Err(_) => book_detail.book.description.clone(),
            },
            None => book_detail.book.description.clone(),
        }
    } else {
        // Use description for other sources
        book_detail.book.description.clone()
    };

    let text_to_summarize = content.unwrap_or_else(|| {
        format!(
            "Book: {} by {}. No additional content available for summarization.",
            book_detail.book.title,
            book_detail.book.author_names()
        )
    });

    // Generate summary using HuggingFace
    let summary_text = summarizer
        .summarize(&text_to_summarize, &payload.language, &payload.style)
        .await?;

    // Calculate word count
    let word_count = summary_text.split_whitespace().count() as i32;

    // Create source hash for caching/deduplication
    let mut hasher = Sha256::new();
    hasher.update(text_to_summarize.as_bytes());
    let source_hash = hex::encode(hasher.finalize());

    // Create summary record for database
    let create_summary = CreateSummary {
        book_id: payload.book_id.clone(),
        book_title: book_detail.book.title.clone(),
        book_author: book_detail.book.author_names(),
        isbn: book_detail.book.isbn.clone(),
        language: payload.language.clone(),
        summary_text: summary_text.clone(),
        word_count,
        style: payload.style.clone(),
        source_hash,
    };

    // Save to database
    let summary = state.db.create_summary(create_summary).await?;

    let response = summary.to_response();

    // Cache the result
    state.cache.set_json(cache_key, &response).await;

    tracing::info!("Generated summary for book: {}", payload.book_id);
    Ok(Json(response))
}
