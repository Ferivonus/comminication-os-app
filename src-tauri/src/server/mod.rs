mod handlers;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use tauri::AppHandle;

use handlers::form_handlers;
use handlers::message_handlers;
use handlers::wailing_wall_handlers;
struct AppState {
    tauri_app: Arc<AppHandle>,
    db_pool: MySqlPool,
}

#[actix_web::main]
pub async fn init(tauri_app: AppHandle) -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    // Set up database pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app_state = web::Data::new(AppState {
        tauri_app: Arc::new(tauri_app),
        db_pool,
    });

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
