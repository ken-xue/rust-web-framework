use axum::Router;
use axum::routing::{get, post, put,delete};

pub mod model;
pub mod handler;
pub mod domain;
pub mod repo;
mod response;
mod request;

pub fn menu_router() -> Router {
    Router::new()
    .route("/menu/:id",  get(handler::info))
    .route("/menu",  post(handler::page))
    .route("/menu",  put(handler::update))
    .route("/menu",  post(handler::create))
    .route("/menu",  delete(handler::delete))
}