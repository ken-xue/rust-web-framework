use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{database};
use crate::common::{request, response};
use crate::system::menu::{domain, repo};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::menu::request::{CreateMenu, UpdateMenu};
// info
pub async fn info(Path(id): Path<u64>) -> impl IntoResponse {
    let repo = repo::MenuRepo::new(database::pool());
    let mut domain = domain::MenuDomain::new(repo);
    let response = domain.get_by_id(id);
    response::response(response)
}
// page
pub async fn page(Json(r): Json<request::Page>) -> impl IntoResponse {
    let repo = repo::MenuRepo::new(database::pool());
    let mut domain = domain::MenuDomain::new(repo);
    let response = domain.page(r);
    response::response(response)
}
// create
pub async fn create(Validated(payload): Validated<CreateMenu>) -> impl IntoResponse {
    let repo = repo::MenuRepo::new(database::pool());
    let mut domain = domain::MenuDomain::new(repo);
    let response = domain.create(payload);
    response::response(response)
}
// update
pub async fn update(Json(cmd): Json<UpdateMenu>) -> impl IntoResponse {
    let repo = repo::MenuRepo::new(database::pool());
    let mut domain = domain::MenuDomain::new(repo);
    let response = domain.update(cmd);
    response::response(response)
}
// delete
pub async fn delete(Json(cmd): Json<request::Delete>)  -> impl IntoResponse  {
    let repo = repo::MenuRepo::new(database::pool());
    let mut domain = domain::MenuDomain::new(repo);
    let response = domain.delete(cmd);
    response::response(response)
}