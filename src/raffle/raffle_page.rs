use actix_web::{
    get,
    post,
    Responder,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, cookie::Cookie,
};
use handlebars::Handlebars;


#[get("/rifas")]
pub async fn raffles_page(hb: web::Data<Handlebars<'_>>) -> impl Responder {

    let body = hb.render("rifas", &{}).unwrap();
    HttpResponse::Ok().body(body)
}