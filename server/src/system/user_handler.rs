use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::database;
use crate::system::user_domain;

// 添加用户
pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut domain = user_domain::User::new(database::pool());
    let user = domain.create_user(payload);
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}


pub async fn get_user() -> impl IntoResponse {
    let mut domain = user_domain::User::new(database::pool());
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