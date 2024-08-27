use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub async fn get_chat_response(messages: Vec<Value>, message_input: &str) -> Result<String, Box<dyn Error>> {
    let api_key = std::env::var("OPEN_AI_KEY")?;
    let client = Client::new();

    let mut messages = messages.clone();
    messages.push(json!({"role": "user", "content": message_input}));

    let request_body = json!({
        "model": "gpt-4",
        "messages": messages
    });

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let json: Value = response.json().await?;
    let message_text = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(message_text)
}
