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
        .route("/get",  get(handler::get))
        .route("/:id",  get(handler::info))
        .route("/page",  post(handler::page))
        .route("/update",  put(handler::update))
        .route("/add",  post(handler::add))
        .route("/delete",  delete(handler::delete))
        .route("/password",  post(handler::password))
}