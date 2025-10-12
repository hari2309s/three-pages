use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::errors::{AppError, Result};

#[derive(Clone)]
pub struct HuggingFaceClient {
    client: Client,
    api_base: String,
    token: String,
}

impl HuggingFaceClient {
    pub fn new(client: Client, api_base: String, token: String) -> Self {
        Self {
            client,
            api_base,
            token,
        }
    }

    pub async fn inference(
        &self,
        model: &str,
        inputs: &str,
        parameters: Option<serde_json::Value>,
    ) -> Result<String> {
        self.inference_with_retry(model, inputs, parameters, 3)
            .await
    }

    async fn inference_with_retry(
        &self,
        model: &str,
        inputs: &str,
        parameters: Option<serde_json::Value>,
        max_retries: u32,
    ) -> Result<String> {
        let url = format!("{}/models/{}", self.api_base, model);

        let mut payload = json!({
            "inputs": inputs,
        });

        if let Some(params) = parameters {
            payload["parameters"] = params;
        }

        let mut last_error = None;

        for attempt in 1..=max_retries {
            tracing::debug!("HF inference attempt {} for model: {}", attempt, model);

            match self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .json(&payload)
                .timeout(std::time::Duration::from_secs(60))
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        match response.json::<Vec<InferenceResponse>>().await {
                            Ok(result) => {
                                if let Some(first_result) = result.first() {
                                    if let Some(text) = &first_result.generated_text {
                                        let cleaned_text = if text.starts_with(inputs) {
                                            text[inputs.len()..].trim().to_string()
                                        } else {
                                            text.clone()
                                        };

                                        if !cleaned_text.trim().is_empty() {
                                            return Ok(cleaned_text);
                                        }
                                    }
                                }

                                let error = AppError::ExternalApi(
                                    "HuggingFace returned empty or invalid response".to_string(),
                                );
                                last_error = Some(error);
                            }
                            Err(e) => {
                                let error = AppError::ExternalApi(format!(
                                    "Failed to parse HuggingFace response: {}",
                                    e
                                ));
                                last_error = Some(error);
                            }
                        }
                    } else {
                        let error_text = response.text().await.unwrap_or_default();
                        let error = AppError::ExternalApi(format!(
                            "HuggingFace API error {}: {}",
                            status, error_text
                        ));
                        last_error = Some(error);

                        // Don't retry on authentication errors
                        if status == 401 || status == 403 {
                            return Err(last_error.unwrap());
                        }
                    }
                }
                Err(e) => {
                    let error = AppError::ExternalApi(format!(
                        "Failed to connect to HuggingFace API: {}",
                        e
                    ));
                    last_error = Some(error);
                }
            }

            if attempt < max_retries {
                let delay = std::time::Duration::from_millis(1000 * attempt as u64);
                tracing::warn!(
                    "HF inference attempt {} failed, retrying in {:?}",
                    attempt,
                    delay
                );
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error
            .unwrap_or_else(|| AppError::ExternalApi("Unknown HuggingFace API error".to_string())))
    }

    pub async fn text_generation(&self, model: &str, prompt: &str) -> Result<String> {
        let parameters = json!({
            "max_new_tokens": 1000,
            "temperature": 0.3,
            "top_p": 0.9,
            "do_sample": true,
            "repetition_penalty": 1.2,
            "length_penalty": 1.0
        });

        match self.inference(model, prompt, Some(parameters)).await {
            Ok(result) => {
                if result.trim().is_empty() {
                    Err(AppError::ExternalApi(
                        "Text generation returned empty result".to_string(),
                    ))
                } else {
                    Ok(result)
                }
            }
            Err(e) => {
                tracing::warn!("Primary text generation failed: {}", e);

                // Fallback with simpler parameters
                let simple_parameters = json!({
                    "max_new_tokens": 500,
                    "temperature": 0.1,
                    "do_sample": false
                });

                self.inference(model, prompt, Some(simple_parameters)).await
            }
        }
    }

    pub async fn tts(&self, model: &str, text: &str) -> Result<Vec<u8>> {
        self.tts_with_retry(model, text, 3).await
    }

    async fn tts_with_retry(&self, model: &str, text: &str, max_retries: u32) -> Result<Vec<u8>> {
        let url = format!("{}/models/{}", self.api_base, model);

        let payload = json!({
            "inputs": text,
        });

        let mut last_error = None;

        for attempt in 1..=max_retries {
            tracing::debug!("TTS attempt {} for model: {}", attempt, model);

            match self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .json(&payload)
                .timeout(std::time::Duration::from_secs(120)) // Longer timeout for TTS
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        match response.bytes().await {
                            Ok(bytes) => {
                                let data = bytes.to_vec();
                                if data.len() > 100 {
                                    // Basic validation - audio should be more than 100 bytes
                                    tracing::debug!(
                                        "TTS successful, generated {} bytes",
                                        data.len()
                                    );
                                    return Ok(data);
                                } else {
                                    let error = AppError::ExternalApi(
                                        "TTS returned insufficient audio data".to_string(),
                                    );
                                    last_error = Some(error);
                                }
                            }
                            Err(e) => {
                                let error = AppError::ExternalApi(format!(
                                    "Failed to read TTS response bytes: {}",
                                    e
                                ));
                                last_error = Some(error);
                            }
                        }
                    } else {
                        let error_text = response.text().await.unwrap_or_default();
                        let error = AppError::ExternalApi(format!(
                            "HuggingFace TTS error {}: {}",
                            status, error_text
                        ));
                        last_error = Some(error);

                        // Don't retry on authentication or model not found errors
                        if status == 401 || status == 403 || status == 404 {
                            return Err(last_error.unwrap());
                        }
                    }
                }
                Err(e) => {
                    let error = AppError::ExternalApi(format!(
                        "Failed to connect to HuggingFace TTS API: {}",
                        e
                    ));
                    last_error = Some(error);
                }
            }

            if attempt < max_retries {
                let delay = std::time::Duration::from_millis(2000 * attempt as u64);
                tracing::warn!("TTS attempt {} failed, retrying in {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error.unwrap_or_else(|| AppError::ExternalApi("Unknown TTS error".to_string())))
    }

    /// BART-specific summarization with better parameters
    pub async fn summarize_bart(
        &self,
        model: &str,
        text: &str,
        max_length: usize,
        min_length: usize,
    ) -> Result<String> {
        self.summarize_bart_with_retry(model, text, max_length, min_length, 2)
            .await
    }

    async fn summarize_bart_with_retry(
        &self,
        model: &str,
        text: &str,
        max_length: usize,
        min_length: usize,
        max_retries: u32,
    ) -> Result<String> {
        let url = format!("{}/models/{}", self.api_base, model);

        let payload = json!({
            "inputs": text,
            "parameters": {
                "max_length": max_length,
                "min_length": min_length,
                "do_sample": false,
                "num_beams": 4,
                "early_stopping": true,
                "no_repeat_ngram_size": 3,
                "length_penalty": 2.0
            }
        });

        let mut last_error = None;

        for attempt in 1..=max_retries {
            tracing::debug!(
                "BART summarization attempt {} for model: {}",
                attempt,
                model
            );

            match self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .json(&payload)
                .timeout(std::time::Duration::from_secs(90))
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        match response.json::<Vec<SummarizationResponse>>().await {
                            Ok(result) => {
                                if let Some(first_result) = result.first() {
                                    let summary = &first_result.summary_text;
                                    if !summary.trim().is_empty()
                                        && summary.split_whitespace().count() >= 10
                                    {
                                        tracing::debug!(
                                            "BART summarization successful, {} words",
                                            summary.split_whitespace().count()
                                        );
                                        return Ok(summary.clone());
                                    } else {
                                        let error = AppError::ExternalApi(
                                            "BART returned insufficient summary content"
                                                .to_string(),
                                        );
                                        last_error = Some(error);
                                    }
                                } else {
                                    let error = AppError::ExternalApi(
                                        "No summary in BART response".to_string(),
                                    );
                                    last_error = Some(error);
                                }
                            }
                            Err(e) => {
                                let error = AppError::ExternalApi(format!(
                                    "Failed to parse BART response: {}",
                                    e
                                ));
                                last_error = Some(error);
                            }
                        }
                    } else {
                        let error_text = response.text().await.unwrap_or_default();
                        let error = AppError::ExternalApi(format!(
                            "HuggingFace Summarization error {}: {}",
                            status, error_text
                        ));
                        last_error = Some(error);

                        // Don't retry on authentication errors
                        if status == 401 || status == 403 {
                            return Err(last_error.unwrap());
                        }
                    }
                }
                Err(e) => {
                    let error = AppError::ExternalApi(format!(
                        "Failed to connect to HuggingFace Summarization API: {}",
                        e
                    ));
                    last_error = Some(error);
                }
            }

            if attempt < max_retries {
                let delay = std::time::Duration::from_millis(3000 * attempt as u64);
                tracing::warn!(
                    "BART summarization attempt {} failed, retrying in {:?}",
                    attempt,
                    delay
                );
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error
            .unwrap_or_else(|| AppError::ExternalApi("Unknown summarization error".to_string())))
    }

    /// Legacy summarize method for backward compatibility
    pub async fn summarize(&self, model: &str, text: &str) -> Result<String> {
        self.summarize_bart(model, text, 500, 50).await
    }
}

#[derive(Debug, Deserialize)]
struct InferenceResponse {
    generated_text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SummarizationResponse {
    summary_text: String,
}
