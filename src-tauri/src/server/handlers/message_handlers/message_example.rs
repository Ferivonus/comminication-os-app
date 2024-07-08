use actix_web::{get, web, HttpResponse};

#[get("/example")]
pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().body("message Example handler")
}

pub fn message_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/message").service(handle);
    conf.service(scope);
}
