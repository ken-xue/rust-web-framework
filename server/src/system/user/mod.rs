use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod model;
pub mod handler;
pub mod domain;
pub mod repo;

pub fn user_router() -> Router {
    return  Router::new()
        .route("/user/:id",  get(handler::info))
        .route("/users",  post(handler::page))
        .route("/user",  put(handler::update))
        .route("/user",  post(handler::create))
        .route("/user",  delete(handler::delete))
}