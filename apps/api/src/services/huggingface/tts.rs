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
        let model = self.get_tts_model(language);

        let truncated = text::truncate_text(text, 500);

        self.client.tts(model, &truncated).await
    }

    fn get_tts_model(&self, language: &str) -> &str {
        match language {
            "es" => "facebook/mms-tts-spa",
            "fr" => "facebook/mms-tts-fra",
            "de" => "facebook/mms-tts-deu",
            "it" => "facebook/mms-tts-ita",
            "pt" => "facebook/mms-tts-por",
            "zh" => "facebook/mms-tts-cmn",
            "ja" => "facebook/mms-tts-jpn",
            "ar" => "facebook/mms-tts-ara",
            "hi" => "facebook/mms-tts-hin",
            "ru" => "facebook/mms-tts-rus",
            _ => TTS_MODEL,
        }
    }
}
