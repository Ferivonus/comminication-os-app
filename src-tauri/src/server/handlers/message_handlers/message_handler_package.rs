use crate::server::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::query;

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
