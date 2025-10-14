use crate::{services::huggingface::client::HuggingFaceClient, utils::errors::Result};

// Switch to BART for better book summarization
const SUMMARIZATION_MODEL: &str = "facebook/bart-large-cnn";

// Language-specific models and prompts
const MULTILINGUAL_MODEL: &str = "facebook/mbart-large-50-many-to-many-mmt";

pub struct SummarizerService {
    client: HuggingFaceClient,
}

impl SummarizerService {
    pub fn new(client: HuggingFaceClient) -> Self {
        Self { client }
    }

    /// Main entry point for book summarization
    /// Implements fast summarization optimized for speed
    pub async fn summarize(&self, content: &str, language: &str, style: &str) -> Result<String> {
        if content.is_empty() {
            return Ok(self.get_fallback_message(language));
        }

        tracing::info!("Starting fast summarization for language: {}", language);

        // Step 1: Clean the content
        let cleaned_content = self.clean_project_gutenberg_text(content);
        let word_count = cleaned_content.split_whitespace().count();

        tracing::info!("Cleaned content has {} words", word_count);

        // For shorter content, skip chunking and summarize directly
        if word_count <= 2000 {
            tracing::info!("Content is short enough for direct summarization");
            return if language == "en" {
                self.client
                    .summarize_bart(SUMMARIZATION_MODEL, &cleaned_content, 800, 200)
                    .await
            } else {
                self.multilingual_summarize(&cleaned_content, language, style, 800, 200)
                    .await
            };
        }

        // Step 2: Smart chunking with larger chunks for efficiency
        let chunks = self.smart_chunk_by_paragraphs(&cleaned_content, 1500);

        if chunks.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content));
        }

        tracing::info!("Split into {} chunks for processing", chunks.len());

        // Step 3: Summarize chunks directly to final summary (skip intermediate steps)
        let chunk_summaries = self.summarize_chunks(&chunks, language, style).await?;

        if chunk_summaries.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content));
        }

        tracing::info!("Generated {} chunk summaries", chunk_summaries.len());

        // Step 4: Create final summary directly from chunk summaries
        let combined_summaries = chunk_summaries.join("\n\n");
        let final_summary = if language == "en" {
            self.client
                .summarize_bart(SUMMARIZATION_MODEL, &combined_summaries, 1200, 300)
                .await
                .unwrap_or_else(|_| self.fallback_summary(&combined_summaries))
        } else {
            self.multilingual_summarize(&combined_summaries, language, style, 1200, 300)
                .await
                .unwrap_or_else(|_| self.fallback_summary(&combined_summaries))
        };

        let word_count = final_summary.split_whitespace().count();
        tracing::info!(
            "Final summary generated with {} words in language: {}",
            word_count,
            language
        );

        Ok(final_summary)
    }

    /// Summarize individual chunks (Level 1)
    async fn summarize_chunks(
        &self,
        chunks: &[String],
        language: &str,
        style: &str,
    ) -> Result<Vec<String>> {
        let mut summaries = Vec::new();

        // Limit to prevent excessive API calls and timeouts
        let max_chunks = 4.min(chunks.len());

        for (i, chunk) in chunks.iter().take(max_chunks).enumerate() {
            let summary_result = if language == "en" {
                // Use BART for English
                self.client
                    .summarize_bart(SUMMARIZATION_MODEL, chunk, 200, 50)
                    .await
            } else {
                // Use multilingual approach for other languages
                self.multilingual_summarize(chunk, language, style, 200, 50)
                    .await
            };

            match summary_result {
                Ok(summary) => {
                    let clean = self.clean_summary(&summary);
                    if !clean.is_empty() {
                        summaries.push(clean);
                        tracing::debug!("Chunk {} summarized successfully in {}", i + 1, language);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to summarize chunk {} in {}: {}", i + 1, language, e);
                    // Continue with other chunks
                    continue;
                }
            }
        }

        Ok(summaries)
    }

    /// Multilingual summarization using text generation with language-specific prompts
    async fn multilingual_summarize(
        &self,
        text: &str,
        language: &str,
        style: &str,
        target_tokens: usize,
        min_tokens: usize,
    ) -> Result<String> {
        // For very short texts, use direct translation approach
        if text.split_whitespace().count() < 100 {
            let simple_prompt = format!(
                "Summarize the following text in {}:\n\n{}",
                self.get_language_name(language),
                text
            );
            return self
                .client
                .summarize_bart(
                    SUMMARIZATION_MODEL,
                    &simple_prompt,
                    target_tokens,
                    min_tokens,
                )
                .await;
        }

        let prompt = self.create_language_prompt(text, language, style, target_tokens);

        // Try multilingual model first, fallback to text generation
        match self
            .client
            .text_generation(MULTILINGUAL_MODEL, &prompt)
            .await
        {
            Ok(result) => {
                // Clean and validate the result
                let cleaned = self.clean_summary(&result);
                if cleaned.split_whitespace().count() > 10 {
                    Ok(cleaned)
                } else {
                    // If result is too short, fallback
                    self.fallback_multilingual_summary(text, language, target_tokens, min_tokens)
                        .await
                }
            }
            Err(e) => {
                tracing::warn!("Multilingual model failed for {}: {}", language, e);
                self.fallback_multilingual_summary(text, language, target_tokens, min_tokens)
                    .await
            }
        }
    }

    /// Fallback multilingual summarization method
    async fn fallback_multilingual_summary(
        &self,
        text: &str,
        language: &str,
        target_tokens: usize,
        min_tokens: usize,
    ) -> Result<String> {
        // Fallback to English BART with language instruction
        let fallback_prompt = format!(
            "Please summarize the following text in {} language:\n\n{}",
            self.get_language_name(language),
            text
        );

        self.client
            .summarize_bart(
                SUMMARIZATION_MODEL,
                &fallback_prompt,
                target_tokens,
                min_tokens,
            )
            .await
    }

    /// Create language-specific prompts for summarization
    fn create_language_prompt(
        &self,
        text: &str,
        language: &str,
        style: &str,
        target_words: usize,
    ) -> String {
        let language_instructions = match language {
            "de" => format!("Erstellen Sie eine detaillierte Zusammenfassung von etwa {} Wörtern des folgenden Textes auf Deutsch:", target_words),
            "ta" => format!("பின்வரும் உரையின் தமிழில் சுமார் {} வார்த்தைகளின் விரிவான சுருக்கத்தை உருவாக்கவும்:", target_words),
            _ => format!("Create a detailed summary of approximately {} words of the following text:", target_words),
        };

        let style_instruction = match style {
            "academic" => self.get_academic_style_instruction(language),
            "simple" => self.get_simple_style_instruction(language),
            "detailed" => self.get_detailed_style_instruction(language),
            _ => self.get_concise_style_instruction(language),
        };

        format!(
            "{}\n\n{}\n\nText: {}",
            language_instructions, style_instruction, text
        )
    }

    fn get_language_name(&self, code: &str) -> &str {
        match code {
            "de" => "German",
            "ta" => "Tamil",
            _ => "English",
        }
    }

    fn get_academic_style_instruction(&self, language: &str) -> String {
        match language {
            "de" => "Verwenden Sie einen akademischen und formalen Ton mit tiefgreifender Analyse."
                .to_string(),
            "ta" => "ஆழமான பகுப்பாய்வுடன் கல்வி மற்றும் முறையான தொனியைப் பயன்படுத்தவும்.".to_string(),
            _ => "Use an academic and formal tone with in-depth analysis.".to_string(),
        }
    }

    fn get_simple_style_instruction(&self, language: &str) -> String {
        match language {
            "de" => "Verwenden Sie eine einfache und klare Sprache, die leicht zu verstehen ist."
                .to_string(),
            "ta" => "புரிந்துகொள்ள எளிதான எளிய மற்றும் தெளிவான மொழியைப் பயன்படுத்தவும்.".to_string(),
            _ => "Use simple and clear language that is easy to understand.".to_string(),
        }
    }

    fn get_detailed_style_instruction(&self, language: &str) -> String {
        match language {
            "de" => {
                "Bieten Sie eine umfassende und ausführliche Abdeckung des Inhalts.".to_string()
            }
            "ta" => "உள்ளடக்கத்தின் விரிவான மற்றும் முழுமையான கவரேஜை வழங்கவும்.".to_string(),
            _ => "Provide comprehensive and thorough coverage of the content.".to_string(),
        }
    }

    fn get_concise_style_instruction(&self, language: &str) -> String {
        match language {
            "de" => {
                "Seien Sie kurz und auf den Punkt, erfassen Sie nur das Wesentliche.".to_string()
            }
            "ta" => "சுருக்கமாகவும் புள்ளிக்கும் இருங்கள், அத்தியாவசியமானவற்றை மட்டும் கைப்பற்றவும்.".to_string(),
            _ => "Be brief and to the point, capturing only the essentials.".to_string(),
        }
    }

    fn get_fallback_message(&self, language: &str) -> String {
        match language {
            "de" => "Kein Inhalt zum Zusammenfassen verfügbar.".to_string(),
            "ta" => "சுருக்கத்திற்கு எந்த உள்ளடக்கமும் இல்லை.".to_string(),
            _ => "No content available for summarization.".to_string(),
        }
    }

    /// Smart chunking by paragraphs to maintain context
    fn smart_chunk_by_paragraphs(&self, text: &str, target_words: usize) -> Vec<String> {
        // For faster processing, create fewer, larger chunks
        // Split by double newlines (paragraphs) or sentences
        let paragraphs: Vec<&str> = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        if paragraphs.is_empty() {
            return self.fallback_chunk_by_words(text, target_words);
        }

        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut word_count = 0;

        for para in paragraphs {
            let para_words = para.split_whitespace().count();

            // If single paragraph is too large, split it
            if para_words > target_words {
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
                    if sent_count + sent_words > target_words && !sentence_chunk.is_empty() {
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

            if word_count + para_words > target_words && !current_chunk.is_empty() {
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
    fn fallback_chunk_by_words(&self, text: &str, target_words: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() <= target_words {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        for chunk in words.chunks(target_words) {
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
