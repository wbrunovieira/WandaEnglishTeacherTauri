use reqwest::Client;
use std::error::Error;
use std::env;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use bytes::Bytes;

pub async fn convert_text_to_speech(message: &str) -> Result<Bytes, Box<dyn Error>> {
    let api_key = env::var("ELEVEN_LABS_API_KEY")?;
    let url = "https://api.elevenlabs.io/v1/text-to-speech/IKne3meq5aSn9XLyUdCD";
    
    let client = Client::new();

    let data = serde_json::json!({
        "text": message,
        "model_id": "eleven_monolingual_v1",
        "voice_settings": {
            "stability": 0.8,
            "similarity_boost": 0.5
        }
    });

    let response = client.post(url)
        .header("xi-api-key", api_key)
        .header(ACCEPT, "audio/mpeg")
        .header(CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await?;

    if response.status().is_success() {
        let audio_content = response.bytes().await?;
        println!("Áudio gerado com sucesso.");
        Ok(audio_content)
    } else {
        println!("Erro ao gerar áudio: {:?}", response.status());
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}
