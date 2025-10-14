use reqwest::Client;
use serde::Deserialize;

use crate::{
    models::{Book, BookSource},
    utils::errors::{AppError, Result},
};

#[derive(Deserialize)]
struct GutenbergResponse {
    results: Vec<GutenbergBook>,
}

#[derive(Deserialize)]
struct GutenbergBook {
    id: i32,
    title: String,
    authors: Vec<Author>,
    subjects: Vec<String>,

    languages: Vec<String>,
    formats: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
struct Author {
    name: String,
}

pub struct GutenbergService {
    client: Client,
    base_url: String,
}

impl GutenbergService {
    pub fn new(_client: Client, base_url: String) -> Self {
        // Create a new client that follows redirects for Gutenberg API
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client, base_url }
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Book>> {
        let url = format!(
            "{}/books/?search={}",
            self.base_url,
            urlencoding::encode(query)
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Gutenberg API error: {}",
                response.status()
            )));
        }

        let data: GutenbergResponse = response.json().await?;

        Ok(data
            .results
            .into_iter()
            .take(limit)
            .map(|book| self.convert_to_book(book))
            .collect())
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Book>> {
        let url = format!("{}/books/{}/", self.base_url, id);

        let response = self.client.get(&url).send().await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Gutenberg API error: {}",
                response.status()
            )));
        }

        let book: GutenbergBook = response.json().await?;

        Ok(Some(self.convert_to_book(book)))
    }

    fn convert_to_book(&self, book: GutenbergBook) -> Book {
        let authors: Vec<String> = book.authors.into_iter().map(|a| a.name).collect();

        let description = if !book.subjects.is_empty() {
            Some(book.subjects.join("; "))
        } else {
            None
        };

        let cover_url = book
            .formats
            .get("image/jpeg")
            .cloned()
            .or_else(|| book.formats.get("image/png").cloned());

        Book {
            id: format!("gutenberg:{}", book.id),
            title: book.title,
            authors,
            description,
            isbn: None,
            publisher: Some("Project Gutenberg".to_string()),
            published_date: None,
            page_count: None,
            language: book.languages.first().cloned(),
            cover_url,
            preview_link: Some(format!("https://www.gutenberg.org/ebooks/{}", book.id)),
            source: BookSource::Gutenberg,
        }
    }
}
