use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{database};
use crate::common::error::AppError;
use crate::common::{request, response};
use crate::system::menu::{service, repo};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::menu::request::{AddMenu, PageMenu, UpdateMenu};

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::MenuService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<PageMenu>) -> Result<impl IntoResponse, AppError> {
    let response = service::MenuService::default().page(r)?;
    Ok(response::success(response))
}
// add
pub async fn add(Validated(r): Validated<AddMenu>) -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().add(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateMenu>) -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().delete(r)?;
    Ok(response::success(response))
}

// list 获取当前用户的菜单权限
pub async fn list() -> Result<impl IntoResponse, AppError> {
    let menus = service::MenuService::default().list()?;
    Ok(response::success(menus))
}