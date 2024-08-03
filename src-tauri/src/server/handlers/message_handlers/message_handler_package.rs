use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::query;
use sqlx::FromRow;
use sqlx::MySqlPool;

// Define a struct to capture the message payload from the request
#[derive(serde::Deserialize)]
struct NewMessage {
    sender: String,
    receiver: String,
    content: String,
    close_one_point: Option<String>, // Optional field
}

// Handler function to send a message
#[post("/send")]
pub async fn send_message(
    pool: web::Data<MySqlPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    // Extract the new message details
    let new_message = new_message.into_inner();

    // Insert the new message into the database
    let result = sqlx::query(
        r#"
        INSERT INTO messages (sender, receiver, content, close_one_point)
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

async fn create_messages_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS messages (
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

// API endpoint'i
#[post("/create-table")]
pub async fn create_table_handler(pool: web::Data<MySqlPool>) -> HttpResponse {
    match create_messages_table(&pool).await {
        Ok(_) => HttpResponse::Ok().body("Table created successfully"),
        Err(err) => {
            eprintln!("Error creating table: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating table")
        }
    }
}

#[get("/example")]
pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().body("message Example handler")
}

pub fn message_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/message")
        .service(handle)
        .service(create_table_handler);
    conf.service(scope);
}
