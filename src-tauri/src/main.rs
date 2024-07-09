// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use reqwest::Client;
use std::thread;
use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_wailing_example_data() -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:4875/wailing/example")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_wailing_example_data, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
