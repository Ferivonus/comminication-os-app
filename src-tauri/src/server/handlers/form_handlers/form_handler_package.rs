use crate::server::AppState;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::FromRow;

#[derive(FromRow)]
struct Message {
    id: i32,
    content: String,
}

#[get("/example")]
pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().body("Form Example handler")
}
/**
 *
 * #[get("/form/messages")]
pub async fn every_message_handler(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.db_pool;
    let result = sqlx::query_as!(Message, "SELECT id, content FROM messages")
        .fetch_all(pool)
        .await;

    match result {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
 *
 */

pub fn form_handler_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/form").service(handle), //.service(every_message_handler),
    );
}
