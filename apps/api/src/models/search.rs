use crate::models::book::Book;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<Book>,
    pub total_results: usize,
    pub query_understood: QueryIntent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryIntent {
    pub original_query: String,
    pub extracted_terms: ExtractedTerms,
    pub search_query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractedTerms {
    pub genre: Option<String>,
    pub theme: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub title: Option<String>,
}

impl QueryIntent {
    pub fn new(original: String, terms: ExtractedTerms, search: String) -> Self {
        Self {
            original_query: original,
            extracted_terms: terms,
            search_query: search,
        }
    }

    pub fn simple(query: String) -> Self {
        Self {
            search_query: query.clone(),
            original_query: query,
            extracted_terms: ExtractedTerms::default(),
        }
    }
}
