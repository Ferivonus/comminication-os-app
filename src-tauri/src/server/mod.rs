use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;
use tauri::AppHandle; // Ensure to import env_logger

mod handlers;
use handlers::{form_handlers, message_handlers, wailing_wall_handlers};

#[allow(dead_code)]
pub struct AppState {
    tauri_app: Arc<AppHandle>,
    db_pool: MySqlPool,
}

// Function to initialize logging
fn init_logging() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
}

// Function to initialize the database pool
async fn init_db_pool() -> MySqlPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}

#[actix_web::main]
pub async fn init(tauri_app: AppHandle) -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    init_logging(); // Initialize the logger

    // Set up database pool
    let db_pool = init_db_pool().await;
    println!("✅ Connection to the database is successful!");

    // Create application state
    let app_state = web::Data::new(AppState {
        tauri_app: Arc::new(tauri_app),
        db_pool,
    });

    // Configure and start the HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:1420")
            .allowed_origin("http://127.0.0.1:4875")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(message_handlers::message_handler_package::message_handler_config)
            .configure(form_handlers::form_handler_package::form_handler_config)
            .configure(wailing_wall_handlers::wailing_handler_package::message_handler_config)
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}
