mod aggregator;
mod google_books;
mod gutenberg;
mod open_library;

pub use aggregator::BookAggregatorService;
pub use google_books::GoogleBooksService;
pub use gutenberg::GutenbergService;
pub use open_library::OpenLibraryService;
