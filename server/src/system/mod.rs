use axum::Router;

pub mod user;
pub mod role;
pub mod menu;
pub mod auth;
pub mod dept;
pub mod config;

pub fn system_router() -> Router {
    Router::new()
        .nest("/user",  user::user_router())
        .nest("/menu",  menu::menu_router())
        .nest("/role",  role::role_router())
        .nest("/dept",  dept::dept_router())
        .nest("/config",  config::config_router())
}