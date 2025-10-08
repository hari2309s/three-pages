use crate::{
    models::{Book, BookDetail, BookSource},
    services::books::{GoogleBooksService, GutenbergService, OpenLibraryService},
    utils::errors::{AppError, Result},
};

pub struct BookAggregatorService {
    google_books: GoogleBooksService,
    open_library: OpenLibraryService,
    gutenberg: GutenbergService,
}

impl BookAggregatorService {
    pub fn new(
        google_books: GoogleBooksService,
        open_library: OpenLibraryService,
        gutenberg: GutenbergService,
    ) -> Self {
        Self {
            google_books,
            open_library,
            gutenberg,
        }
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Book>> {
        let per_source = (limit / 2).max(5);

        let google_result = self.google_books.search(query, per_source).await;
        let openlibrary_result = self.open_library.search(query, per_source).await;

        let results = vec![google_result, openlibrary_result];

        let mut all_books = Vec::new();

        for result in results {
            match result {
                Ok(books) => all_books.extend(books),
                Err(e) => {
                    tracing::warn!("Book source failed: {}", e);
                }
            }
        }

        all_books.sort_by(|a, b| {
            let a_score = self.calculate_relevance_score(a, query);
            let b_score = self.calculate_relevance_score(b, query);
            b_score
                .partial_cmp(&a_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        all_books.truncate(limit);

        Ok(all_books)
    }

    pub async fn get_book_details(&self, id: &str) -> Result<Option<BookDetail>> {
        let parts: Vec<&str> = id.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::InvalidInput("Invalid book ID format".to_string()));
        }

        let (source, book_id) = (parts[0], parts[1]);

        let book = match source {
            "google" => self.google_books.get_by_id(book_id).await?,
            "openlibrary" => self.open_library.get_by_id(book_id).await?,
            "gutenberg" => {
                let gid: i32 = book_id
                    .parse()
                    .map_err(|_| AppError::InvalidInput("Invalid Gutenberg ID".to_string()))?;
                self.gutenberg.get_by_id(gid).await?
            }
            _ => return Err(AppError::InvalidInput("Unknown book source".to_string())),
        };

        if let Some(book) = book {
            let detail = self.enrich_book_detail(book).await;
            Ok(Some(detail))
        } else {
            Ok(None)
        }
    }

    pub async fn get_book_content(&self, id: &str) -> Result<String> {
        let parts: Vec<&str> = id.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::InvalidInput("Invalid book ID format".to_string()));
        }

        let (source, book_id) = (parts[0], parts[1]);

        match source {
            "gutenberg" => {
                let gid: i32 = book_id
                    .parse()
                    .map_err(|_| AppError::InvalidInput("Invalid Gutenberg ID".to_string()))?;
                self.gutenberg.get_content(gid).await
            }
            "openlibrary" => {
                match self.open_library.get_ia_identifier(book_id).await? {
                    Some(ia_id) => {
                        self.open_library.get_content(book_id, &ia_id).await
                    }
                    None => {
                        Err(AppError::ExternalApi(
                            "This book does not have full text available on Internet Archive".to_string()
                        ))
                    }
                }
            }
            _ => {
                Err(AppError::InvalidInput(
                    "Content only available for Gutenberg and Open Library books with Internet Archive copies".to_string()
                ))
            }
        }
    }

    async fn enrich_book_detail(&self, book: Book) -> BookDetail {
        let content_url = match book.source {
            BookSource::Gutenberg => {
                let id = book.id.replace("gutenberg:", "");
                Some(format!(
                    "https://www.gutenberg.org/files/{}/{}-0.txt",
                    id, id
                ))
            }
            BookSource::OpenLibrary => {
                // Try to get IA identifier
                let ol_key = book.id.replace("openlibrary:", "");
                if let Ok(Some(ia_id)) = self.open_library.get_ia_identifier(&ol_key).await {
                    Some(format!(
                        "https://archive.org/download/{}/{}.txt",
                        ia_id, ia_id
                    ))
                } else {
                    None
                }
            }
            _ => None,
        };

        let gutenberg_id = if book.source == BookSource::Gutenberg {
            book.id.replace("gutenberg:", "").parse::<i32>().ok()
        } else {
            None
        };

        BookDetail {
            book,
            content_url,
            gutenberg_id,
        }
    }

    fn calculate_relevance_score(&self, book: &Book, query: &str) -> f32 {
        let mut score = 0.0;
        let query_lower = query.to_lowercase();

        if book.title.to_lowercase().contains(&query_lower) {
            score += 10.0;
        }

        for author in &book.authors {
            if author.to_lowercase().contains(&query_lower) {
                score += 8.0;
            }
        }

        if let Some(desc) = &book.description {
            if desc.to_lowercase().contains(&query_lower) {
                score += 3.0;
            }
        }

        if book.cover_url.is_some() {
            score += 1.0;
        }

        if book.description.is_some() {
            score += 1.0;
        }

        if book.source == BookSource::Gutenberg {
            score += 2.0;
        }

        score
    }
}
