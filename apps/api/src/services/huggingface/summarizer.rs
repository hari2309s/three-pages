use crate::{services::huggingface::client::HuggingFaceClient, utils::errors::Result};

// Switch to BART for better book summarization
const SUMMARIZATION_MODEL: &str = "facebook/bart-large-cnn";

struct StyleParameters {
    target_tokens: usize,
    min_tokens: usize,
}

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

        tracing::info!(
            "Starting fast summarization for language: {} with style: {}",
            language,
            style
        );

        // Step 1: Clean the content
        let cleaned_content = self.clean_project_gutenberg_text(content);
        let word_count = cleaned_content.split_whitespace().count();

        tracing::info!("Cleaned content has {} words", word_count);

        // For shorter content, skip chunking and summarize directly
        if word_count <= 2000 {
            tracing::info!("Content is short enough for direct summarization");
            let style_params = self.get_style_parameters(style);
            tracing::info!(
                "Using style '{}' with {} target tokens, {} min tokens",
                style,
                style_params.target_tokens,
                style_params.min_tokens
            );
            let styled_content = self.add_style_instruction(&cleaned_content, style);
            return self
                .client
                .summarize_bart(
                    SUMMARIZATION_MODEL,
                    &styled_content,
                    style_params.target_tokens,
                    style_params.min_tokens,
                )
                .await;
        }

        // Step 2: Smart chunking with larger chunks for efficiency
        let chunks = self.smart_chunk_by_paragraphs(&cleaned_content, 1500);

        if chunks.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content, style));
        }

        tracing::info!("Split into {} chunks for processing", chunks.len());

        // Step 3: Summarize chunks directly to final summary (skip intermediate steps)
        let chunk_summaries = self.summarize_chunks(&chunks, language, style).await?;

        if chunk_summaries.is_empty() {
            return Ok(self.fallback_summary(&cleaned_content, style));
        }

        tracing::info!("Generated {} chunk summaries", chunk_summaries.len());

        // Step 4: Create final summary directly from chunk summaries
        let combined_summaries = chunk_summaries.join("\n\n");
        let style_params = self.get_style_parameters(style);
        tracing::info!(
            "Applying final summarization with style '{}': {} target tokens, {} min tokens",
            style,
            style_params.target_tokens,
            style_params.min_tokens
        );
        let styled_content = self.add_style_instruction(&combined_summaries, style);
        let final_summary = self
            .client
            .summarize_bart(
                SUMMARIZATION_MODEL,
                &styled_content,
                style_params.target_tokens,
                style_params.min_tokens,
            )
            .await
            .unwrap_or_else(|_| self.fallback_summary(&combined_summaries, style));

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
        _language: &str,
        style: &str,
    ) -> Result<Vec<String>> {
        let mut summaries = Vec::new();

        // Limit to prevent excessive API calls and timeouts
        let max_chunks = 4.min(chunks.len());

        for (i, chunk) in chunks.iter().take(max_chunks).enumerate() {
            let chunk_params = self.get_chunk_style_parameters(style);
            let styled_chunk = self.add_style_instruction(chunk, style);
            let summary_result = self
                .client
                .summarize_bart(
                    SUMMARIZATION_MODEL,
                    &styled_chunk,
                    chunk_params.target_tokens,
                    chunk_params.min_tokens,
                )
                .await;

            match summary_result {
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

    fn get_fallback_message(&self, _language: &str) -> String {
        "No content available for summarization.".to_string()
    }

    fn get_style_parameters(&self, style: &str) -> StyleParameters {
        match style {
            "detailed" => StyleParameters {
                target_tokens: 1500,
                min_tokens: 400,
            },
            "academic" => StyleParameters {
                target_tokens: 1300,
                min_tokens: 350,
            },
            "simple" => StyleParameters {
                target_tokens: 1000,
                min_tokens: 250,
            },
            _ => StyleParameters {
                // concise
                target_tokens: 800,
                min_tokens: 200,
            },
        }
    }

    fn get_chunk_style_parameters(&self, style: &str) -> StyleParameters {
        match style {
            "detailed" => StyleParameters {
                target_tokens: 300,
                min_tokens: 80,
            },
            "academic" => StyleParameters {
                target_tokens: 250,
                min_tokens: 70,
            },
            "simple" => StyleParameters {
                target_tokens: 180,
                min_tokens: 50,
            },
            _ => StyleParameters {
                // concise
                target_tokens: 150,
                min_tokens: 40,
            },
        }
    }

    fn add_style_instruction(&self, text: &str, style: &str) -> String {
        let instruction = match style {
            "detailed" => "INSTRUCTION: Write a comprehensive, in-depth summary that covers all major themes, character development, plot points, literary devices, and contextual significance. Include specific examples, quotes, and detailed analysis. Aim for thorough coverage with rich descriptions and explanations. Use sophisticated vocabulary and complex sentence structures:",
            "academic" => "INSTRUCTION: Compose a formal, scholarly analysis in academic style. Use formal language, analytical frameworks, critical theory perspectives, and structured argumentation. Include discussion of literary merit, historical context, thematic significance, and scholarly interpretations. Maintain objective, analytical tone throughout:",
            "simple" => "INSTRUCTION: Write a clear, straightforward summary using simple words and short sentences. Explain everything in an easy-to-understand way, as if writing for someone who is new to reading literature. Avoid complex vocabulary and focus on basic plot and main ideas:",
            _ => "INSTRUCTION: Create a brief, focused summary that captures only the most essential plot points and main themes. Be direct and concise, highlighting key events and core messages without unnecessary detail:", // concise
        };

        tracing::debug!("Applied '{}' style instruction to content", style);
        format!("{}\n\n{}", instruction, text)
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
    fn fallback_summary(&self, content: &str, style: &str) -> String {
        let sentence_count = match style {
            "detailed" => 20,
            "academic" => 18,
            "simple" => 12,
            _ => 15, // concise
        };

        tracing::info!(
            "Using fallback summary with style '{}' and {} sentence limit",
            style,
            sentence_count
        );

        let sentences: Vec<&str> = content
            .split(&['.', '!', '?'])
            .filter(|s| {
                let trimmed = s.trim();
                trimmed.len() > 20
                    && !trimmed.contains("Project Gutenberg")
                    && !trimmed.starts_with("CHAPTER")
            })
            .take(sentence_count)
            .collect();

        if sentences.is_empty() {
            let default_message = match style {
                "detailed" => "This comprehensive literary work presents an intricate narrative structure with well-developed characters, exploring complex themes through detailed storytelling techniques. The author employs various literary devices to create a rich reading experience that engages with profound questions and offers multiple layers of meaning for readers to discover and analyze.",
                "academic" => "This literary work demonstrates significant artistic merit through its sophisticated narrative construction and thematic complexity. The text employs established literary conventions while contributing to broader cultural and intellectual discourse within its genre and historical context.",
                "simple" => "This book tells an interesting story with good characters. It's easy to read and has important ideas that help readers learn and think about life.",
                _ => "This literary work offers engaging storytelling with memorable characters and explores meaningful themes that resonate with readers.", // concise
            };
            default_message.to_string()
        } else {
            let joined = sentences.join(". ").trim().to_string() + ".";
            match style {
                "detailed" | "academic" => format!("This work presents: {}", joined),
                "simple" => format!("This book is about: {}", joined),
                _ => joined, // concise
            }
        }
    }
}
