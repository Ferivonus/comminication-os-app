use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{query, MySqlPool};

// Define a struct to capture the message payload from the request
#[derive(Deserialize)]
struct NewMessage {
    sender: String,
    receiver: String,
    content: String,
    close_one_point: Option<String>, // Optional field
}

// Handler function to send a message to 'my-client'
#[post("/my-client/send")]
pub async fn send_message_my_client(
    pool: web::Data<MySqlPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    // Extract the new message details
    let new_message = new_message.into_inner();

    // Insert the new message into the database
    let result = sqlx::query(
        r#"
        INSERT INTO messages_send_to_my_client (sender, receiver, content, close_one_point)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(new_message.sender)
    .bind(new_message.receiver)
    .bind(new_message.content)
    .bind(new_message.close_one_point)
    .execute(&**pool)
    .await;

    // Check for errors and respond accordingly
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error inserting message: {}", e);
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
    // Extract the new message details
    let new_message = new_message.into_inner();

    // Insert the new message into the database
    let result = sqlx::query(
        r#"
        INSERT INTO messages_send_to_other_client (sender, receiver, content, close_one_point)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(new_message.sender)
    .bind(new_message.receiver)
    .bind(new_message.content)
    .bind(new_message.close_one_point)
    .execute(&**pool)
    .await;

    // Check for errors and respond accordingly
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error inserting message: {}", e);
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
            eprintln!("Error creating table: {:?}", err);
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
            eprintln!("Error creating table: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating table")
        }
    }
}

async fn create_messages_send_to_other_client_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS messages_send_to_other_client (
            id INT AUTO_INCREMENT PRIMARY KEY,
            sender VARCHAR(255) NOT NULL,
            receiver VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            counter INT DEFAULT 1,
            close_one_point VARCHAR(255),
            INDEX (close_one_point)
        );
    "#;
    query(create_table_query).execute(pool).await.map(|_| ())
}

async fn create_messages_send_to_my_client_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS messages_send_to_my_client (
            id INT AUTO_INCREMENT PRIMARY KEY,
            sender VARCHAR(255) NOT NULL,
            receiver VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
            counter INT DEFAULT 1,
            close_one_point VARCHAR(255),
            INDEX (close_one_point)
        );
    "#;
    query(create_table_query).execute(pool).await.map(|_| ())
}

pub fn message_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/message")
        .service(create_messages_send_to_my_client_table_handler)
        .service(create_messages_send_to_other_client_table_handler)
        .service(send_message_my_client)
        .service(send_message_other_client);
    conf.service(scope);
}
