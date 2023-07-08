use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod model;
pub mod handler;
pub mod service;
pub mod repo;
mod response;
mod request;

pub fn menu_router() -> Router {
    Router::new()
    .route("/:id",  get(handler::info))
    .route("/page",  post(handler::page))
    .route("/",  put(handler::update))
    .route("/",  post(handler::create))
    .route("/",  delete(handler::delete))
}