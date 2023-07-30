use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use crate::{common};
use crate::common::{request, response};
use crate::system::user::{service};
use crate::common::error::AppError;
use crate::common::validator::Validated;

use crate::system::user::request::{AddUser, ExistUsername, PageUser, UpdatePassword, UpdateUser};

// info
pub async fn get() -> Result<impl IntoResponse, AppError> {
    let response = service::UserService::default().get_by_username(common::current_username())?;
    Ok(response::success(response))
}
// get
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::UserService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<PageUser>) -> Result<impl IntoResponse, AppError> {
    let response = service::UserService::default().page(r)?;
    Ok(response::success(response))
}
// add
pub async fn add(Validated(r): Validated<AddUser>) -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().add(r)?;
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

// update password
pub async fn password(Json(r): Json<UpdatePassword>) -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().password(r)?;
    Ok(response::success(response))
}

// 判断用户是否已经存在
pub async fn exist(Json(r): Json<ExistUsername>) -> Result<impl IntoResponse, AppError>  {
    let response = service::UserService::default().exist(r)?;
    Ok(response::success(response))
}