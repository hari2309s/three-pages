use std::collections::HashMap;

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
        let per_source = (limit / 3).max(5);

        // Search all sources concurrently
        let (google_result, openlibrary_result, gutenberg_result) = tokio::join!(
            self.google_books.search(query, per_source),
            self.open_library.search(query, per_source),
            self.gutenberg.search(query, per_source)
        );

        let mut all_books = Vec::new();

        // Collect results from each source
        if let Ok(books) = google_result {
            tracing::debug!("Google Books returned {} results", books.len());
            all_books.extend(books);
        } else if let Err(e) = google_result {
            tracing::warn!("Google Books search failed: {}", e);
        }

        if let Ok(books) = openlibrary_result {
            tracing::debug!("Open Library returned {} results", books.len());
            all_books.extend(books);
        } else if let Err(e) = openlibrary_result {
            tracing::warn!("Open Library search failed: {}", e);
        }

        if let Ok(books) = gutenberg_result {
            tracing::debug!("Gutenberg returned {} results", books.len());
            all_books.extend(books);
        } else if let Err(e) = gutenberg_result {
            tracing::warn!("Gutenberg search failed: {}", e);
        }

        tracing::info!("Total books before deduplication: {}", all_books.len());

        // Deduplicate and prioritize
        let deduplicated = self.deduplicate_and_prioritize(all_books, query);

        // Limit results
        let final_results: Vec<Book> = deduplicated.into_iter().take(limit).collect();

        tracing::info!(
            "Final results after deduplication and limiting: {}",
            final_results.len()
        );

        Ok(final_results)
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

    fn deduplicate_and_prioritize(&self, books: Vec<Book>, query: &str) -> Vec<Book> {
        // Group books by deduplication key
        let mut book_groups: HashMap<String, Vec<Book>> = HashMap::new();

        for book in books {
            let key = self.create_dedup_key(&book);
            book_groups.entry(key).or_insert_with(Vec::new).push(book);
        }

        let mut deduplicated = Vec::new();

        // For each group, pick the best book based on source priority
        for (_, mut group) in book_groups {
            // Sort by source priority first, then by completeness/quality
            group.sort_by(|a, b| {
                let a_priority = self.get_source_priority(&a.source);
                let b_priority = self.get_source_priority(&b.source);

                // Lower number = higher priority
                if a_priority != b_priority {
                    return a_priority.cmp(&b_priority);
                }

                // If same source priority, prefer more complete records
                let a_completeness = self.calculate_completeness_score(a);
                let b_completeness = self.calculate_completeness_score(b);

                b_completeness
                    .partial_cmp(&a_completeness)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Take the best book from this group
            if let Some(best_book) = group.into_iter().next() {
                deduplicated.push(best_book);
            }
        }

        // Sort final results by relevance to query and source priority
        deduplicated.sort_by(|a, b| {
            let a_relevance = self.calculate_relevance_score(a, query);
            let b_relevance = self.calculate_relevance_score(b, query);

            // Higher relevance score comes first
            b_relevance
                .partial_cmp(&a_relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        deduplicated
    }

    fn create_dedup_key(&self, book: &Book) -> String {
        // Create a normalized key for deduplication based on title and primary author
        let title = self.normalize_string(&book.title);
        let author = if book.authors.is_empty() {
            String::new()
        } else {
            self.normalize_string(&book.authors[0])
        };
        format!("{}|{}", title, author)
    }

    fn normalize_string(&self, s: &str) -> String {
        s.to_lowercase()
            .trim()
            .replace(&[' ', '-', '_', ':', '.', ',', ';', '(', ')', '[', ']'], "")
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    fn get_source_priority(&self, source: &BookSource) -> u8 {
        match source {
            BookSource::Gutenberg => 1,   // Highest priority - full text available
            BookSource::OpenLibrary => 2, // Medium priority - good metadata
            BookSource::Google => 3,      // Lowest priority - commercial/limited
        }
    }

    fn calculate_completeness_score(&self, book: &Book) -> f32 {
        let mut score = 0.0;

        // Basic required fields
        if !book.title.is_empty() {
            score += 1.0;
        }
        if !book.authors.is_empty() {
            score += 1.0;
        }

        // Additional metadata
        if book.isbn.is_some() {
            score += 0.5;
        }
        if book.cover_url.is_some() {
            score += 0.3;
        }
        if book.description.is_some() {
            score += 0.7;
        }
        if book.published_date.is_some() {
            score += 0.2;
        }
        if book.language.is_some() {
            score += 0.2;
        }
        if book.page_count.is_some() {
            score += 0.1;
        }

        score
    }

    fn calculate_relevance_score(&self, book: &Book, query: &str) -> f32 {
        let mut score = 0.0;
        let query_lower = query.to_lowercase();

        // Title match is most important
        if book.title.to_lowercase().contains(&query_lower) {
            score += 10.0;

            // Exact title match gets bonus
            if book.title.to_lowercase() == query_lower {
                score += 5.0;
            }

            // Title starts with query gets bonus
            if book.title.to_lowercase().starts_with(&query_lower) {
                score += 3.0;
            }
        }

        // Author match is second most important
        for author in &book.authors {
            let author_lower = author.to_lowercase();
            if author_lower.contains(&query_lower) {
                score += 8.0;

                // Exact author match gets bonus
                if author_lower == query_lower {
                    score += 4.0;
                }
            }
        }

        // Description match
        if let Some(desc) = &book.description {
            if desc.to_lowercase().contains(&query_lower) {
                score += 2.0;
            }
        }

        // Source-based bonuses
        match book.source {
            BookSource::Gutenberg => score += 3.0,   // Free full-text
            BookSource::OpenLibrary => score += 2.0, // Good metadata
            BookSource::Google => score += 1.0,      // Commercial but comprehensive
        }

        // Quality bonuses
        if book.cover_url.is_some() {
            score += 0.5;
        }
        if book.description.is_some() {
            score += 0.5;
        }
        if book.isbn.is_some() {
            score += 0.3;
        }

        score
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
                // Try to get Internet Archive identifier
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
}
