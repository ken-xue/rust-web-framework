use axum::{
    async_trait, Router, extract::TypedHeader, http::StatusCode,
    headers::authorization::{Authorization, Bearer}, http::Request, middleware::{Next},
    response::Response, RequestPartsExt, Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;


use axum::extract::FromRequestParts;
use axum::http::header::HeaderValue;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::routing::post;
use thiserror::Error;
use validator::{Validate};
use crate::{common, database, system};
use crate::common::error::AppError;
use crate::common::validator::Validated;
use crate::system::auth::{AuthBody, AuthError, AuthPayload, Claims, KEYS};


// 登录授权
pub async fn authorize(Validated(payload): Validated<AuthPayload>) -> Result<impl IntoResponse,AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials)
    }
    // Here you can check the user credentials from a database
    let mut domain = system::user::service::UserService::default();
    // check
    let user = domain.authorize(payload.username,payload.password).map_err(|e|AuthError::WrongCredentials)?;
    // token
    let current_time = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user.username.to_owned(),
        exp: (current_time + 60 * 10) as usize, // Mandatory expiry time as UTC timestamp
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
    // Send the authorized token
    let body = AuthBody::new(token);
    // find all permission
    let permissions: Vec<&str> = Vec::new();
    // save permission to cached
    // database::redis::sadd(user.username, permissions.deref()).map_err(|_| AuthError::TokenCreation)?;
    //
    Ok(common::response::success(body))
}


// 退出登录
pub async fn logout() -> Result<impl IntoResponse, AppError> {
    //清理缓存
    Ok(common::response::success(""))
}
