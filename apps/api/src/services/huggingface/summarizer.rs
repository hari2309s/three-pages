use crate::{
    services::huggingface::client::HuggingFaceClient,
    utils::{errors::Result, text},
};

const SUMMARIZATION_MODEL: &str = "mistralai/Mistral-7B-Instruct-v0.2";

pub struct SummarizerService {
    client: HuggingFaceClient,
}

impl SummarizerService {
    pub fn new(client: HuggingFaceClient) -> Self {
        Self { client }
    }

    pub async fn summarize(&self, content: &str, language: &str, style: &str) -> Result<String> {
        let chunks = text::chunk_text_with_overlap(content);

        if chunks.is_empty() {
            return Ok(String::new());
        }

        let mut summaries = Vec::new();

        for chunk in chunks.iter().take(5) {
            let prompt = self.build_summary_prompt(chunk, language, style);
            let summary = self
                .client
                .text_generation(SUMMARIZATION_MODEL, &prompt)
                .await?;
            summaries.push(self.clean_summary(&summary));
        }

        if summaries.len() > 1 {
            let combined = summaries.join("\n\n");
            let final_prompt = self.build_final_summary_prompt(&combined, language, style);
            let final_summary = self
                .client
                .text_generation(SUMMARIZATION_MODEL, &final_prompt)
                .await?;
            Ok(self.clean_summary(&final_summary))
        } else {
            Ok(summaries.into_iter().next().unwrap_or_default())
        }
    }

    fn build_summary_prompt(&self, content: &str, language: &str, style: &str) -> String {
        let style_instruction = match style {
            "concise" => "Write a brief, concise summary focusing on the main points.",
            "detailed" => "Write a comprehensive, detailed summary covering all key aspects.",
            "academic" => "Write an academic-style summary with formal language.",
            "simple" => "Write a simple, easy-to-understand summary for general readers.",
            _ => "Write a clear and balanced summary.",
        };

        let lang_instruction = if language != "en" {
            format!(" Write the summary in {}.", Self::language_name(language))
        } else {
            String::new()
        };

        format!(
            r#"<s>[INST] {}{}

Text to summarize:
{}

Provide only the summary, no additional commentary.
[/INST]"#,
            style_instruction, lang_instruction, content
        )
    }

    fn build_final_summary_prompt(&self, summaries: &str, language: &str, style: &str) -> String {
        let lang_instruction = if language != "en" {
            format!(" in {}", Self::language_name(language))
        } else {
            String::new()
        };

        format!(
            r#"<s>[INST] Combine these chapter summaries into one cohesive 3-page summary{}.
Keep the {} style. Focus on the main narrative arc and key themes.

Chapter summaries:
{}

Provide only the final summary, no additional text.
[/INST]"#,
            lang_instruction, style, summaries
        )
    }

    fn clean_summary(&self, summary: &str) -> String {
        summary
            .trim()
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn language_name(code: &str) -> &str {
        match code {
            "es" => "Spanish",
            "fr" => "French",
            "de" => "German",
            "it" => "Italian",
            "pt" => "Portuguese",
            "zh" => "Chinese",
            "ja" => "Japanese",
            "ko" => "Korean",
            "ar" => "Arabic",
            "hi" => "Hindi",
            "ru" => "Russian",
            _ => "English",
        }
    }
}
