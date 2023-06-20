use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
    (StatusCode::CREATED, Json(response.unwrap()))
}


// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn get(Path(user_id): Path<u64>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.get_by_id(user_id);
    (StatusCode::OK, Json(response.unwrap()))
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
    let _ = domain.update(cmd);
    (StatusCode::OK, Json(""))
}

#[derive(Deserialize)]
pub struct Delete {
    pub ids: Vec<u64>,
}

pub async fn delete(Json(cmd): Json<Delete>)  -> impl IntoResponse  {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    domain.delete(cmd);
    (StatusCode::OK, Json(""))
}