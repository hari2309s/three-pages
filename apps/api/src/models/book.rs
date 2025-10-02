use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub page_count: Option<i32>,
    pub language: Option<String>,
    pub cover_url: Option<String>,
    pub preview_link: Option<String>,
    pub source: BookSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BookSource {
    Google,
    OpenLibrary,
    Gutenberg,
}

impl Book {
    pub fn author_names(&self) -> String {
        self.authors.join(", ")
    }

    pub fn has_content(&self) -> bool {
        matches!(self.source, BookSource::Gutenberg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookDetail {
    #[serde(flatten)]
    pub book: Book,
    pub content_url: Option<String>,
    pub gutenberg_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub title: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(rename = "imageLinks")]
    pub image_links: Option<ImageLinks>,
    pub description: Option<String>,
    #[serde(rename = "industryIdentifiers")]
    pub industry_identifiers: Option<Vec<IndustryIdentifier>>,
    pub publisher: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(rename = "pageCount")]
    pub page_count: Option<i32>,
    pub language: Option<String>,
    #[serde(rename = "previewLink")]
    pub preview_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageLinks {
    pub thumbnail: Option<String>,
    #[serde(rename = "smallThumbnail")]
    pub small_thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryIdentifier {
    #[serde(rename = "type")]
    pub id_type: String,
    pub identifier: String,
}

impl VolumeInfo {
    pub fn extract_isbn(&self) -> Option<String> {
        self.industry_identifiers.as_ref()?.iter().find_map(|id| {
            if id.id_type.contains("ISBN") {
                Some(id.identifier.clone())
            } else {
                None
            }
        })
    }
}
