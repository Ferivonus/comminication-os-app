use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError, FromRow, MySqlPool};

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
    id: i32,
    sender: String,
    receiver: String,
    content: String,
    timestamp: String,
    counter: i32,
    close_one_point: Option<String>,
}

// Helper function to handle SQL insertion
async fn insert_message(
    pool: &MySqlPool,
    table: &str,
    new_message: &NewMessage,
) -> Result<(), SqlxError> {
    let query_str = format!(
        "INSERT INTO {} (sender, receiver, content, close_one_point, connected_person) VALUES (?, ?, ?, ?, ?)",
        table
    );

    query(&query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected_person)
        .execute(pool)
        .await?;

    Ok(())
}

// Handler function to send a message to 'my-client'
#[post("/my-client/send")]
pub async fn send_message_my_client(
    pool: web::Data<MySqlPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let new_message = new_message.into_inner();

    match insert_message(&pool, "messages_send_to_my_client", &new_message).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'my-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to send a message to 'other-client'
#[post("/other-client/send")]
pub async fn send_message_other_client(
    pool: web::Data<MySqlPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let new_message = new_message.into_inner();

    match insert_message(&pool, "messages_send_to_other_client", &new_message).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Helper function to retrieve messages by connected_person
async fn get_messages_by_person(
    pool: &MySqlPool,
    table: &str,
    connected_person: &str,
) -> Result<Vec<Message>, SqlxError> {
    let query_str = format!(
        "SELECT id, sender, receiver, content, timestamp, counter, close_one_point FROM {} WHERE connected_person = ? ORDER BY timestamp DESC",
        table
    );

    let messages = query_as::<_, Message>(&query_str)
        .bind(connected_person)
        .fetch_all(pool)
        .await?;

    Ok(messages)
}

// Handler function to get messages from 'my-client' by connected_person
#[get("/my-client/messages/{connected_person}")]
pub async fn get_messages_my_client(
    pool: web::Data<MySqlPool>,
    connected_person: web::Path<String>,
) -> impl Responder {
    let connected_person = connected_person.into_inner();

    match get_messages_by_person(&pool, "messages_send_to_my_client", &connected_person).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Error retrieving messages from 'my-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to get messages from 'other-client' by connected_person
#[get("/other-client/messages/{connected_person}")]
pub async fn get_messages_other_client(
    pool: web::Data<MySqlPool>,
    connected_person: web::Path<String>,
) -> impl Responder {
    let connected_person = connected_person.into_inner();

    match get_messages_by_person(&pool, "messages_send_to_other_client", &connected_person).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Error retrieving messages from 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// API endpoint to create 'messages_send_to_my_client' table
#[post("/create-my-local-messages-on-my-torr-server-table")]
pub async fn create_messages_send_to_my_client_table_handler(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    match create_messages_send_to_my_client_table(&pool).await {
        Ok(_) => HttpResponse::Ok().body("Table created successfully"),
        Err(err) => {
            eprintln!("Error creating 'my-client' table: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating table")
        }
    }
}

// API endpoint to create 'messages_send_to_other_client' table
#[post("/create-messages-send-to-torr-accounts-table")]
pub async fn create_messages_send_to_other_client_table_handler(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    match create_messages_send_to_other_client_table(&pool).await {
        Ok(_) => HttpResponse::Ok().body("Table created successfully"),
        Err(err) => {
            eprintln!("Error creating 'other-client' table: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating table")
        }
    }
}

// Helper function to create a table
async fn create_table(pool: &MySqlPool, table_name: &str) -> Result<(), SqlxError> {
    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INT AUTO_INCREMENT PRIMARY KEY,
            sender VARCHAR(255) NOT NULL,
            receiver VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            counter INT DEFAULT 1,
            close_one_point VARCHAR(255),
            connected_person VARCHAR(255),
            INDEX (connected_person),
            INDEX (close_one_point)
        );",
        table_name
    );

    query(&create_table_query).execute(pool).await.map(|_| ())
}

async fn create_messages_send_to_other_client_table(pool: &MySqlPool) -> Result<(), SqlxError> {
    create_table(pool, "messages_send_to_other_client").await
}

async fn create_messages_send_to_my_client_table(pool: &MySqlPool) -> Result<(), SqlxError> {
    create_table(pool, "messages_send_to_my_client").await
}
