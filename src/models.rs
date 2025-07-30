use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct User {
    pub uuid: String,
    pub full_name: String,
    pub is_blocked: bool,
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserDTO {
    pub full_name: String,
    pub is_blocked: bool,
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserDTO {
    pub full_name: Option<String>,
    pub is_blocked: Option<bool>,
    pub roles: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub uuid: String,
    pub full_name: String,
    pub is_blocked: bool,
    pub roles: Vec<RoleReference>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserQuery {
    pub uuid: Option<String>,
    pub full_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Role {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRoleDTO {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRoleDTO {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct RolesResponse {
    pub roles: Vec<Role>,
}

#[derive(Serialize, Debug)]
pub struct RoleReference {
    pub(crate) uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleQuery {
    pub uuid: Option<String>,
    pub name: Option<String>,
}