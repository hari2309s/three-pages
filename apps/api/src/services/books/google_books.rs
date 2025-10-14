use reqwest::Client;
use serde::Deserialize;

use crate::{
    models::{Book, BookSource, VolumeInfo},
    utils::errors::{AppError, Result},
};

#[derive(Deserialize)]
struct GoogleBooksResponse {
    items: Option<Vec<BookItem>>,
}

#[derive(Deserialize)]
struct BookItem {
    id: String,
    #[serde(rename = "volumeInfo")]
    volume_info: VolumeInfo,
}

pub struct GoogleBooksService {
    client: Client,
    api_key: Option<String>,
}

impl GoogleBooksService {
    pub fn new(client: Client, api_key: Option<String>) -> Self {
        Self { client, api_key }
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Book>> {
        let mut url = format!(
            "https://www.googleapis.com/books/v1/volumes?q={}&maxResults={}",
            urlencoding::encode(query),
            limit
        );

        if let Some(key) = &self.api_key {
            url.push_str(&format!("&key={}", key));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Google Books API error: {}",
                response.status()
            )));
        }

        let data: GoogleBooksResponse = response.json().await?;

        Ok(data
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|item| self.convert_to_book(item))
            .collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Book>> {
        let mut url = format!("https://www.googleapis.com/books/v1/volumes/{}", id);

        if let Some(key) = &self.api_key {
            url.push_str(&format!("?key={}", key));
        }

        let response = self.client.get(&url).send().await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Google Books API error: {}",
                response.status()
            )));
        }

        let item: BookItem = response.json().await?;

        Ok(Some(self.convert_to_book(item)))
    }

    fn convert_to_book(&self, item: BookItem) -> Book {
        let volume = item.volume_info;
        let isbn = volume.extract_isbn();

        Book {
            id: format!("google:{}", item.id),
            title: volume.title,
            authors: volume.authors,
            description: volume.description,
            isbn,
            publisher: volume.publisher,
            published_date: volume.published_date,
            page_count: volume.page_count,
            language: volume.language,
            cover_url: volume
                .image_links
                .and_then(|img| img.thumbnail.or(img.small_thumbnail)),
            preview_link: volume.preview_link,
            source: BookSource::Google,
        }
    }
}
