mod raffle;
use raffle::raffle_routing;
mod login;
use sqlx::{Connection, Sqlite, Pool};

use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Result,
};
use handlebars::Handlebars;
// use serde_json::json;

struct MyDb {
    sqlite: Pool<Sqlite>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conn = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&conn).await;

    let mut handlebars = Handlebars::new();
    handlebars.set_dev_mode(true);
    // handlebars.register_template_file("parent", "./static/templates/parent.html");
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    let db_ref = web::Data::new(conn);

    HttpServer::new(move || {
        App::new()
        .wrap(error_handlers())
        .app_data(handlebars_ref.clone())
        .app_data(db_ref.clone())
            .service(raffle_routing::get_scope())
            .service(index)
            .service(raffle::raffle_page::raffles_page)
            .service(login::login_page::login)
            .service(login::login_page::login_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    let body = hb.render("index", &{}).unwrap();
    HttpResponse::Ok().body(body)
}


fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>();
    match hb {
        Some(hb) => {
            let data = HtmlError {
                err: error.to_string(),
                status: res.status().as_str().to_string()
            };
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[derive(serde::Serialize)]
struct HtmlError {
    err: String,
    status: String
}