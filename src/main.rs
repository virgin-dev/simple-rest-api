mod handlers;
mod models;
mod storage;
mod frontend;
mod api_docs;

use crate::storage::{load_roles, load_users};
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api_docs::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");

    let users = web::Data::new(Mutex::new(load_users()));
    let roles = web::Data::new(Mutex::new(load_roles()));
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .app_data(roles.clone())
            .service(handlers::get_all_users)
            .service(handlers::get_user)
            .service(handlers::create_user)
            .service(handlers::update_user)
            .service(handlers::delete_user)
            .service(handlers::get_all_roles)
            .service(handlers::get_role)
            .service(handlers::create_role)
            .service(handlers::update_role)
            .service(handlers::delete_role)
            .service(handlers::forbidden)
            .service(frontend::index)
            .service(
                SwaggerUi::new("/docs")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
