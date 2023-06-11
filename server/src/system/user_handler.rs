use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::database;
use crate::system::user_domain;

// 添加用户
pub async fn create_user(
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
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}


pub async fn get_user() -> impl IntoResponse {
    let pc = database::pool();
    let mut domain = user_domain::User::new(pc);
    let response = domain.get_user_by_id(1);
    (StatusCode::OK, Json(response.unwrap()))
}

pub async fn update_user() -> impl IntoResponse {
    let pc = database::pool();
    let mut domain = user_domain::User::new(pc);
    domain.update_user();
    (StatusCode::OK, Json(""))
}

pub async fn delete_user() {}