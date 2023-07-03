use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::{request, response};
use crate::system::user::{domain, repo};
use validator::{Validate};
use crate::common::error::AppError;
use crate::common::validator::Validated;
use crate::system::user::model::SysUser;
use crate::system::user::request::{CreateUser, UpdateUser};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = domain::UserDomain::default().get_by_id(id)?;
    Ok(response::success(response))
}
// pub async fn info(Path(id): Path<u64>) -> impl IntoResponse {
// pub async fn info(Path(id): Path<u64>) -> Result<(), AppError> {
//     let mut domain = domain::UserDomain::default();
//     let response = domain.get_by_id(id)?;
//     Ok(())
//     // response::response(response)
//     // (StatusCode::CREATED,Json(response))
// }
// // page
pub async fn page(Json(r): Json<request::Page>) -> Result<impl IntoResponse, AppError> {
    let response = domain::UserDomain::default().page(r)?;
    Ok(response::success(response))
}
// // create
// pub async fn create(Validated(payload): Validated<CreateUser>) -> impl IntoResponse {
//     let repo = repo::UserRepo::new(database::pool());
//     let mut domain = domain::UserDomain::new(repo);
//     let response = domain.create(payload);
//     response::response(response)
// }
// // update
// pub async fn update(Json(cmd): Json<UpdateUser>) -> impl IntoResponse {
//     let repo = repo::UserRepo::new(database::pool());
//     let mut domain = domain::UserDomain::new(repo);
//     let response = domain.update(cmd);
//     response::response(response)
// }
// // delete
// pub async fn delete(Json(cmd): Json<request::Delete>)  -> impl IntoResponse  {
//     let repo = repo::UserRepo::new(database::pool());
//     let mut domain = domain::UserDomain::new(repo);
//     let response = domain.delete(cmd);
//     response::response(response)
// }