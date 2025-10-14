use crate::utils::errors::{AppError, Result};

const SUPPORTED_LANGUAGES: &[&str] = &["en"];

pub fn validate_language(lang: &str) -> Result<()> {
    if SUPPORTED_LANGUAGES.contains(&lang) {
        Ok(())
    } else {
        Err(AppError::InvalidInput(format!(
            "Unsupported language: {}. Supported: {}",
            lang,
            SUPPORTED_LANGUAGES.join(", ")
        )))
    }
}

pub fn validate_query(query: &str) -> Result<()> {
    let trimmed = query.trim();

    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("Query cannot be empty".to_string()));
    }

    if trimmed.len() < 2 {
        return Err(AppError::InvalidInput(
            "Query must be at least 2 characters".to_string(),
        ));
    }

    if trimmed.len() > 500 {
        return Err(AppError::InvalidInput(
            "Query cannot exceed 500 characters".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_style(style: &str) -> Result<()> {
    const VALID_STYLES: &[&str] = &["concise", "detailed", "academic", "simple"];

    if VALID_STYLES.contains(&style) {
        Ok(())
    } else {
        Err(AppError::InvalidInput(format!(
            "Invalid style: {}. Valid styles: {}",
            style,
            VALID_STYLES.join(", ")
        )))
    }
}
