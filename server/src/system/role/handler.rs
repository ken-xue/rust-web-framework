use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::error::AppError;
use crate::common::{request, response};
use crate::system::role::{service, repo};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::role::request::{CreateRole, UpdateRole};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::RoleService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<request::Page>) -> Result<impl IntoResponse, AppError> {
    let response = service::RoleService::default().page(r)?;
    Ok(response::success(response))
}
// create
pub async fn create(Validated(r): Validated<CreateRole>) -> Result<impl IntoResponse, AppError>  {
    let response = service::RoleService::default().create(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateRole>) -> Result<impl IntoResponse, AppError>  {
    let response = service::RoleService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::RoleService::default().delete(r)?;
    Ok(response::success(response))
}