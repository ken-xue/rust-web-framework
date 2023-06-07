use axum::body::Body;
use axum::http::{Method, Request, Response, StatusCode};
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use diesel::delete;

mod user_handler;
mod user_domain;

pub fn user_router() -> Router {
    return  Router::new()
        .route("/user",  get(user_handler::get_user))
        .route("/user",  post(user_handler::create_user))
        .route("/user",  put(user_handler::update_user))
        .route("/user",  delete(user_handler::delete_user))
}
