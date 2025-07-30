use actix_web::{get, HttpResponse, Responder};

#[get("/forbidden")]
pub async fn forbidden() -> impl Responder {HttpResponse::Forbidden().body("Access to this resource is forbidden.")}