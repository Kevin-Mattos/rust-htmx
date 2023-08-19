use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder, http};
use crate::raffle::raffle_repository;

pub fn get_scope() -> actix_web::Scope {
    web::scope("/raffle")
    .service(get_raffles)
    .service(create_raffle)
    .service(get_raffle_by_id)
    .service(get_raffle_graph_data)
    .service(delete_raffle)
}

#[get("")]
pub async fn get_raffles() -> impl Responder {
    let a = raffle_repository::RaffleRepository{};
    let json = web::Json(a.get_all());
    (json, http::StatusCode::OK)
}

#[post("")]
pub async fn create_raffle() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get_raffle_by_id(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body("asdf")
}

#[get("/{id}")]
pub async fn get_raffle_graph_data(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

#[delete("/{id}")]
pub async fn delete_raffle(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}
