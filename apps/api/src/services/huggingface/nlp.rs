use crate::{
    models::{ExtractedTerms, QueryIntent},
    services::huggingface::client::HuggingFaceClient,
    utils::errors::Result,
};

const NLP_MODEL: &str = "mistralai/Mistral-7B-Instruct-v0.2";

pub struct NLPService {
    client: HuggingFaceClient,
}

impl NLPService {
    pub fn new(client: HuggingFaceClient) -> Self {
        Self { client }
    }

    pub async fn understand_query(&self, query: &str) -> Result<QueryIntent> {
        let prompt = self.build_nlp_prompt(query);

        let response = self.client.text_generation(NLP_MODEL, &prompt).await?;

        self.parse_nlp_response(query, &response)
    }

    fn build_nlp_prompt(&self, query: &str) -> String {
        format!(
            r#"<s>[INST] You are a book search assistant. Extract key information from the user's query to help search for books.

User query: "{}"

Extract the following information in JSON format:
- genre: the book genre if mentioned (e.g., "thriller", "romance", "science fiction")
- theme: the main theme or topic (e.g., "artificial intelligence", "space travel", "medieval")
- keywords: list of important search keywords
- author: author name if mentioned
- title: book title if mentioned

Respond with only valid JSON, no additional text.

Example output:
{{"genre": "thriller", "theme": "artificial intelligence", "keywords": ["AI", "technology", "suspense"], "author": null, "title": null}}
[/INST]"#,
            query
        )
    }

    fn parse_nlp_response(&self, original_query: &str, response: &str) -> Result<QueryIntent> {
        let extracted = self.extract_json_from_response(response);

        let terms: ExtractedTerms =
            serde_json::from_str(&extracted).unwrap_or_else(|_| ExtractedTerms::default());

        let search_query = self.build_search_query(&terms, original_query);

        Ok(QueryIntent::new(
            original_query.to_string(),
            terms,
            search_query,
        ))
    }

    fn extract_json_from_response(&self, response: &str) -> String {
        if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                return response[start..=end].to_string();
            }
        }
        "{}".to_string()
    }

    fn build_search_query(&self, terms: &ExtractedTerms, fallback: &str) -> String {
        let mut parts = Vec::new();

        if let Some(title) = &terms.title {
            parts.push(title.clone());
        }

        if let Some(author) = &terms.author {
            parts.push(format!("author:{}", author));
        }

        if let Some(genre) = &terms.genre {
            parts.push(genre.clone());
        }

        if let Some(theme) = &terms.theme {
            parts.push(theme.clone());
        }

        for keyword in &terms.keywords {
            if !parts.iter().any(|p| p.contains(keyword)) {
                parts.push(keyword.clone());
            }
        }

        if parts.is_empty() {
            fallback.to_string()
        } else {
            parts.join(" ")
        }
    }
}
