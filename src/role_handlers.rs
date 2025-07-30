use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::{debug, info};
use serde_json::json;
use uuid::Uuid;
use crate::models::{CreateRoleDTO, Role, RoleQuery, RolesResponse, UpdateRoleDTO};
use crate::storage::save_roles;

type RoleStore = Mutex<HashMap<String, Role>>;

#[get("/roles")]
pub async fn get_roles(roles: web::Data<RoleStore>, query: web::Query<RoleQuery>) -> impl Responder {
    let roles = roles.lock().unwrap();
    info!("roles : {:?}", roles);

    info!("query: {:?}", query);

    let filter = roles.values().filter(|role| {
        let uuid_match = query.uuid.as_ref().map_or(true, |uuid| {&role.uuid == uuid});
        debug!("uuid_match: {:?}", uuid_match);

        let name_match = query.name.as_ref().map_or(true, |name| {role.name.to_lowercase().contains(name.to_lowercase().as_str())});
        debug!("name_match: {:?}", name_match);

        uuid_match && name_match
    }).cloned().collect::<Vec<_>>();
    info!("filtered_roles: {:?}", filter);

    HttpResponse::Ok().json(RolesResponse { roles: filter })
}

#[post("/roles")]
pub async fn create_role(dto: web::Json<CreateRoleDTO>, roles: web::Data<RoleStore>) -> impl Responder {
    let mut roles = roles.lock().unwrap();
    let uuid = Uuid::new_v4().to_string();
    let role = Role {
        uuid: uuid.clone(),
        name: dto.name.clone(),
        description: dto.description.clone(),
    };
    roles.insert(uuid.clone(), role.clone());
    save_roles(&roles);
    HttpResponse::Created().json(role)
}

#[put("/roles/{uuid}")]
pub async fn update_role(uuid: web::Path<String>, dto: web::Json<UpdateRoleDTO>, roles: web::Data<RoleStore>) -> impl Responder {
    let id = uuid.into_inner();
    let mut roles_guard = roles.lock().unwrap();

    if let Some(role) = roles_guard.get_mut(&id) {
        if let Some(name) = &dto.name {
            role.name = name.clone();
        }
        if let Some(desc) = &dto.description {
            role.description = desc.clone();
        }

        let role_clone = role.clone();
        let roles_copy = roles_guard.clone();
        drop(roles_guard);

        save_roles(&roles_copy);
        HttpResponse::Ok().json(role_clone)
    } else {
        HttpResponse::NotFound().body("Role not found")
    }
}

#[delete("/roles/{uuid}")]
pub async fn delete_role(uuid: web::Path<String>, roles: web::Data<RoleStore>) -> impl Responder {
    let mut roles = roles.lock().unwrap();
    if roles.remove(&uuid.into_inner()).is_some() {
        save_roles(&*roles);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Role not found")
    }
}