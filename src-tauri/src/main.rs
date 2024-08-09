#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::fmt;
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct FormPage {
    slug: String,
    title: String,
}

#[derive(Deserialize, Serialize)]
struct NewMessage {
    sender: String,
    receiver: String,
    content: String,
    close_one_point: Option<String>,
    connected_person: Option<String>,
}

// Define error types for more specific error handling
#[derive(Debug)]
enum ApiError {
    ReqwestError(ReqwestError),
    HttpError(reqwest::StatusCode),
    ParseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "Request error: {}", err),
            ApiError::HttpError(status) => write!(f, "HTTP error with status code: {}", status),
            ApiError::ParseError(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<ReqwestError> for ApiError {
    fn from(error: ReqwestError) -> Self {
        ApiError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::ParseError(error.to_string())
    }
}

// Helper function to handle HTTP responses
async fn handle_response(response: reqwest::Response) -> Result<String, ApiError> {
    if response.status().is_success() {
        response
            .text()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    } else {
        Err(ApiError::HttpError(response.status()))
    }
}

// Command to fetch form pages
#[tauri::command]
async fn fetch_form_pages() -> Result<Vec<FormPage>, String> {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:4875/form/all-form-pages")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let form_pages: Vec<FormPage> = response.json().await.map_err(|e| e.to_string())?;
        Ok(form_pages)
    } else {
        Err(format!("Failed to fetch form pages: {}", response.status()))
    }
}

// Command to notify the frontend
#[tauri::command]
async fn notify_frontend(app_handle: tauri::AppHandle, message: String) -> Result<(), String> {
    app_handle
        .emit_all("notification", message)
        .map_err(|e| e.to_string())
}

// Command to greet
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Command to fetch example data
#[tauri::command]
async fn fetch_wailing_example_data() -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:4875/wailing/example")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    handle_response(response).await.map_err(|e| e.to_string())
}

// Command to send a message to 'my-client'
#[tauri::command]
async fn send_message_my_client(message: NewMessage) -> Result<(), String> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:4875/message/my/send/")
        .json(&message)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    handle_response(response)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// Command to send a message to 'other-client'
#[tauri::command]
async fn send_message_other_client(message: NewMessage) -> Result<(), String> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:4875/message/other/send/")
        .json(&message)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // Pass the entire response to handle_response
    handle_response(response)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// Command to get messages from 'my-client' by connectedPerson
#[tauri::command]
async fn get_messages_my_client(connected: String) -> Result<String, String> {
    validate_connected_person(&connected)?;

    let client = Client::new();
    let url = format!("http://127.0.0.1:4875/message/my/get/{}", connected);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    handle_response(response).await.map_err(|e| e.to_string())
}

// Command to get messages from 'other-client' by connectedPerson
#[tauri::command]
async fn get_messages_other_client(connected: String) -> Result<String, String> {
    validate_connected_person(&connected)?;

    let client = Client::new();
    let url = format!("http://127.0.0.1:4875/message/other/get/{}", connected);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    handle_response(response).await.map_err(|e| e.to_string())
}

// Helper function to validate connected_person
fn validate_connected_person(connected: &str) -> Result<(), String> {
    if connected.is_empty() {
        Err("Connected person cannot be empty".into())
    } else {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    id: String,
    nick: Option<String>,
    age: Option<i32>,
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
}

#[tauri::command]
async fn get_contacts_my_client() -> Result<Vec<Person>, String> {
    let client = Client::new();
    let url = "http://127.0.0.1:4875/message/my/people";
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let people: Vec<Person> = response.json().await.map_err(|e| e.to_string())?;
        Ok(people)
    } else {
        Err(format!(
            "Failed to fetch contacts from 'my-client': {}",
            response.status()
        ))
    }
}

#[tauri::command]
async fn get_contacts_other_client() -> Result<Vec<Person>, String> {
    let client = Client::new();
    let url = "http://127.0.0.1:4875/message/other/people";
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let people: Vec<Person> = response.json().await.map_err(|e| e.to_string())?;
        Ok(people)
    } else {
        Err(format!(
            "Failed to fetch contacts from 'other-client': {}",
            response.status()
        ))
    }
}

#[tauri::command]
async fn add_contact_my_client(
    id: String,
    nick: String,
    age: Option<i32>,
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
) -> Result<(), String> {
    let response: reqwest::Response = reqwest::Client::new()
        .post("http://127.0.0.1:4875/message/my/people/") // Adjust the URL if necessary
        .json(&serde_json::json!({
            "id": id,
            "nick": nick,
            "age": age,
            "location": location,
            "occupation": occupation,
            "extra_info": extra_info
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to add contact: {}", response.status()))
    }
}

#[tauri::command]
async fn add_contact_other_client(
    id: String,
    nick: String,
    age: Option<i32>,
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
) -> Result<(), String> {
    let response: reqwest::Response = reqwest::Client::new()
        .post("http://127.0.0.1:4875/message/other/people/") // Adjust the URL if necessary
        .json(&serde_json::json!({
            "id": id,
            "nick": nick,
            "age": age,
            "location": location,
            "occupation": occupation,
            "extra_info": extra_info
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to add contact: {}", response.status()))
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize server in a separate thread
            std::thread::spawn(move || {
                if let Err(e) = server::init(app_handle.clone()) {
                    eprintln!("Error initializing server: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_wailing_example_data,
            greet,
            notify_frontend,
            send_message_my_client,
            send_message_other_client,
            get_messages_my_client,
            get_messages_other_client,
            fetch_form_pages,
            get_contacts_my_client,
            get_contacts_other_client,
            add_contact_my_client,
            add_contact_other_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
