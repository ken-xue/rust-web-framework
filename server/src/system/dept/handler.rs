use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::error::AppError;
use crate::common::{request, response};
use crate::system::dept::{service, repo};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::dept::request::{AddDept,PageDept, UpdateDept};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::DeptService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<PageDept>) -> Result<impl IntoResponse, AppError> {
    let response = service::DeptService::default().page(r)?;
    Ok(response::success(response))
}
// add
pub async fn add(Validated(r): Validated<AddDept>) -> Result<impl IntoResponse, AppError>  {
    let response = service::DeptService::default().add(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateDept>) -> Result<impl IntoResponse, AppError>  {
    let response = service::DeptService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::DeptService::default().delete(r)?;
    Ok(response::success(response))
}
// list
pub async fn list() -> Result<impl IntoResponse, AppError> {
    let response = service::DeptService::default().list()?;
    Ok(response::success(response))
}