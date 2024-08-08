use crate::server::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

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
    connected_person: Option<String>,
}

// Handler function to send a message to 'my-client'
#[post("/my-client/send")]
pub async fn send_message_my_client(
    pool: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let query_str = "INSERT INTO messages_send_to_my_client (sender, receiver, content, close_one_point, connected_person) VALUES (?, ?, ?, ?, ?)";

    match query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected_person)
        .execute(&pool.db_pool)
        .await
    {
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
    pool: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let query_str = "INSERT INTO messages_send_to_other_client (sender, receiver, content, close_one_point, connected_person) VALUES (?, ?, ?, ?, ?)";

    match query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected_person)
        .execute(&pool.db_pool)
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to get messages from 'my-client' by connected_person
#[get("/my-client/messages/{connected_person}")]
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

// Handler function to get messages from 'other-client' by connected_person
#[get("/other-client/messages/{connected_person}")]
pub async fn get_messages_other_client(
    pool: web::Data<AppState>,
    connected_person: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT id, sender, receiver, content, timestamp, counter, close_one_point, connected_person FROM messages_send_to_other_client WHERE connected_person = ? ORDER BY timestamp DESC";

    match query_as::<_, Message>(query_str)
        .bind(connected_person.as_str())
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Error retrieving messages from 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to reset 'messages_send_to_my_client' table
#[post("/reset-my-client-messages-table")]
pub async fn reset_messages_send_to_my_client_table_handler(
    pool: web::Data<AppState>,
) -> impl Responder {
    let drop_table_query = "DROP TABLE IF EXISTS messages_send_to_my_client;";
    let create_table_query = "
        CREATE TABLE messages_send_to_my_client (
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
        );";

    match query(drop_table_query).execute(&pool.db_pool).await {
        Ok(_) => match query(create_table_query).execute(&pool.db_pool).await {
            Ok(_) => {
                HttpResponse::Ok().body("Table 'messages_send_to_my_client' reset successfully")
            }
            Err(e) => {
                eprintln!("Error creating 'messages_send_to_my_client' table: {}", e);
                HttpResponse::InternalServerError().body("Error creating table")
            }
        },
        Err(e) => {
            eprintln!("Error dropping 'messages_send_to_my_client' table: {}", e);
            HttpResponse::InternalServerError().body("Error dropping table")
        }
    }
}

// Handler function to reset 'messages_send_to_other_client' table
#[post("/reset-other-client-messages-table")]
pub async fn reset_messages_send_to_other_client_table_handler(
    pool: web::Data<AppState>,
) -> impl Responder {
    let drop_table_query = "DROP TABLE IF EXISTS messages_send_to_other_client;";
    let create_table_query = "
        CREATE TABLE messages_send_to_other_client (
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
        );";

    match query(drop_table_query).execute(&pool.db_pool).await {
        Ok(_) => match query(create_table_query).execute(&pool.db_pool).await {
            Ok(_) => {
                HttpResponse::Ok().body("Table 'messages_send_to_other_client' reset successfully")
            }
            Err(e) => {
                eprintln!(
                    "Error creating 'messages_send_to_other_client' table: {}",
                    e
                );
                HttpResponse::InternalServerError().body("Error creating table")
            }
        },
        Err(e) => {
            eprintln!(
                "Error dropping 'messages_send_to_other_client' table: {}",
                e
            );
            HttpResponse::InternalServerError().body("Error dropping table")
        }
    }
}

// Handler function to reset 'connected_people' table
#[post("/reset-connected-people-table")]
pub async fn reset_connected_people_table_handler(pool: web::Data<AppState>) -> impl Responder {
    let drop_table_query = "DROP TABLE IF EXISTS connected_people;";
    let create_table_query = "
        CREATE TABLE connected_people (
            id VARCHAR(256) PRIMARY KEY,
            nick VARCHAR(255),
            age INT,
            location VARCHAR(255),
            occupation VARCHAR(255),
            extra_info VARCHAR(300)
        );";

    match query(drop_table_query).execute(&pool.db_pool).await {
        Ok(_) => match query(create_table_query).execute(&pool.db_pool).await {
            Ok(_) => HttpResponse::Ok().body("Table 'connected_people' reset successfully"),
            Err(e) => {
                eprintln!("Error creating 'connected_people' table: {}", e);
                HttpResponse::InternalServerError().body("Error creating table")
            }
        },
        Err(e) => {
            eprintln!("Error dropping 'connected_people' table: {}", e);
            HttpResponse::InternalServerError().body("Error dropping table")
        }
    }
}

// Handler function to reset 'connecting_people' table
#[post("/reset-connecting-people-table")]
pub async fn reset_connecting_people_table_handler(pool: web::Data<AppState>) -> impl Responder {
    let drop_table_query = "DROP TABLE IF EXISTS connecting_people;";
    let create_table_query = "
        CREATE TABLE connecting_people (
            id VARCHAR(256) PRIMARY KEY,
            nick VARCHAR(255),
            age INT,
            location VARCHAR(255),
            occupation VARCHAR(255),
            extra_info VARCHAR(300)
        );";

    match query(drop_table_query).execute(&pool.db_pool).await {
        Ok(_) => match query(create_table_query).execute(&pool.db_pool).await {
            Ok(_) => HttpResponse::Ok().body("Table 'connecting_people' reset successfully"),
            Err(e) => {
                eprintln!("Error creating 'connecting_people' table: {}", e);
                HttpResponse::InternalServerError().body("Error creating table")
            }
        },
        Err(e) => {
            eprintln!("Error dropping 'connecting_people' table: {}", e);
            HttpResponse::InternalServerError().body("Error dropping table")
        }
    }
}

// Handler function to get connected people
#[get("/connected-people")]
pub async fn get_connected_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM connected_people";
    match query_as::<_, ProcessedPerson>(query_str)
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connected people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to get connecting people
#[get("/connecting-people")]
pub async fn get_connecting_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM connecting_people";
    match query_as::<_, ProcessedPerson>(query_str)
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connecting people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ProcessedPerson {
    id: String,
    nick: String,
    age: Option<u32>,
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
}
