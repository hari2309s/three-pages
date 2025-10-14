use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AudioResponse {
    pub id: Uuid,
    pub summary_id: Uuid,
    pub language: String,
    pub voice_type: String,
    pub duration_ms: Option<i32>,
    pub file_size_kb: Option<i32>,
    pub audio_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AudioFile {
    pub id: Uuid,
    pub summary_id: Uuid,
    pub language: String,
    pub voice_type: String,
    pub file_url: String,
    pub duration_ms: Option<i32>,
    pub file_size_kb: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl AudioFile {
    pub fn to_response(self) -> AudioResponse {
        AudioResponse {
            id: self.id,
            summary_id: self.summary_id,
            language: self.language,
            voice_type: self.voice_type,
            duration_ms: self.duration_ms,
            file_size_kb: self.file_size_kb,
            audio_url: self.file_url,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug)]
pub struct CreateAudioFile {
    pub summary_id: Uuid,
    pub language: String,
    pub voice_type: String,
    pub file_url: String,
    pub duration_ms: Option<i32>,
    pub file_size_kb: Option<i32>,
}
