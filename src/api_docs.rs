use utoipa::OpenApi;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::get_all_users,
        crate::handlers::get_user,
        crate::handlers::create_user,
        crate::handlers::update_user,
        crate::handlers::delete_user,
        crate::handlers::get_all_roles,
        crate::handlers::get_role,
        crate::handlers::create_role,
        crate::handlers::update_role,
        crate::handlers::delete_role,
    ),
    components(
        schemas(User, CreateUserDTO, UpdateUserDTO, Role, CreateRoleDTO, UpdateRoleDTO)
    ),
    tags(
        (name = "User API", description = "Управление пользователями"),
        (name = "Role API", description = "Управление ролями")
    )
)]
pub struct ApiDoc;