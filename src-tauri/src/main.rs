// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::json;
use tauri::command;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn health_check() -> String {
    json!({"message": "healthy"}).to_string()
}


fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, health_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

         
}
