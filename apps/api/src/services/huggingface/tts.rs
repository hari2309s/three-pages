use crate::{
    services::huggingface::client::HuggingFaceClient,
    utils::{errors::Result, text},
};

const TTS_MODEL: &str = "microsoft/speecht5_tts";
const BACKUP_TTS_MODEL: &str = "facebook/mms-tts-eng";

pub struct TTSService {
    client: HuggingFaceClient,
}

impl TTSService {
    pub fn new(client: HuggingFaceClient) -> Self {
        Self { client }
    }

    pub async fn generate_audio(&self, text: &str, language: &str) -> Result<Vec<u8>> {
        let truncated = text::truncate_text(text, 500);
        let cleaned_text = self.clean_text_for_tts(&truncated);

        if cleaned_text.trim().is_empty() {
            return Err(crate::utils::errors::AppError::InvalidInput(
                "No valid text for audio generation".to_string(),
            ));
        }

        tracing::info!(
            "Generating audio for language: {} with {} characters",
            language,
            cleaned_text.len()
        );

        // Try language-specific model first
        let primary_model = self.get_tts_model(language);

        // First attempt with primary model
        match self
            .generate_with_retry(primary_model, &cleaned_text, 2)
            .await
        {
            Ok(audio_data) => {
                tracing::info!("Successfully generated audio with {} model", primary_model);
                Ok(audio_data)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to generate audio with {}: {}. Trying fallbacks...",
                    primary_model,
                    e
                );

                // Fallback 1: Try main TTS model if not already used
                if primary_model != TTS_MODEL {
                    match self.generate_with_retry(TTS_MODEL, &cleaned_text, 2).await {
                        Ok(audio_data) => {
                            tracing::info!("Successfully generated audio with main TTS model");
                            return Ok(audio_data);
                        }
                        Err(fallback_e) => {
                            tracing::warn!("Main TTS model also failed: {}", fallback_e);
                        }
                    }
                }

                // Fallback 2: Try backup model
                if primary_model != BACKUP_TTS_MODEL && TTS_MODEL != BACKUP_TTS_MODEL {
                    match self
                        .generate_with_retry(BACKUP_TTS_MODEL, &cleaned_text, 2)
                        .await
                    {
                        Ok(audio_data) => {
                            tracing::info!("Successfully generated audio with backup model");
                            return Ok(audio_data);
                        }
                        Err(backup_e) => {
                            tracing::warn!("Backup TTS model failed: {}", backup_e);
                        }
                    }
                }

                // Fallback 3: Try with shortened text
                let short_text =
                    self.clean_text_for_tts(&cleaned_text[..cleaned_text.len().min(200)]);
                if !short_text.is_empty() && short_text != cleaned_text {
                    tracing::info!(
                        "Attempting TTS with shortened text ({} chars)",
                        short_text.len()
                    );
                    match self.generate_with_retry(TTS_MODEL, &short_text, 1).await {
                        Ok(audio_data) => {
                            tracing::info!("Successfully generated audio with shortened text");
                            return Ok(audio_data);
                        }
                        Err(short_e) => {
                            tracing::warn!("Shortened text TTS failed: {}", short_e);
                        }
                    }
                }

                tracing::error!("All TTS fallback strategies failed");
                Err(crate::utils::errors::AppError::ExternalApi(format!(
                    "TTS generation failed for language {} after trying multiple models and strategies",
                    language
                )))
            }
        }
    }

    async fn generate_with_retry(
        &self,
        model: &str,
        text: &str,
        max_retries: u32,
    ) -> Result<Vec<u8>> {
        let mut last_error = None;

        for attempt in 1..=max_retries {
            tracing::debug!("TTS attempt {} for model: {}", attempt, model);

            match self.client.tts(model, text).await {
                Ok(data) => {
                    if data.is_empty() {
                        let error = crate::utils::errors::AppError::ExternalApi(
                            "TTS returned empty audio data".to_string(),
                        );
                        last_error = Some(error);
                        continue;
                    }
                    return Ok(data);
                }
                Err(e) => {
                    tracing::warn!("TTS attempt {} failed: {}", attempt, e);
                    last_error = Some(e);

                    if attempt < max_retries {
                        // Wait before retry (exponential backoff)
                        let delay = std::time::Duration::from_millis(1000 * attempt as u64);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            crate::utils::errors::AppError::ExternalApi("Unknown TTS error".to_string())
        }))
    }

    fn clean_text_for_tts(&self, text: &str) -> String {
        let cleaned = text
            // Remove markdown and formatting
            .replace("**", "")
            .replace("*", "")
            .replace("_", "")
            .replace("#", "")
            .replace("`", "")
            .replace("~", "")
            // Replace problematic characters
            .replace("&", "and")
            .replace("@", "at")
            .replace("%", "percent")
            .replace("$", "dollars")
            .replace("€", "euros")
            .replace("£", "pounds")
            // Replace common abbreviations that might cause issues
            .replace("e.g.", "for example")
            .replace("i.e.", "that is")
            .replace("etc.", "and so on")
            .replace("vs.", "versus")
            .replace("Mr.", "Mister")
            .replace("Mrs.", "Missus")
            .replace("Dr.", "Doctor")
            // Remove URLs and email patterns
            .split_whitespace()
            .filter(|word| !word.contains("http") && !word.contains("@"))
            .collect::<Vec<_>>()
            .join(" ")
            // Remove special characters that might cause TTS issues
            .chars()
            .filter(|c| c.is_alphanumeric() || " .,!?;:'-".contains(*c))
            .collect::<String>()
            // Clean up extra whitespace and punctuation
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            // Limit to reasonable length for TTS
            .chars()
            .take(1000)
            .collect::<String>();

        // Ensure we don't end mid-sentence
        if cleaned.len() >= 1000 {
            if let Some(last_sentence_end) = cleaned.rfind(&['.', '!', '?'][..]) {
                if last_sentence_end > 500 {
                    // Only truncate if we have at least 500 chars
                    return cleaned[..=last_sentence_end].to_string();
                }
            }
        }

        cleaned
    }

    fn get_tts_model(&self, language: &str) -> &str {
        match language {
            "de" => "facebook/mms-tts-deu",
            "ta" => "facebook/mms-tts-tam",
            "es" => "facebook/mms-tts-spa",
            "fr" => "facebook/mms-tts-fra",
            _ => TTS_MODEL, // English (en) and fallback
        }
    }
}
