pub mod client;
pub mod nlp;
pub mod summarizer;
pub mod tts;

pub use client::HuggingFaceClient;
pub use nlp::NLPService;
pub use summarizer::SummarizerService;
pub use tts::TTSService;
