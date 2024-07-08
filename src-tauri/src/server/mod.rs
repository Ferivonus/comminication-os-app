mod handlers;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};

use handlers::form_handlers;
use handlers::message_handlers;
use handlers::wailing_wall_handlers;

use handlers::{example::handle, form_handlers::form_example};
use tauri::AppHandle;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use dotenv::dotenv;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::example::handle)
            .configure(message_handlers::message_example::message_handler_config)
            .configure(form_handlers::form_example::form_handler_config)
            .configure(wailing_wall_handlers::wailing_example::message_handler_config)
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}
