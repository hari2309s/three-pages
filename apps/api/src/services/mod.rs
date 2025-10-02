// services/mod.rs

// Declare and re-export sub-modules under services
pub mod books;
pub mod cache;
pub mod huggingface;
pub mod storage;

// Re-export specific items for convenience
pub use books::GoogleBooksService;
pub use cache::CacheService;
pub use huggingface::{HuggingFaceClient, NLPService, SummarizerService, TTSService};
pub use storage::DatabaseService;
