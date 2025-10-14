use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SummaryRequest {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_style")]
    pub style: String,
    pub max_pages: Option<usize>,
}

fn default_style() -> String {
    "concise".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResponse {
    pub id: Uuid,
    pub summary_text: String,
    pub language: String,
    pub word_count: i32,
    pub book_info: BookInfo,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookInfo {
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Summary {
    pub id: Uuid,
    pub book_id: String,
    pub book_title: String,
    pub book_author: String,
    pub isbn: Option<String>,
    pub language: String,
    pub summary_text: String,
    pub word_count: i32,
    pub style: String,
    pub source_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Summary {
    pub fn to_response(self) -> SummaryResponse {
        SummaryResponse {
            id: self.id,
            summary_text: self.summary_text,
            language: self.language,
            word_count: self.word_count,
            book_info: BookInfo {
                title: self.book_title,
                author: self.book_author,
                isbn: self.isbn,
            },
            created_at: self.created_at,
        }
    }
}

#[derive(Debug)]
pub struct CreateSummary {
    pub book_id: String,
    pub book_title: String,
    pub book_author: String,
    pub isbn: Option<String>,
    pub language: String,
    pub summary_text: String,
    pub word_count: i32,
    pub style: String,
    pub source_hash: String,
}
