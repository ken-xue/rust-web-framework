use axum::body::Body;
use axum::http::{Method, Request, Response, StatusCode};
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use diesel::delete;
use serde::{Deserialize, Serialize};

pub mod sys_handler;

pub fn user_router() -> Router {
    return  Router::new()
        .route("/user",  get(get_user))
        .route("/user",  post(create_user))
        .route("/user",  put(update_user))
        .route("/user",  delete(delete_user))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}