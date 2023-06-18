use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::database;
use crate::system::user_domain;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}
// 添加用户
pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let mut domain = user_domain::UserDomain::new(database::pool());
    let response = domain.create_user(payload);
    // if response.is_err() {
    //     return Json(response.err())
    // }
    // (StatusCode::CREATED, Json(response.unwrap()))
}


// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}


pub async fn get_user() -> impl IntoResponse {
    let mut domain = user_domain::UserDomain::new(database::pool());
    let response = domain.get_by_id(1);
    (StatusCode::OK, Json(response.unwrap()))
}

pub async fn update(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let pc = database::pool();
    let mut domain = user_domain::UserDomain::new(pc);
    domain.update();
    (StatusCode::OK, Json(""))
}

#[derive(Deserialize)]
pub struct DeleteCmd {
    pub ids: []int,
}
pub async fn delete(Json(payload): Json<CreateUser>) {}