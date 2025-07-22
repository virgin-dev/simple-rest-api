use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{CreateRoleDTO, CreateUserDTO, Role, UpdateRoleDTO, UpdateUserDTO, User};
use crate::storage::{save_roles, save_users};

type UserStore = Mutex<HashMap<String, User>>;
type RoleStore = Mutex<HashMap<String, Role>>;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Список пользователей", body = [User])
    )
)]
#[get("/users")]
pub async fn get_all_users(users: web::Data<UserStore>) -> impl Responder {
    let users = users.lock().unwrap();
    HttpResponse::Ok().json(users.values().cloned().collect::<Vec<_>>())
}

#[utoipa::path(
    get,
    path = "/users/{uuid}",
    params(("uuid" = String, Path, description = "UUID пользователя")),
    responses(
        (status = 200, description = "Пользователь найден", body = User),
        (status = 404, description = "Пользователь не найден")
    )
)]
#[get("/users/{uuid}")]
pub async fn get_user(uuid: web::Path<String>, users: web::Data<UserStore>) -> impl Responder {
    let users = users.lock().unwrap();
    match users.get(&uuid.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDTO,
    responses((status = 201, description = "Пользователь создан", body = User))
)]
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

#[utoipa::path(
    put,
    path = "/users/{uuid}",
    request_body = UpdateUserDTO,
    params(("uuid" = String, Path, description = "UUID пользователя")),
    responses(
        (status = 200, description = "Пользователь обновлён", body = User),
        (status = 404, description = "Пользователь не найден")
    )
)]
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



#[utoipa::path(
    delete,
    path = "/users/{uuid}",
    params(("uuid" = String, Path, description = "UUID пользователя")),
    responses(
        (status = 204, description = "Пользователь удалён"),
        (status = 404, description = "Пользователь не найден")
    )
)]
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

#[utoipa::path(
    get,
    path = "/roles",
    responses((status = 200, description = "Список всех ролей", body = [Role]))
)]
#[get("/roles")]
pub async fn get_all_roles(roles: web::Data<RoleStore>) -> impl Responder {
    let roles = roles.lock().unwrap();
    HttpResponse::Ok().json(roles.values().cloned().collect::<Vec<_>>())
}

#[utoipa::path(
    get,
    path = "/roles/{uuid}",
    params(("uuid" = String, Path, description = "UUID роли")),
    responses(
        (status = 200, description = "Роль найдена", body = Role),
        (status = 404, description = "Роль не найдена")
    )
)]
#[get("/roles/{uuid}")]
pub async fn get_role(uuid: web::Path<String>, roles: web::Data<RoleStore>) -> impl Responder {
    let roles = roles.lock().unwrap();
    match roles.get(&uuid.into_inner()) {
        Some(role) => HttpResponse::Ok().json(role),
        None => HttpResponse::NotFound().body("Role not found"),
    }
}

#[utoipa::path(
    post,
    path = "/roles",
    request_body = CreateRoleDTO,
    responses((status = 201, description = "Роль создана", body = Role))
)]
#[post("/roles")]
pub async fn create_role(
    dto: web::Json<CreateRoleDTO>,
    roles: web::Data<RoleStore>,
) -> impl Responder {
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

#[utoipa::path(
    put,
    path = "/roles/{uuid}",
    request_body = UpdateRoleDTO,
    params(("uuid" = String, Path, description = "UUID роли")),
    responses(
        (status = 200, description = "Роль обновлена", body = Role),
        (status = 404, description = "Роль не найдена")
    )
)]
#[put("/roles/{uuid}")]
pub async fn update_role(
    uuid: web::Path<String>,
    dto: web::Json<UpdateRoleDTO>,
    roles: web::Data<RoleStore>,
) -> impl Responder {
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

#[utoipa::path(
    delete,
    path = "/roles/{uuid}",
    params(("uuid" = String, Path, description = "UUID роли")),
    responses(
        (status = 204, description = "Роль удалена"),
        (status = 404, description = "Роль не найдена")
    )
)]
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
pub async fn forbidden() -> impl Responder {
    HttpResponse::Forbidden().body("Access to this resource is forbidden.")
}