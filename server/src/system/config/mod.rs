use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod model;
pub mod handler;
pub mod service;
pub mod repo;
pub mod response;
pub mod request;

pub fn config_router() -> Router {
    Router::new()
    .route("/:id",  get(handler::info))
    .route("/page",  post(handler::page))
    .route("/list",  post(handler::list))
    .route("/update",  put(handler::update))
    .route("/add",  post(handler::add))
    .route("/delete",  delete(handler::delete))
}