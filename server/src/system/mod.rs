use axum::Router;

pub mod user;
pub mod role;
pub mod menu;
pub mod auth;
pub mod dept;

pub fn system_router() -> Router {
    Router::new()
        .nest("/user",  user::user_router())
        .nest("/menu",  menu::menu_router())
        .nest("/role",  role::role_router())
        .nest("/dept",  dept::dept_router())
}