use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::info;
use crate::models::{User, Role};

const USERS_FILE: &str = "users.json";
const ROLES_FILE: &str = "roles.json";

pub fn load_users() -> HashMap<String, User> {
    load_or_initialize(USERS_FILE, default_users)
}

pub fn save_users(users: &HashMap<String, User>) {
    save_map(USERS_FILE, users);
}

pub fn load_roles() -> HashMap<String, Role> {
    info!("Start load roles");
    load_or_initialize(ROLES_FILE, default_roles)
}

pub fn save_roles(roles: &HashMap<String, Role>) {
    save_map(ROLES_FILE, roles);
}

fn load_or_initialize<T: serde::de::DeserializeOwned + serde::Serialize>(
    path: &str,
    default_fn: fn() -> HashMap<String, T>,
) -> HashMap<String, T> {
    if !Path::new(path).exists() {
        let data = default_fn();
        save_map(path, &data);
        return data;
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    if content.trim().is_empty() {
        let data = default_fn();
        save_map(path, &data);
        return data;
    }

    serde_json::from_str(&content).unwrap_or_else(|_| {
        let data = default_fn();
        save_map(path, &data);
        data
    })
}

fn save_map<T: serde::Serialize>(path: &str, data: &HashMap<String, T>) {
    let json = serde_json::to_string_pretty(data).unwrap();
    let _ = fs::write(path, json);
}

fn default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "user-1".into(),
        User {
            uuid: "user-1".into(),
            full_name: "Alice Admin".into(),
            is_blocked: false,
            roles: vec!["admin".into()],
        },
    );
    users
}

fn default_roles() -> HashMap<String, Role> {
    let mut roles = HashMap::new();
    roles.insert(
        "admin".into(),
        Role {
            uuid: "admin".into(),
            name: "Admin".into(),
            description: "Administrator role with full access.".into(),
        },
    );
    roles.insert(
        "users".into(),
        Role {
            uuid: "user".into(),
            name: "User".into(),
            description: "User role with base access.".into(),
        },
    );
    roles
}
