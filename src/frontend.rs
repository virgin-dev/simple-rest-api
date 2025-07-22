use actix_web::{get, HttpResponse, Responder};

const HTML: &str = include_str!("frontend.html");

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HTML)
}