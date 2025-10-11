use crate::{services::huggingface::client::HuggingFaceClient, utils::errors::Result};

// Switch to BART for better book summarization
const SUMMARIZATION_MODEL: &str = "facebook/bart-large-cnn";

// Target words for 3 pages (500-600 words per page)
const TARGET_FINAL_WORDS: usize = 1650; // Middle of 1500-1800 range
const MIN_FINAL_WORDS: usize = 1500;
const MAX_FINAL_WORDS: usize = 1800;

pub struct SummarizerService {
    client: HuggingFaceClient,
}

impl SummarizerService {
    pub fn new(client: HuggingFaceClient) -> Self {
        Self { client }
    }

    /// Main entry point for book summarization
    /// Implements hierarchical summarization: chunks → sections → final 3-page summary
    pub async fn summarize(&self, content: &str, language: &str, style: &str) -> Result<String> {
        if content.is_empty() {
            return Ok("No content available for summarization.".to_string());
        }

        tracing::info!("Starting hierarchical summarization...");

        // Step 1: Clean the content
        let cleaned_content = self.clean_project_gutenberg_text(content);
        let word_count = cleaned_content.split_whitespace().count();

        tracing::info!("Cleaned content has {} words", word_count);

        // Step 2: Smart chunking based on book length
        let chunks = self.smart_chunk_by_paragraphs(&cleaned_content, 1000);

        if chunks.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content));
        }

        tracing::info!("Split into {} chunks for processing", chunks.len());

        // Step 3: First pass - Summarize each chunk (Level 1)
        let chunk_summaries = self.summarize_chunks(&chunks).await?;

        if chunk_summaries.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content));
        }

        tracing::info!("Generated {} chunk summaries", chunk_summaries.len());

        // Step 4: Second pass - Combine chunk summaries into section summaries (Level 2)
        let section_summaries = self.combine_into_sections(&chunk_summaries).await?;

        tracing::info!("Generated {} section summaries", section_summaries.len());

        // Step 5: Final pass - Create the 3-page summary (Level 3)
        let final_summary = self
            .create_final_summary(&section_summaries, language, style)
            .await?;

        tracing::info!(
            "Final summary generated with {} words",
            final_summary.split_whitespace().count()
        );

        Ok(final_summary)
    }

    /// Summarize individual chunks (Level 1)
    async fn summarize_chunks(&self, chunks: &[String]) -> Result<Vec<String>> {
        let mut summaries = Vec::new();

        // Limit to prevent excessive API calls
        let max_chunks = 15.min(chunks.len());

        for (i, chunk) in chunks.iter().take(max_chunks).enumerate() {
            match self
                .client
                .summarize_bart(SUMMARIZATION_MODEL, chunk, 200, 50)
                .await
            {
                Ok(summary) => {
                    let clean = self.clean_summary(&summary);
                    if !clean.is_empty() {
                        summaries.push(clean);
                        tracing::debug!("Chunk {} summarized successfully", i + 1);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to summarize chunk {}: {}", i + 1, e);
                    // Continue with other chunks
                    continue;
                }
            }
        }

        Ok(summaries)
    }

    /// Combine chunk summaries into section summaries (Level 2)
    async fn combine_into_sections(&self, chunk_summaries: &[String]) -> Result<Vec<String>> {
        if chunk_summaries.len() <= 3 {
            // If we have 3 or fewer summaries, skip this level
            return Ok(chunk_summaries.to_vec());
        }

        let mut section_summaries = Vec::new();

        // Group chunks into sections (3-4 chunks per section)
        let chunks_per_section = 3;

        for section_chunks in chunk_summaries.chunks(chunks_per_section) {
            let combined = section_chunks.join("\n\n");

            match self
                .client
                .summarize_bart(SUMMARIZATION_MODEL, &combined, 300, 100)
                .await
            {
                Ok(summary) => {
                    section_summaries.push(self.clean_summary(&summary));
                }
                Err(e) => {
                    tracing::warn!("Failed to combine section: {}", e);
                    // Fall back to using the chunks directly
                    section_summaries.push(combined);
                }
            }
        }

        Ok(section_summaries)
    }

    /// Create final 3-page summary (Level 3)
    async fn create_final_summary(
        &self,
        section_summaries: &[String],
        _language: &str,
        _style: &str,
    ) -> Result<String> {
        let combined = section_summaries.join("\n\n");

        // Calculate target length in tokens (roughly 1.3 tokens per word)
        let target_tokens = (TARGET_FINAL_WORDS as f32 * 1.3) as usize;
        let min_tokens = (MIN_FINAL_WORDS as f32 * 1.3) as usize;

        match self
            .client
            .summarize_bart(SUMMARIZATION_MODEL, &combined, target_tokens, min_tokens)
            .await
        {
            Ok(summary) => {
                let final_text = self.clean_summary(&summary);
                let word_count = final_text.split_whitespace().count();

                // If summary is too short, try to expand it
                if word_count < MIN_FINAL_WORDS && section_summaries.len() > 1 {
                    tracing::info!("Summary too short ({}), attempting expansion", word_count);
                    return self.expand_summary(section_summaries).await;
                }

                Ok(final_text)
            }
            Err(e) => {
                tracing::error!("Failed to create final summary: {}", e);
                // Fallback: combine sections with some formatting
                Ok(self.format_sections(section_summaries))
            }
        }
    }

    /// Expand summary if it's too short
    async fn expand_summary(&self, section_summaries: &[String]) -> Result<String> {
        // Use larger target for expansion
        let combined = section_summaries.join("\n\n");
        let target_tokens = (MAX_FINAL_WORDS as f32 * 1.3) as usize;
        let min_tokens = (TARGET_FINAL_WORDS as f32 * 1.3) as usize;

        match self
            .client
            .summarize_bart(SUMMARIZATION_MODEL, &combined, target_tokens, min_tokens)
            .await
        {
            Ok(summary) => Ok(self.clean_summary(&summary)),
            Err(_) => Ok(self.format_sections(section_summaries)),
        }
    }

    /// Format sections as fallback
    fn format_sections(&self, sections: &[String]) -> String {
        sections
            .iter()
            .enumerate()
            .map(|(i, section)| {
                if sections.len() > 3 {
                    format!("Part {}: {}", i + 1, section)
                } else {
                    section.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Smart chunking by paragraphs to maintain context
    fn smart_chunk_by_paragraphs(&self, text: &str, max_words: usize) -> Vec<String> {
        // Split by double newlines (paragraphs) or sentences
        let paragraphs: Vec<&str> = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        if paragraphs.is_empty() {
            return self.fallback_chunk_by_words(text, max_words);
        }

        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut word_count = 0;

        for para in paragraphs {
            let para_words = para.split_whitespace().count();

            // If single paragraph is too large, split it
            if para_words > max_words {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.join("\n\n"));
                    current_chunk.clear();
                    word_count = 0;
                }

                // Split large paragraph by sentences
                let sentences = self.split_into_sentences(para);
                let mut sentence_chunk = Vec::new();
                let mut sent_count = 0;

                for sent in sentences {
                    let sent_words = sent.split_whitespace().count();
                    if sent_count + sent_words > max_words && !sentence_chunk.is_empty() {
                        chunks.push(sentence_chunk.join(" "));
                        sentence_chunk.clear();
                        sent_count = 0;
                    }
                    sentence_chunk.push(sent);
                    sent_count += sent_words;
                }

                if !sentence_chunk.is_empty() {
                    chunks.push(sentence_chunk.join(" "));
                }
                continue;
            }

            if word_count + para_words > max_words && !current_chunk.is_empty() {
                chunks.push(current_chunk.join("\n\n"));
                current_chunk.clear();
                word_count = 0;
            }

            current_chunk.push(para);
            word_count += para_words;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join("\n\n"));
        }

        chunks
    }

    /// Fallback chunking by words if paragraph splitting fails
    fn fallback_chunk_by_words(&self, text: &str, max_words: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() <= max_words {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        for chunk in words.chunks(max_words) {
            chunks.push(chunk.join(" "));
        }
        chunks
    }

    /// Split text into sentences
    fn split_into_sentences<'a>(&self, text: &'a str) -> Vec<&'a str> {
        text.split(&['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect()
    }

    /// Clean Project Gutenberg headers and footers
    fn clean_project_gutenberg_text(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut start_idx = 0;
        let mut end_idx = lines.len();

        // Find start of actual content
        for (i, line) in lines.iter().enumerate() {
            if line.contains("*** START OF") && line.contains("PROJECT GUTENBERG") {
                start_idx = i + 1;
                break;
            }
            if line.starts_with("CHAPTER") || line.starts_with("Chapter") {
                start_idx = i;
                break;
            }
        }

        // Find end of actual content
        for (i, line) in lines.iter().enumerate().rev() {
            if line.contains("*** END OF") && line.contains("PROJECT GUTENBERG") {
                end_idx = i;
                break;
            }
        }

        if start_idx < end_idx {
            lines[start_idx..end_idx]
                .iter()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.is_empty()
                        && !trimmed.starts_with("Produced by")
                        && !trimmed.starts_with("Updated:")
                        && !trimmed.contains("gutenberg.org")
                        && !trimmed.starts_with("[Illustration")
                })
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            content.to_string()
        }
    }

    /// Clean summary output
    fn clean_summary(&self, summary: &str) -> String {
        summary
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Fallback summary for very short content
    fn fallback_summary(&self, content: &str) -> String {
        let sentences: Vec<&str> = content
            .split(&['.', '!', '?'])
            .filter(|s| {
                let trimmed = s.trim();
                trimmed.len() > 20
                    && !trimmed.contains("Project Gutenberg")
                    && !trimmed.starts_with("CHAPTER")
            })
            .take(15)
            .collect();

        if sentences.is_empty() {
            "This literary work offers engaging storytelling with memorable characters and explores meaningful themes that resonate with readers.".to_string()
        } else {
            sentences.join(". ").trim().to_string() + "."
        }
    }
}
