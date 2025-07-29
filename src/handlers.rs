use actix_web::web::Query;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{CreateRoleDTO, CreateUserDTO, Role, RoleReference, RolesResponse, UpdateRoleDTO, UpdateUserDTO, User, UserResponse, UsersResponse};
use crate::storage::{save_roles, save_users};

type UserStore = Mutex<HashMap<String, User>>;
type RoleStore = Mutex<HashMap<String, Role>>;

#[get("/users")]
pub async fn get_all_users(users: web::Data<UserStore>) -> impl Responder {
    let user_list = users.lock().unwrap().values().map(|user| {
        UserResponse {
            uuid: user.uuid.clone(),
            full_name: user.full_name.clone(),
            is_blocked: user.is_blocked,
            roles: user.roles.iter().map(|rid| RoleReference { uuid: rid.clone() }).collect(),
        }
    }).collect::<Vec<_>>();

    HttpResponse::Ok().json(UsersResponse { users: user_list })

}

#[get("/users/{uuid}")]
pub async fn get_user(uuid: web::Path<String>, users: web::Data<UserStore>) -> impl Responder {
    let users = users.lock().unwrap();
    let id = uuid.into_inner();

    match users.get(&id) {
        Some(user) => {
            let response_user = UserResponse {
                uuid: user.uuid.clone(),
                full_name: user.full_name.clone(),
                is_blocked: user.is_blocked,
                roles: user.roles.iter().map(|rid| RoleReference {
                    uuid: rid.clone()
                }).collect(),
            };

            HttpResponse::Ok().json(UsersResponse {
                users: vec![response_user],
            })
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[get("/users/search")]
pub async fn get_user_search(query: Query<HashMap<String,String>>, users: web::Data<UserStore>) -> impl Responder {
    let users = users.lock().unwrap();
    if let Some(query) = query.get("query") {
        let lowered = query.to_lowercase();
        let matched: Vec<_> = users.values().filter(|u| u.uuid.to_lowercase().contains(&lowered) || u.full_name.to_lowercase().contains(&lowered)).cloned().collect();

        let response = json!({ "users": &matched });
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
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

#[get("/roles")]
pub async fn get_all_roles(roles: web::Data<RoleStore>) -> impl Responder {
    let roles = roles.lock().unwrap();
    let response = RolesResponse {
        roles: roles.values().cloned().collect(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/roles/{uuid}")]
pub async fn get_role(uuid: web::Path<String>, roles: web::Data<RoleStore>) -> impl Responder {
    let roles = roles.lock().unwrap();
    match roles.get(&uuid.into_inner()) {
        Some(role) => {
            let response = json!({ "roles": [&role] });
            HttpResponse::Ok().json(response)
        },
        None => HttpResponse::NotFound().body("Role not found"),
    }
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
#[get("/forbidden")]
pub async fn forbidden() -> impl Responder {HttpResponse::Forbidden().body("Access to this resource is forbidden.")}