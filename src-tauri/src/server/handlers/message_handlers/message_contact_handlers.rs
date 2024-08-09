use crate::server::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow};

#[derive(Deserialize)]
struct NewContact {
    id: String,
    nick: String,
    age: Option<i32>, // Adjust type based on your SQL schema
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
}

#[post("/my/people/")]
async fn add_contact_my_client(
    pool: web::Data<AppState>,
    new_contact: web::Json<NewContact>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        INSERT INTO connecting_people (id, nick, age, location, occupation, extra_info)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_contact.id)
        .bind(&new_contact.nick)
        .bind(new_contact.age)
        .bind(&new_contact.location)
        .bind(&new_contact.occupation)
        .bind(&new_contact.extra_info)
        .execute(&pool.db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error adding contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other/people/")]
async fn add_contact_other_client(
    pool: web::Data<AppState>,
    new_contact: web::Json<NewContact>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        INSERT INTO connected_people (id, nick, age, location, occupation, extra_info)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_contact.id)
        .bind(&new_contact.nick)
        .bind(new_contact.age)
        .bind(&new_contact.location)
        .bind(&new_contact.occupation)
        .bind(&new_contact.extra_info)
        .execute(&pool.db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error adding contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ProcessedPerson {
    id: String,
    nick: String,
    age: Option<u32>,
    location: Option<String>,
    occupation: Option<String>,
    extra_info: Option<String>,
}

// Handler function to get connected people
#[get("/my/people")]
pub async fn get_connected_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM connected_people";
    match query_as::<_, ProcessedPerson>(query_str)
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connected people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Handler function to get connecting people
#[get("/other/people")]
pub async fn get_connecting_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM connecting_people";
    match query_as::<_, ProcessedPerson>(query_str)
        .fetch_all(&pool.db_pool)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connecting people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
