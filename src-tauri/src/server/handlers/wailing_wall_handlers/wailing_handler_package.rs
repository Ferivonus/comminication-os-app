use actix_web::{get, HttpResponse};

#[get("/example")]
pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().body("wailing Example handler")
}
