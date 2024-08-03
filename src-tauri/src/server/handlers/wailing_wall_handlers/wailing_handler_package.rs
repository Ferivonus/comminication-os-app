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
    HttpResponse::Ok().body("wailing Example handler")
}

pub fn message_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/wailing").service(handle);
    conf.service(scope);
}
