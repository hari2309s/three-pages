use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Book not found: {0}")]
    BookNotFound(String),

    #[error("Summary not found")]
    SummaryNotFound,

    #[error("Audio not found")]
    AudioNotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Content too large")]
    ContentTooLarge,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::BookNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::SummaryNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::AudioNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::RateLimit => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            AppError::ContentTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, self.to_string()),
            AppError::Database(_) | AppError::Internal(_) => {
                tracing::error!("Internal error: {}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::HttpRequest(_) | AppError::ExternalApi(_) => {
                tracing::error!("External API error: {}", self);
                (
                    StatusCode::BAD_GATEWAY,
                    "External service unavailable".to_string(),
                )
            }
            AppError::Serialization(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Serialization error".to_string(),
            ),
        };

        let body = Json(json!({
            "success": false,
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
