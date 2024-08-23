use crate::server::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow};

// Define the Message struct to use with database queries
#[derive(Debug, FromRow)]
struct Message {
    id: i32,
    sender: String,
    receiver: String,
    content: String,
    timestamp: DateTime<Utc>,
    close_one_point: Option<String>,
    connected: String,
}

// Define a struct to represent a message record for API responses
#[derive(Debug, Serialize)]
struct MessageResponse {
    id: i32,
    sender: String,
    receiver: String,
    content: String,
    timestamp: String, // Change to String for API responses
    close_one_point: Option<String>,
    connected: String,
}

impl Message {
    fn to_response(&self) -> MessageResponse {
        MessageResponse {
            id: self.id,
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            content: self.content.clone(),
            timestamp: self.timestamp.to_rfc3339(), // Convert to string in RFC 3339 format
            close_one_point: self.close_one_point.clone(),
            connected: self.connected.clone(),
        }
    }
}

#[get("/my/get/{connected}")]
pub async fn get_messages_my_client(
    pool: web::Data<AppState>,
    connected: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT id, sender, receiver, content, timestamp, close_one_point, connected FROM messages_send_to_my_client WHERE connected = ? ORDER BY timestamp DESC";

    match sqlx::query_as::<_, Message>(query_str)
        .bind(connected.as_str())
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(messages) => {
            let response: Vec<MessageResponse> =
                messages.into_iter().map(|m| m.to_response()).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error retrieving messages from 'my-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/other/get/{connected}")]
pub async fn get_messages_other_client(
    pool: web::Data<AppState>,
    connected: web::Path<String>,
) -> impl Responder {
    let query_str = "
        SELECT id, sender, receiver, content, timestamp, close_one_point, connected 
        FROM messages_send_to_other_client 
        WHERE connected = ? 
        ORDER BY timestamp DESC
    ";

    match sqlx::query_as::<_, Message>(query_str)
        .bind(connected.as_str())
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(messages) => {
            let response: Vec<MessageResponse> =
                messages.into_iter().map(|m| m.to_response()).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error retrieving messages from 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Define a struct to capture the message payload from the request
#[derive(Deserialize)]
struct NewMessage {
    sender: String,
    receiver: String,
    content: String,
    close_one_point: Option<String>, // Optional field
    connected: String,               // Optional field to track conversation partner
}

// Handler function to send a message to 'my-client'
#[post("/my/send/")]
pub async fn send_message_my_client(
    pool: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let query_str = "
        INSERT INTO messages_send_to_my_client (sender, receiver, content, close_one_point, connected)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected)
        .execute(&pool.db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'my-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to send a message to 'other-client'
#[post("/other/send/")]
pub async fn send_message_other_client(
    pool: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let query_str = "
        INSERT INTO messages_send_to_other_client (sender, receiver, content, close_one_point, connected)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected)
        .execute(&pool.db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
