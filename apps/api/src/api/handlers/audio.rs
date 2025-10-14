use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::CreateAudioFile,
    services::huggingface::{HuggingFaceClient, TTSService},
    utils::{
        errors::{AppError, Result},
        validators,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct AudioQuery {
    pub language: String,
    pub voice_type: Option<String>,
}

pub async fn get_audio(
    State(state): State<AppState>,
    Path(summary_id): Path<String>,
    Query(query): Query<AudioQuery>,
) -> Result<impl IntoResponse> {
    tracing::info!(
        "Audio generation request - Summary: {}, Language: {}, Voice: {:?}",
        summary_id,
        query.language,
        query.voice_type
    );

    validators::validate_language(&query.language)?;

    let summary_uuid = Uuid::parse_str(&summary_id)
        .map_err(|_| AppError::InvalidInput("Invalid summary ID format".to_string()))?;

    let summary = state
        .db
        .get_summary_by_id(summary_uuid)
        .await?
        .ok_or_else(|| {
            tracing::warn!("Summary not found: {}", summary_id);
            AppError::SummaryNotFound
        })?;

    // Check for existing audio file first
    if let Some(existing_audio) = state
        .db
        .get_audio_by_summary(summary_uuid, &query.language)
        .await?
    {
        tracing::info!(
            "Returning existing audio for summary: {} in language: {}",
            summary_id,
            query.language
        );

        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_vec(&existing_audio.to_response()).unwrap(),
        ));
    }

    tracing::info!(
        "No existing audio found, generating new audio for summary: {} in language: {}",
        summary_id,
        query.language
    );

    // Validate summary has content for audio generation
    if summary.summary_text.trim().is_empty() {
        tracing::error!("Summary has no text content for audio generation");
        return Err(AppError::InvalidInput(
            "Summary contains no text for audio generation".to_string(),
        ));
    }

    tracing::debug!(
        "Summary text length: {} characters for audio generation",
        summary.summary_text.len()
    );

    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let tts_service = TTSService::new(hf_client);

    tracing::info!(
        "Starting TTS generation for summary {} in language {} with {} characters",
        summary_id,
        query.language,
        summary.summary_text.len()
    );

    let audio_data = match tts_service
        .generate_audio(&summary.summary_text, &query.language)
        .await
    {
        Ok(data) => {
            if data.is_empty() {
                tracing::error!(
                    "TTS service returned empty audio data for summary {}",
                    summary_id
                );
                return Err(AppError::ServiceError(
                    "Audio generation completed but produced no audio data. Please try again."
                        .to_string(),
                ));
            }

            tracing::info!(
                "Successfully generated audio: {} bytes for summary {} in language {}",
                data.len(),
                summary_id,
                query.language
            );
            data
        }
        Err(e) => {
            tracing::error!(
                "TTS generation failed for summary {} in language {}: {}",
                summary_id,
                query.language,
                e
            );

            // Provide more user-friendly error messages based on error type
            let user_error = match e {
                crate::utils::errors::AppError::ExternalApi(ref msg) => {
                    if msg.contains("Authentication") || msg.contains("401") || msg.contains("403")
                    {
                        "Audio service authentication failed. Please contact support.".to_string()
                    } else if msg.contains("timeout") || msg.contains("timed out") {
                        "Audio generation timed out. Please try again with shorter text."
                            .to_string()
                    } else if msg.contains("rate limit") || msg.contains("429") {
                        "Audio service is temporarily busy. Please try again in a moment."
                            .to_string()
                    } else if msg.contains("model") || msg.contains("404") {
                        "Audio generation model temporarily unavailable. Please try again later."
                            .to_string()
                    } else {
                        "Audio generation service temporarily unavailable. Please try again later."
                            .to_string()
                    }
                }
                _ => "Audio generation failed due to an unexpected error. Please try again."
                    .to_string(),
            };

            return Err(AppError::ServiceError(user_error));
        }
    };

    let file_size_kb = (audio_data.len() / 1024) as i32;

    tracing::debug!("Encoding audio data to base64, size: {}KB", file_size_kb);

    let audio_base64 = general_purpose::STANDARD.encode(&audio_data);
    let data_url = format!("data:audio/wav;base64,{}", audio_base64);

    if audio_base64.len() < 100 {
        tracing::error!(
            "Generated base64 audio data is too small ({} chars) for summary {}",
            audio_base64.len(),
            summary_id
        );
        return Err(AppError::ServiceError(
            "Generated audio appears to be corrupted. Please try again.".to_string(),
        ));
    }

    tracing::debug!(
        "Generated base64 audio data: {} characters for summary {}",
        audio_base64.len(),
        summary_id
    );

    let create_audio = CreateAudioFile {
        summary_id: summary_uuid,
        language: query.language.clone(),
        voice_type: query.voice_type.unwrap_or_else(|| "default".to_string()),
        file_url: data_url,
        duration_ms: Some(file_size_kb * 100), // Rough estimate: ~100ms per KB
        file_size_kb: Some(file_size_kb),
    };

    let audio_file = match state.db.create_audio_file(create_audio).await {
        Ok(file) => {
            tracing::info!(
                "Successfully saved audio file for summary: {} in language: {}",
                summary_id,
                query.language
            );
            file
        }
        Err(e) => {
            tracing::error!(
                "Failed to save audio file to database for summary {}: {}",
                summary_id,
                e
            );
            return Err(AppError::ServiceError(
                "Audio was generated successfully but failed to save. Please try again."
                    .to_string(),
            ));
        }
    };

    let response = audio_file.to_response();

    tracing::info!(
        "Audio generation completed successfully - Summary: {}, Language: {}, Size: {}KB",
        summary_id,
        query.language,
        file_size_kb
    );

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_vec(&response).unwrap(),
    ))
}
