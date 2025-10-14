use reqwest::Client;
use serde::Deserialize;

use crate::{
    models::{Book, BookSource},
    utils::errors::{AppError, Result},
};

#[derive(Deserialize)]
struct OpenLibraryResponse {
    docs: Vec<OpenLibraryDoc>,
}

#[derive(Deserialize)]
struct OpenLibraryDoc {
    key: String,
    title: String,
    #[serde(default)]
    author_name: Vec<String>,
    first_publish_year: Option<i32>,
    #[serde(default)]
    isbn: Vec<String>,
    #[serde(default)]
    publisher: Vec<String>,
    number_of_pages_median: Option<i32>,
    #[serde(default)]
    language: Vec<String>,
    cover_i: Option<i64>,
    #[serde(default)]
    subject: Vec<String>,
    #[serde(default)]
    ia: Vec<String>, // Internet Archive identifiers
}

pub struct OpenLibraryService {
    client: Client,
}

impl OpenLibraryService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Book>> {
        let url = format!(
            "https://openlibrary.org/search.json?q={}&limit={}",
            urlencoding::encode(query),
            limit
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Open Library API error: {}",
                response.status()
            )));
        }

        let data: OpenLibraryResponse = response.json().await?;

        Ok(data
            .docs
            .into_iter()
            .map(|doc| self.convert_to_book(doc))
            .collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Book>> {
        let url = format!("https://openlibrary.org{}.json", id);

        let response = self.client.get(&url).send().await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Open Library API error: {}",
                response.status()
            )));
        }

        let doc: OpenLibraryDoc = response.json().await?;

        Ok(Some(self.convert_to_book(doc)))
    }

    fn convert_to_book(&self, doc: OpenLibraryDoc) -> Book {
        let cover_url = doc
            .cover_i
            .map(|id| format!("https://covers.openlibrary.org/b/id/{}-L.jpg", id));

        let description = if !doc.subject.is_empty() {
            Some(
                doc.subject
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        } else {
            None
        };

        // Check if book has Internet Archive full text available
        let has_ia_content = !doc.ia.is_empty();

        Book {
            id: format!("openlibrary:{}", doc.key),
            title: doc.title,
            authors: doc.author_name,
            description,
            isbn: doc.isbn.first().cloned(),
            publisher: doc.publisher.first().cloned(),
            published_date: doc.first_publish_year.map(|y| y.to_string()),
            page_count: doc.number_of_pages_median,
            language: doc.language.first().cloned(),
            cover_url,
            preview_link: Some(format!("https://openlibrary.org{}", doc.key)),
            source: if has_ia_content {
                // If it has IA content, we can treat it like a content source
                BookSource::OpenLibrary
            } else {
                BookSource::OpenLibrary
            },
        }
    }

    // Helper method to extract Internet Archive identifier from book
    pub async fn get_ia_identifier(&self, openlibrary_key: &str) -> Result<Option<String>> {
        let url = format!("https://openlibrary.org{}.json", openlibrary_key);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let doc: OpenLibraryDoc = response.json().await?;

        Ok(doc.ia.first().cloned())
    }
}
