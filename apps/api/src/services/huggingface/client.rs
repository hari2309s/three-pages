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
        let url = format!("{}/models/{}", self.api_base, model);

        let mut payload = json!({
            "inputs": inputs,
        });

        if let Some(params) = parameters {
            payload["parameters"] = params;
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalApi(format!(
                "HuggingFace API error: {}",
                error_text
            )));
        }

        let result: Vec<InferenceResponse> = response.json().await?;

        result
            .first()
            .and_then(|r| r.generated_text.clone())
            .ok_or_else(|| AppError::ExternalApi("No generated text in response".to_string()))
    }

    pub async fn text_generation(&self, model: &str, prompt: &str) -> Result<String> {
        let parameters = json!({
            "max_new_tokens": 2000,
            "temperature": 0.7,
            "top_p": 0.95,
            "do_sample": true,
        });

        self.inference(model, prompt, Some(parameters)).await
    }

    pub async fn tts(&self, model: &str, text: &str) -> Result<Vec<u8>> {
        let url = format!("{}/models/{}", self.api_base, model);

        let payload = json!({
            "inputs": text,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalApi(format!(
                "HuggingFace TTS error: {}",
                error_text
            )));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, Deserialize)]
struct InferenceResponse {
    generated_text: Option<String>,
}
