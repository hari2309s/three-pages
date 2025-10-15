use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

use crate::{
    models::{AudioFile, CreateAudioFile, CreateSummary, Summary},
    utils::errors::Result,
};

#[derive(Clone)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self> {
        // Sanitize URL for logging
        let sanitized_url = if database_url.contains("@") {
            let parts: Vec<&str> = database_url.split("@").collect();
            if parts.len() > 1 {
                format!("postgres://***:***@{}", parts[1])
            } else {
                "postgres://***".to_string()
            }
        } else {
            "postgres://***".to_string()
        };

        tracing::info!(
            "Creating database connection pool with {} connections to {}",
            max_connections,
            sanitized_url
        );

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(30))
            .connect(database_url)
            .await
            .map_err(|e| {
                tracing::error!("Database connection pool creation failed: {}", e);
                tracing::error!("Connection URL format: {}", sanitized_url);
                tracing::error!("Max connections: {}", max_connections);
                e
            })?;

        tracing::info!("Database connection pool created successfully");
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn create_summary(&self, summary: CreateSummary) -> Result<Summary> {
        let id = Uuid::new_v4();

        let record = sqlx::query_as::<_, Summary>(
            r#"
            INSERT INTO summaries (
                id, book_id, book_title, book_author, isbn, language,
                summary_text, word_count, style, source_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&summary.book_id)
        .bind(&summary.book_title)
        .bind(&summary.book_author)
        .bind(&summary.isbn)
        .bind(&summary.language)
        .bind(&summary.summary_text)
        .bind(summary.word_count)
        .bind(&summary.style)
        .bind(&summary.source_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_summary_by_book(
        &self,
        book_id: &str,
        language: &str,
        style: &str,
    ) -> Result<Option<Summary>> {
        let record = sqlx::query_as::<_, Summary>(
            r#"
            SELECT * FROM summaries
            WHERE book_id = $1 AND language = $2 AND style = $3
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(book_id)
        .bind(language)
        .bind(style)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_summary_by_id(&self, id: Uuid) -> Result<Option<Summary>> {
        let record = sqlx::query_as::<_, Summary>(
            r#"
            SELECT * FROM summaries WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn create_audio_file(&self, audio: CreateAudioFile) -> Result<AudioFile> {
        let id = Uuid::new_v4();

        let record = sqlx::query_as::<_, AudioFile>(
            r#"
            INSERT INTO audio_files (
                id, summary_id, language, voice_type, file_url, duration_ms, file_size_kb
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(audio.summary_id)
        .bind(&audio.language)
        .bind(&audio.voice_type)
        .bind(&audio.file_url)
        .bind(audio.duration_ms)
        .bind(audio.file_size_kb)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_audio_by_summary(
        &self,
        summary_id: Uuid,
        language: &str,
    ) -> Result<Option<AudioFile>> {
        let record = sqlx::query_as::<_, AudioFile>(
            r#"
            SELECT * FROM audio_files
            WHERE summary_id = $1 AND language = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(summary_id)
        .bind(language)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_audio_by_id(&self, id: Uuid) -> Result<Option<AudioFile>> {
        let record = sqlx::query_as::<_, AudioFile>(
            r#"
            SELECT * FROM audio_files WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
