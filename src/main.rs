mod handlers;
mod models;
mod storage;
mod frontend;
mod config;
mod user_handlers;
mod role_handlers;

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
            .service(user_handlers::get_users)
            .service(user_handlers::create_user)
            .service(user_handlers::update_user)
            .service(user_handlers::delete_user)
            .service(role_handlers::get_roles)
            .service(role_handlers::create_role)
            .service(role_handlers::update_role)
            .service(role_handlers::delete_role)
            .service(handlers::forbidden)
            .service(frontend::index)
    })
        .bind((config.ip, config.port))?
        .run()
        .await
}
