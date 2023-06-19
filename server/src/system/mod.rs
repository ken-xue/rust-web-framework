use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod sys_model;
pub mod user_handler;
pub mod user_domain;
pub mod user_repo;

pub fn user_router() -> Router {
    return  Router::new()
        .route("/user",  post(user_handler::create))
        .route("/user",  delete(user_handler::delete))
        .route("/user",  put(user_handler::update))
        .route("/user",  get(user_handler::get))
}

//...role
//...menu