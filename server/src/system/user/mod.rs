use axum::Router;
use axum::routing::{get, post, put, delete};

pub mod model;
pub mod handler;
pub mod service;
pub mod repo;
pub mod response;
pub mod request;

pub fn user_router() -> Router {
    Router::new()
        .route("/user/info",  get(handler::info))
        .route("/user/:id",  get(handler::get))
        .route("/users",  post(handler::page))
        .route("/user",  put(handler::update))
        .route("/user",  post(handler::create))
        .route("/user",  delete(handler::delete))
}