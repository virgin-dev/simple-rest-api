mod handlers;
mod models;
mod storage;
mod frontend;
mod config;

use crate::config::Config;
use crate::storage::{load_roles, load_users};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::{Builder, Env};
use log::{debug, info, LevelFilter};
use std::fs;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config.toml");

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level.as_str())).init();

    info!("Server running on http://{}:{}", &config.ip, &config.port);

    let users = web::Data::new(Mutex::new(load_users()));

    let roles = web::Data::new(Mutex::new(load_roles()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
            .service(handlers::get_user_search)
    })
        .bind((config.ip, config.port))?
        .run()
        .await
}
