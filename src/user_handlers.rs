use crate::models::{CreateUserDTO, RoleReference, UpdateUserDTO, User, UserQuery, UserResponse, UsersResponse};
use crate::storage::save_users;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

type UserStore = Mutex<HashMap<String, User>>;

#[get("/users")]
pub async fn get_users(users: web::Data<UserStore>, query: web::Query<UserQuery>) -> impl Responder {
    let users = users.lock().unwrap();
    info!("users: {:?}", users);

    info!("query: {:?}", query);

    let filter = users.values().filter(|user| {
        let uuid_match = query.uuid.as_ref().map_or(true, |uuid| {&user.uuid == uuid});
        debug!("uuid match: {:?}", uuid_match);

        let name_match = query.full_name.as_ref().map_or(true, |full_name| user.full_name.to_lowercase().contains(full_name.to_lowercase().as_str()));
        debug!("name match: {:?}", name_match);

        uuid_match && name_match
    }).map(|user| UserResponse{
        uuid: user.uuid.clone(),
        full_name: user.full_name.clone(),
        is_blocked: user.is_blocked,
        roles: user.roles.iter().map(|rid| RoleReference {
            uuid: rid.clone(),
        }).collect(),
    }).collect::<Vec<_>>();
    info!("filtered_users: {:?}", filter);
    HttpResponse::Ok().json(UsersResponse {users: filter})
}

#[post("/users")]
pub async fn create_user(dto: web::Json<CreateUserDTO>, users: web::Data<UserStore>) -> impl Responder {
    let mut users = users.lock().unwrap();
    let uuid = Uuid::new_v4().to_string();
    let user = User {
        uuid: uuid.clone(),
        full_name: dto.full_name.clone(),
        is_blocked: dto.is_blocked,
        roles: dto.roles.clone(),
    };
    users.insert(uuid.clone(), user.clone());
    save_users(&*users);
    HttpResponse::Created().json(user)
}

#[put("/users/{uuid}")]
pub async fn update_user(uuid: web::Path<String>, dto: web::Json<UpdateUserDTO>, users: web::Data<UserStore>) -> impl Responder {
    let id = uuid.into_inner();
    let mut users_guard = users.lock().unwrap();

    if let Some(user) = users_guard.get_mut(&id) {
        if let Some(name) = &dto.full_name {
            user.full_name = name.clone();
        }
        if let Some(b) = dto.is_blocked {
            user.is_blocked = b;
        }
        if let Some(roles) = &dto.roles {
            user.roles = roles.clone();
        }

        let user_clone = user.clone();

        let users_copy = users_guard.clone();
        drop(users_guard);

        crate::storage::save_users(&users_copy);

        HttpResponse::Ok().json(user_clone)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[delete("/users/{uuid}")]
pub async fn delete_user(uuid: web::Path<String>, users: web::Data<UserStore>) -> impl Responder {
    let mut users = users.lock().unwrap();
    if users.remove(&uuid.into_inner()).is_some() {
        save_users(&*users);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}