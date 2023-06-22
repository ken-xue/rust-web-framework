use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::{request, response};
use crate::system::{user_domain, user_repo};

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}
// info
pub async fn info(Path(id): Path<u64>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.get_by_id(id);
    (StatusCode::CREATED, Json(response::response(response)))
}
// page
pub async fn page(Json(r): Json<request::Page>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    (StatusCode::CREATED, Json(response::response(domain.page(r))))
}
// the input to our `page` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}
// add
pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.create(payload);
    (StatusCode::CREATED, Json(response::response(response)))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}

// update
pub async fn update(Json(cmd): Json<UpdateUser>) -> impl IntoResponse {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.update(cmd);
    (StatusCode::CREATED, Json(response::response(response)))
}

// delete
pub async fn delete(Json(cmd): Json<request::Delete>)  -> impl IntoResponse  {
    let repo = user_repo::UserRepo::new(database::pool());
    let mut domain = user_domain::UserDomain::new(repo);
    let response = domain.delete(cmd);
    (StatusCode::CREATED, Json(response::response(response)))
}