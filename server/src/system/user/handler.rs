use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::{Validate};
use crate::{database};
use crate::common::{request, response};
use crate::system::user::{service, repo};
use crate::common::error::AppError;
use crate::common::validator::Validated;
use crate::system::user::model::SysUser;
use crate::system::user::request::{CreateUser, UpdateUser};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::UserService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<request::Page>) -> Result<impl IntoResponse, AppError> {
    let response = service::UserService::default().page(r)?;
    Ok(response::success(response))
}
// create
pub async fn create(Validated(r): Validated<CreateUser>) -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().create(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateUser>) -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().delete(r)?;
    Ok(response::success(response))
}