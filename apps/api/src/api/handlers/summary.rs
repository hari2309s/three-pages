use axum::{
    extract::{Path, State},
    Json,
};
use hex;
use sha2::{Digest, Sha256};
use std::time::Duration;
use tokio::time::timeout;

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
    Path(book_id): Path<String>,
    Json(payload): Json<SummaryRequest>,
) -> Result<Json<SummaryResponse>> {
    // Validate inputs with detailed error messages
    validators::validate_query(&book_id)
        .map_err(|_| AppError::InvalidInput(format!("Invalid book ID format: {}", book_id)))?;
    validators::validate_language(&payload.language).map_err(|_| {
        AppError::InvalidInput(format!("Unsupported language: {}", payload.language))
    })?;
    validators::validate_style(&payload.style)
        .map_err(|_| AppError::InvalidInput(format!("Invalid summary style: {}", payload.style)))?;

    let max_pages = payload.max_pages.unwrap_or(3);

    // Create more comprehensive cache key including language
    let cache_key = format!(
        "summary:{}:{}:{}:{}",
        book_id, max_pages, payload.style, payload.language
    );

    tracing::info!(
        "Processing summary request - Book: {}, Style: {}, Language: {}, Max Pages: {}, Cache Key: {}",
        book_id,
        payload.style,
        payload.language,
        max_pages,
        cache_key
    );

    // Check cache first with timeout
    let cache_result = timeout(
        Duration::from_millis(100),
        state.cache.get_json::<SummaryResponse>(&cache_key),
    )
    .await;

    if let Ok(Some(cached)) = cache_result {
        tracing::info!(
            "Returning cached summary for book: {} with style: {} (cache hit)",
            book_id,
            payload.style
        );
        return Ok(Json(cached));
    }

    tracing::info!(
        "No cached summary found for book: {} with style: {} - generating new summary",
        book_id,
        payload.style
    );

    // Initialize book services with timeout protection
    let google_books = GoogleBooksService::new(
        state.http_client.clone(),
        state.config.google_books_api_key.clone(),
    );

    let open_library = OpenLibraryService::new(state.http_client.clone());

    let gutenberg = GutenbergService::new(
        state.http_client.clone(),
        state.config.gutenberg_api_base_url.clone(),
    );

    let book_aggregator = BookAggregatorService::new(google_books, open_library, gutenberg);

    // Get book details with timeout
    let book_detail = timeout(
        Duration::from_secs(30),
        book_aggregator.get_book_details(&book_id),
    )
    .await
    .map_err(|_| AppError::ServiceTimeout("Book lookup timed out".to_string()))?
    .map_err(|e| AppError::ServiceError(format!("Failed to fetch book details: {}", e)))?
    .ok_or_else(|| AppError::BookNotFound(format!("Book with ID {} not found", book_id)))?;

    tracing::info!(
        "Found book: '{}' by {} (source: {:?})",
        book_detail.book.title,
        book_detail.book.author_names(),
        book_detail.book.source
    );

    // Initialize HuggingFace client for summarization
    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let summarizer = SummarizerService::new(hf_client);

    // Get book content with improved error handling and timeout
    let content = match extract_book_content(&state, &book_detail).await {
        Ok(text) => text,
        Err(e) => {
            tracing::warn!("Failed to extract book content: {}, using fallback", e);
            create_fallback_content(&book_detail.book)
        }
    };

    let text_to_summarize = if content.trim().is_empty() {
        tracing::warn!(
            "No content available for book {}, using minimal fallback",
            book_id
        );
        create_minimal_fallback(&book_detail.book)
    } else {
        content
    };

    // Limit text size to prevent API issues
    let max_chars = 50000; // Reasonable limit for most summarization APIs
    let truncated_text = if text_to_summarize.len() > max_chars {
        tracing::info!(
            "Truncating content from {} to {} characters",
            text_to_summarize.len(),
            max_chars
        );
        format!("{}...", &text_to_summarize[..max_chars])
    } else {
        text_to_summarize
    };

    // Generate summary using HuggingFace with timeout and retry logic
    let summary_text = timeout(
        Duration::from_secs(120), // Generous timeout for AI processing
        summarizer.summarize(&truncated_text, &payload.language, &payload.style),
    )
    .await
    .map_err(|_| AppError::ServiceTimeout("Summary generation timed out".to_string()))?
    .map_err(|e| {
        tracing::error!("Summary generation failed: {}", e);
        AppError::ServiceError(format!("Failed to generate summary: {}", e))
    })?;

    if summary_text.trim().is_empty() {
        return Err(AppError::ServiceError(
            "Generated summary is empty".to_string(),
        ));
    }

    // Calculate word count and validate summary quality
    let word_count = summary_text.split_whitespace().count() as i32;

    if word_count < 10 {
        tracing::warn!(
            "Generated summary is very short ({} words), may be low quality",
            word_count
        );
    }

    // Create source hash for caching/deduplication
    let mut hasher = Sha256::new();
    hasher.update(truncated_text.as_bytes());
    hasher.update(payload.language.as_bytes());
    hasher.update(payload.style.as_bytes());
    let source_hash = hex::encode(hasher.finalize());

    tracing::info!(
        "Generated summary with {} words for book: {}",
        word_count,
        book_id
    );

    // Create summary record for database
    let create_summary = CreateSummary {
        book_id: book_id.clone(),
        book_title: book_detail.book.title.clone(),
        book_author: book_detail.book.author_names(),
        isbn: book_detail.book.isbn.clone(),
        language: payload.language.clone(),
        summary_text: summary_text.clone(),
        word_count,
        style: payload.style.clone(),
        source_hash,
    };

    // Save to database with timeout
    let summary = timeout(
        Duration::from_secs(10),
        state.db.create_summary(create_summary),
    )
    .await
    .map_err(|_| AppError::ServiceTimeout("Database save timed out".to_string()))?
    .map_err(|e| {
        tracing::error!("Failed to save summary to database: {}", e);
        AppError::DatabaseError(format!("Failed to save summary: {}", e))
    })?;

    let response = summary.to_response();

    // Cache the result asynchronously (don't block response)
    let cache_key_clone = cache_key.clone();
    let response_clone = response.clone();
    let cache_service = state.cache.clone();
    tokio::spawn(async move {
        cache_service
            .set_json(cache_key_clone, &response_clone)
            .await;
        tracing::debug!("Summary cached successfully");
    });

    tracing::info!(
        "Generated and saved summary for book: {} with style: {} (cache key: {})",
        book_id,
        payload.style,
        cache_key
    );

    Ok(Json(response))
}

/// Extract content from book with proper error handling
async fn extract_book_content(
    state: &AppState,
    book_detail: &crate::models::BookDetail,
) -> Result<String> {
    if !book_detail.book.has_content() {
        return Ok(book_detail.book.description.clone().unwrap_or_default());
    }

    if let Some(ref url) = book_detail.content_url {
        tracing::info!("Fetching book content from: {}", url);

        let response = timeout(Duration::from_secs(30), state.http_client.get(url).send())
            .await
            .map_err(|_| AppError::ServiceTimeout("Content fetch timed out".to_string()))?
            .map_err(|e| AppError::ServiceError(format!("Failed to fetch content: {}", e)))?;

        let content = timeout(Duration::from_secs(10), response.text())
            .await
            .map_err(|_| AppError::ServiceTimeout("Content parsing timed out".to_string()))?
            .map_err(|e| AppError::ServiceError(format!("Failed to parse content: {}", e)))?;

        if content.len() > 1000 {
            tracing::info!(
                "Successfully fetched {} characters of content",
                content.len()
            );
            Ok(content)
        } else {
            tracing::warn!("Fetched content is very short, falling back to description");
            Ok(book_detail.book.description.clone().unwrap_or_default())
        }
    } else {
        Ok(book_detail.book.description.clone().unwrap_or_default())
    }
}

/// Create fallback content when primary content extraction fails
fn create_fallback_content(book: &crate::models::Book) -> String {
    let mut content = format!("Title: {}\nAuthor(s): {}", book.title, book.author_names());

    if let Some(desc) = &book.description {
        content.push_str(&format!("\nDescription: {}", desc));
    }

    if let Some(date) = &book.published_date {
        content.push_str(&format!("\nPublished: {}", date));
    }

    content
}

/// Create minimal fallback when no content is available at all
fn create_minimal_fallback(book: &crate::models::Book) -> String {
    format!(
        "This is a book titled '{}' by {}. Unfortunately, no detailed content is available for summarization. Please note that this summary will be based on the limited information available.",
        book.title,
        book.author_names()
    )
}
