use reqwest::Client;
use reqwest::multipart;
use std::error::Error;
use serde_json::Value;

pub async fn convert_audio_to_text(audio_data: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
    let api_key = std::env::var("OPEN_AI_KEY")?;
    let client = Client::new();

  
    let part = multipart::Part::bytes(audio_data.to_vec())
        .file_name("audio.wav")
        .mime_str("audio/wav")?;

    let form = multipart::Form::new()
        .part("file", part);

    let response = client.post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?;

    let json: Value = response.json().await?;
    let transcript = json["text"].as_str().unwrap_or_default().to_string();
    
    Ok(transcript)
}
