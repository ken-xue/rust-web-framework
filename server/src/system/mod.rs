use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod user_handler;
pub mod user_domain;
pub mod sys_model;

pub fn user_router() -> Router {
    return  Router::new()
        .route("/user",  post(user_handler::create_user))
        .route("/user",  delete(user_handler::delete_user))
        .route("/user",  put(user_handler::update_user))
        .route("/user",  get(user_handler::get_user))
}

//...role
//...menu