use axum::extract::Path;

use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::{request, response};
use crate::system::user::{domain, repo};
use validator::{Validate, ValidationError};//see:https://github.com/Keats/validator


#[derive(Debug,Serialize)]
pub struct User {
    id: u64,
    username: String,
}
// info
pub async fn info(Path(id): Path<u64>) -> impl IntoResponse {
    let repo = repo::UserRepo::new(database::pool());
    let mut domain = domain::UserDomain::new(repo);
    let response = domain.get_by_id(id);
    response::response(response)
}
// page
pub async fn page(Json(r): Json<request::Page>) -> impl IntoResponse {
    let repo = repo::UserRepo::new(database::pool());
    let mut domain = domain::UserDomain::new(repo);
    let response = domain.page(r);
    response::response(response)
}
// the input to our `page` handler
#[derive(Debug, Validate,Deserialize)]
pub struct CreateUser {
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub account: String,
    pub password: String,
}

// 新增
pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    //验证
    if let Err(err) = payload.validate() {
        // return response::error(Box::try_from(err).unwrap());
    }
    //存储
    let repo = repo::UserRepo::new(database::pool());
    let mut domain = domain::UserDomain::new(repo);
    let response = domain.create(payload);
    response::response(response)
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
    let repo = repo::UserRepo::new(database::pool());
    let mut domain = domain::UserDomain::new(repo);
    let response = domain.update(cmd);
    response::response(response)
}

// delete
pub async fn delete(Json(cmd): Json<request::Delete>)  -> impl IntoResponse  {
    let repo = repo::UserRepo::new(database::pool());
    let mut domain = domain::UserDomain::new(repo);
    let response = domain.delete(cmd);
    response::response(response)
}