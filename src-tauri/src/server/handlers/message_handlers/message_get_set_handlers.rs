use crate::server::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

// Handler function to get messages from 'my-client' by connected_person
#[get("/my/get/{connected_person}")]
pub async fn get_messages_my_client(
    pool: web::Data<AppState>,
    connected_person: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT id, sender, receiver, content, timestamp, counter, close_one_point, connected_person FROM messages_send_to_my_client WHERE connected_person = ? ORDER BY timestamp DESC";

    match query_as::<_, Message>(query_str)
        .bind(connected_person.as_str())
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Error retrieving messages from 'my-client' table: {}", e);
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
    close_one_point: Option<String>,  // Optional field
    connected_person: Option<String>, // Optional field to track conversation partner
}

// Define a struct to represent a message record
#[derive(Debug, Serialize, FromRow)]
struct Message {
    id: String,
    sender: String,
    receiver: String,
    content: String,
    timestamp: String, // Change this to String for serialization
    counter: i32,
    close_one_point: Option<String>,
    connected_person: String,
}

#[get("/other/get/{connected_person}")]
pub async fn get_messages_other_client(
    pool: web::Data<AppState>,
    connected_person: web::Path<String>,
) -> impl Responder {
    let query_str = "
        SELECT id, sender, receiver, content, timestamp, counter, close_one_point, connected_person 
        FROM messages_send_to_other_client 
        WHERE connected_person = ? 
        ORDER BY timestamp DESC
    ";

    match sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            String,
            sqlx::types::chrono::NaiveDateTime,
            i32,
            Option<String>,
            String,
        ),
    >(query_str)
    .bind(connected_person.into_inner())
    .fetch_all(&pool.db_pool)
    .await
    {
        Ok(results) => {
            // Map results to Message struct
            let messages: Vec<Message> = results
                .into_iter()
                .map(
                    |(
                        id,
                        sender,
                        receiver,
                        content,
                        timestamp,
                        counter,
                        close_one_point,
                        connected_person,
                    )| {
                        Message {
                            id,
                            sender,
                            receiver,
                            content,
                            timestamp: timestamp.to_string(), // Convert NaiveDateTime to string
                            counter,
                            close_one_point,
                            connected_person,
                        }
                    },
                )
                .collect();

            HttpResponse::Ok().json(messages)
        }
        Err(e) => {
            eprintln!("Error retrieving messages from 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to send a message to 'my-client'
#[post("/my/send/")]
pub async fn send_message_my_client(
    pool: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        INSERT INTO messages_send_to_my_client (sender, receiver, content, close_one_point, connected_person)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected_person)
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
        INSERT INTO messages_send_to_other_client (sender, receiver, content, close_one_point, connected_person)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected_person)
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
