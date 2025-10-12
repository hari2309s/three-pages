use crate::{
    services::huggingface::client::HuggingFaceClient,
    utils::{errors::Result, text},
};

const TTS_MODEL: &str = "facebook/mms-tts-eng";

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
                    "Failed to generate audio with {}: {}. Trying fallback...",
                    primary_model,
                    e
                );

                // Fallback to English model if language-specific model fails
                if primary_model != TTS_MODEL {
                    match self.generate_with_retry(TTS_MODEL, &cleaned_text, 2).await {
                        Ok(audio_data) => {
                            tracing::info!(
                                "Successfully generated audio with fallback English model"
                            );
                            Ok(audio_data)
                        }
                        Err(fallback_e) => {
                            tracing::error!(
                                "Both primary and fallback TTS failed: {} | {}",
                                e,
                                fallback_e
                            );
                            Err(crate::utils::errors::AppError::ExternalApi(format!(
                                "TTS generation failed for both {} and English models",
                                language
                            )))
                        }
                    }
                } else {
                    tracing::error!("English TTS model failed: {}", e);
                    Err(e)
                }
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
        text
            // Remove markdown and formatting
            .replace("**", "")
            .replace("*", "")
            .replace("_", "")
            .replace("#", "")
            // Replace common abbreviations that might cause issues
            .replace("&", "and")
            .replace("e.g.", "for example")
            .replace("i.e.", "that is")
            .replace("etc.", "and so on")
            // Remove extra whitespace
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            // Limit to reasonable length for TTS
            .chars()
            .take(1000)
            .collect()
    }

    fn get_tts_model(&self, language: &str) -> &str {
        match language {
            "de" => "facebook/mms-tts-deu",
            "ta" => "facebook/mms-tts-tam",
            _ => TTS_MODEL, // English (en) and fallback
        }
    }
}
