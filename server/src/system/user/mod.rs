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
        .route("/info",  get(handler::info))
        .route("/:id",  get(handler::get))
        .route("/page",  post(handler::page))
        .route("/",  put(handler::update))
        .route("/",  post(handler::create))
        .route("/",  delete(handler::delete))
}