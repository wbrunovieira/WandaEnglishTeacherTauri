// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod openai;
mod utils;
mod storage;
mod elevenlabs;
mod audio_capture;

use serde_json::json;
use tauri::command;

use openai::audio::convert_audio_to_text;
use openai::chat::get_chat_response;
use storage::messages::{get_recent_messages, store_messages, reset_messages};
use crate::audio_capture::audio_capture::AudioCapture;
use std::path::Path;

use serde_json::Value;

use crate::elevenlabs::tts::convert_text_to_speech;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn health_check() -> String {
    json!({"message": "healthy"}).to_string()
}

#[command]
async fn start_audio_capture() -> Result<String, String> {
    let mut audio_capture = AudioCapture::new();
    audio_capture.start().map_err(|e| e.to_string())?;

    // Simula a gravação por 5 segundos
    std::thread::sleep(std::time::Duration::from_secs(5));

    audio_capture.stop();
    let file_path = "output.wav";
    audio_capture.save_to_file(file_path)?;

    Ok(file_path.to_string())
}

#[command]
async fn process_audio(file_path: String) -> Result<String, String> {
    let audio_data = std::fs::read(&file_path).map_err(|e| e.to_string())?;
match convert_audio_to_text(&audio_data).await {
        Ok(transcript) => {
            let chat_response = get_chat_response(vec![], &transcript).await;
            chat_response.map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn generate_audio(message: String) -> Result<String, String> {
    match convert_text_to_speech(&message).await {
        Ok(audio_data) => {
            // Aqui você pode salvar o áudio gerado em um arquivo ou retornar o áudio como um arquivo temporário
            // Exemplo de salvar o arquivo:
            std::fs::write("output.mp3", &audio_data).map_err(|e| e.to_string())?;
            Ok("output.mp3".to_string())
        }
        Err(e) => {
            println!("Erro ao converter texto em áudio: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
fn get_messages() -> Vec<String> {
    get_recent_messages().iter().map(|msg| msg.content.clone()).collect()
}

#[command]
fn add_message(request: String, response: String) {
    store_messages(&request, &response);
}

#[command]
fn clear_messages() {
    reset_messages();
}


fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, health_check,process_audio,generate_audio,get_messages, add_message, clear_messages,start_audio_capture])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

         
}
