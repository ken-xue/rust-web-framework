use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database, util};
use crate::system::{user_domain, user_repo};


// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn get(Path(id): Path<u64>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.get_by_id(id);
    (StatusCode::CREATED, Json(util::response(response)))
}

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
    (StatusCode::CREATED, Json(util::response(response)))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}

pub async fn update(Json(cmd): Json<UpdateUser>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.update(cmd);
    (StatusCode::CREATED, Json(util::response(response)))
}

#[derive(Deserialize)]
pub struct Delete {
    pub ids: Vec<u64>,
}

pub async fn delete(Json(cmd): Json<Delete>)  -> impl IntoResponse  {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.delete(cmd);
    (StatusCode::CREATED, Json(util::response(response)))
}