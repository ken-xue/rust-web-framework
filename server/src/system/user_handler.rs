use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::database;
use crate::system::{user_domain, user_repo};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}
// 添加用户
pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.create(payload);
    // if response.is_err() {
    //     return Json(response.err())
    // }
    (StatusCode::CREATED, Json(response.unwrap()))
}


// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}


pub async fn get() -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.get_by_id(1);
    (StatusCode::OK, Json(response.unwrap()))
}

pub async fn update() -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    domain.update();
    (StatusCode::OK, Json(""))
}

#[derive(Deserialize)]
pub struct DeleteCmd {
    pub ids: Vec<i32>,
}
pub async fn delete(Json(cmd): Json<DeleteCmd>) {

}