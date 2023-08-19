use actix_web::{
    get,
    post,
    Responder,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, cookie::Cookie,
};
use handlebars::Handlebars;


#[derive(serde::Serialize)]
struct Thing {
    name: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Info {
    username: String,
    password: String
}

#[get("/login")]
pub async fn login_page(hb: web::Data<Handlebars<'_>>) -> impl Responder {

    let body = hb.render("login/login", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/login")]
pub async fn login(web::Form(form): web::Form<Info>, hb: web::Data<Handlebars<'_>>, db: web::Data<sqlx::Pool<sqlx::Sqlite>>) -> HttpResponse {

    let dbaa = db.acquire().await.unwrap();
    let a =
     sqlx::query("SELECT * FROM USER where name = $1").bind(form.username).execute(db).await;

    let mut http = HttpResponse::build(StatusCode::OK);
    if true {
        let data = Thing {
            name: form.username
        };

        let cookie = actix_web::cookie::Cookie::new("session", format!("{}", data.name));
        http.append_header(("HX-Redirect", "/rifas"))
        .cookie(cookie);
        http.insert_header(
            ContentType::html()
        ).finish()
    } else {
        let data = Thing {
            name: form.username
        };

        let body = hb.render("error_title", &data).unwrap();
        http.insert_header(
            ContentType::html()
        ).cookie(Cookie::new("asd", "value")).body(body)
    }
}
