use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::error::AppError;
use crate::common::{request, response};
use crate::system::config::{service, repo};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::config::request::{AddConfig,PageConfig, UpdateConfig};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::ConfigService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<PageConfig>) -> Result<impl IntoResponse, AppError> {
    let response = service::ConfigService::default().page(r)?;
    Ok(response::success(response))
}
// add
pub async fn add(Validated(r): Validated<AddConfig>) -> Result<impl IntoResponse, AppError>  {
    let response = service::ConfigService::default().add(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateConfig>) -> Result<impl IntoResponse, AppError>  {
    let response = service::ConfigService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::ConfigService::default().delete(r)?;
    Ok(response::success(response))
}
// list
pub async fn list() -> Result<impl IntoResponse, AppError> {
    let response = service::ConfigService::default().list()?;
    Ok(response::success(response))
}