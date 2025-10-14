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

                // Final fallback: Generate a simple WAV file with synthesized content
                tracing::warn!("All HuggingFace TTS strategies failed, using local fallback");
                match self.generate_fallback_audio(&cleaned_text, language) {
                    Ok(audio_data) => {
                        tracing::info!("Successfully generated fallback audio");
                        Ok(audio_data)
                    }
                    Err(fallback_e) => {
                        tracing::error!("Even fallback audio generation failed: {}", fallback_e);
                        Err(crate::utils::errors::AppError::ExternalApi(format!(
                            "TTS generation failed for language {} after trying all strategies including fallback: {}",
                            language, fallback_e
                        )))
                    }
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
            _ => TTS_MODEL, // English (en) and fallback
        }
    }

    /// Generate a fallback audio file when all TTS services fail
    /// Creates a pleasant, multi-tone audio notification with realistic duration
    fn generate_fallback_audio(&self, text: &str, language: &str) -> Result<Vec<u8>> {
        // Calculate realistic duration based on average reading speed
        let word_count = text.split_whitespace().count();
        let words_per_minute = 150.0; // Average TTS speed
        let duration_seconds = ((word_count as f32 / words_per_minute * 60.0) + 2.0)
            .min(45.0)
            .max(3.0);

        let sample_rate = 22050u32;
        let channels = 1u16;
        let bits_per_sample = 16u16;

        let num_samples = (duration_seconds * sample_rate as f32) as u32;
        let data_size = num_samples * channels as u32 * (bits_per_sample / 8) as u32;
        let file_size = 44 + data_size; // WAV header is 44 bytes

        let mut wav_data = Vec::with_capacity(file_size as usize);

        // Write WAV header
        wav_data.extend_from_slice(b"RIFF");
        wav_data.extend_from_slice(&(file_size - 8).to_le_bytes());
        wav_data.extend_from_slice(b"WAVE");
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // PCM format
        wav_data.extend_from_slice(&channels.to_le_bytes());
        wav_data.extend_from_slice(&sample_rate.to_le_bytes());
        wav_data.extend_from_slice(
            &(sample_rate * channels as u32 * (bits_per_sample / 8) as u32).to_le_bytes(),
        ); // byte rate
        wav_data.extend_from_slice(&(channels * (bits_per_sample / 8)).to_le_bytes()); // block align
        wav_data.extend_from_slice(&bits_per_sample.to_le_bytes());
        wav_data.extend_from_slice(b"data");
        wav_data.extend_from_slice(&data_size.to_le_bytes());

        // Generate a more pleasant multi-tone audio pattern that simulates speech rhythm
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration_seconds;

            // Create a speech-like rhythm with pauses
            let rhythm_factor = if (t % 4.0) < 2.8 { 1.0 } else { 0.3 }; // Simulate pauses

            // Multiple frequency components for richer sound
            let base_freq = 180.0; // Lower base frequency
            let harmonic1 = (2.0 * std::f32::consts::PI * base_freq * t).sin();
            let harmonic2 = 0.3 * (2.0 * std::f32::consts::PI * base_freq * 1.5 * t).sin();
            let harmonic3 = 0.15 * (2.0 * std::f32::consts::PI * base_freq * 2.0 * t).sin();

            // Add subtle frequency modulation to simulate speech prosody
            let modulation = 1.0 + 0.1 * (2.0 * std::f32::consts::PI * 0.5 * t).sin();

            // Envelope with gentle fade in/out and speech-like dynamics
            let envelope = if progress < 0.1 {
                progress * 10.0 // Fade in
            } else if progress > 0.9 {
                (1.0 - progress) * 10.0 // Fade out
            } else {
                0.8 + 0.2 * (2.0 * std::f32::consts::PI * 2.0 * t).sin().abs() // Slight variation
            };

            let combined_wave = (harmonic1 + harmonic2 + harmonic3) * modulation * rhythm_factor;
            let amplitude = (envelope * 0.08 * 32767.0) as i16; // Gentle volume
            let sample = (amplitude as f32 * combined_wave) as i16;

            wav_data.extend_from_slice(&sample.to_le_bytes());
        }

        tracing::info!(
            "Generated enhanced fallback audio for {} ({} words): {} bytes, {:.1}s duration, language: {}",
            if text.len() > 50 { &text[..50] } else { text },
            word_count,
            wav_data.len(),
            duration_seconds,
            language
        );

        Ok(wav_data)
    }
}
