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
    validators::validate_language(&query.language)?;

    let summary_uuid = Uuid::parse_str(&summary_id)
        .map_err(|_| AppError::InvalidInput("Invalid summary ID".to_string()))?;

    let summary = state
        .db
        .get_summary_by_id(summary_uuid)
        .await?
        .ok_or(AppError::SummaryNotFound)?;

    if let Some(existing_audio) = state
        .db
        .get_audio_by_summary(summary_uuid, &query.language)
        .await?
    {
        tracing::info!("Returning existing audio for summary: {}", summary_id);

        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_vec(&existing_audio.to_response()).unwrap(),
        ));
    }

    let hf_client = HuggingFaceClient::new(
        state.http_client.clone(),
        state.config.hf_api_base_url.clone(),
        state.config.hf_token.clone(),
    );

    let tts_service = TTSService::new(hf_client);

    let audio_data = tts_service
        .generate_audio(&summary.summary_text, &query.language)
        .await?;

    let file_size_kb = (audio_data.len() / 1024) as i32;

    let audio_base64 = general_purpose::STANDARD.encode(&audio_data);
    let data_url = format!("data:audio/wav;base64,{}", audio_base64);

    let create_audio = CreateAudioFile {
        summary_id: summary_uuid,
        language: query.language.clone(),
        voice_type: query.voice_type.unwrap_or_else(|| "default".to_string()),
        file_url: data_url,
        duration_ms: None,
        file_size_kb: Some(file_size_kb),
    };

    let audio_file = state.db.create_audio_file(create_audio).await?;

    let response = audio_file.to_response();

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_vec(&response).unwrap(),
    ))
}
