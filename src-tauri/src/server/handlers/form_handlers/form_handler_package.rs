use actix_web::{get, web, HttpResponse};

#[get("/example")]
pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().body("Form Example handler")
}

pub fn form_handler_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/form").service(handle), //.service(every_message_handler),
    );
}
