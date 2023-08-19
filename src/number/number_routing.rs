use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};


#[get("/{id}/sold_numbers")]
pub async fn get_sold_numbers() -> impl Responder {
    

    HttpResponse::Ok().body("Hello world!")
}

#[post("/{id}/sell")]
pub async fn sell_numbers() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
