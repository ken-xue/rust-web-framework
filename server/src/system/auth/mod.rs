mod handler;
mod domain;

use axum::{
    async_trait, Router, extract::TypedHeader, http::StatusCode,
    headers::authorization::{Authorization, Bearer}, http::Request, middleware::{Next},
    response::Response, routing::get, RequestPartsExt, Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use anyhow::bail;
use axum::extract::FromRequestParts;
use axum::http::header::HeaderValue;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::routing::post;
use thiserror::Error;
use validator::{Validate};
use crate::{common, database, system};
use crate::common::validator::Validated;
use crate::system::user::repo;

pub const X_REFRESH_TOKEN: &str = "x-refresh-token";
pub const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRATION_BUFFER: i64 = 60 * 10; // JWT令牌过期缓冲时间，单位是秒

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>, next: Next<B>,
) -> Result<Response, AuthError> {
    //验证token
    let token_data = decode::<Claims>(auth.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;
    let mut response = next.run(request).await;
    let claims = token_data.claims;
    // 获取当前时间戳
    let current_time = chrono::Utc::now().timestamp();
    //如果token将要过期则颁布新的token在请求头中
    if claims.exp < (current_time + TOKEN_EXPIRATION_BUFFER) as usize {
        let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::InvalidToken)?;
        let val = HeaderValue::from_str((TOKEN_TYPE.to_string() + " " + token.as_str()).as_str());
        response.headers_mut().append(X_REFRESH_TOKEN, val.unwrap());
    }
    Ok(response)
}

///////////////

static KEYS: Lazy<Keys> = Lazy::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = "secret";
    Keys::new(secret.as_bytes())
});

pub fn auth_router() -> Router {
    Router::new().route("/authorize", post(authorize))
}

// 授权
async fn authorize(Validated(payload): Validated<AuthPayload>) -> Result<impl IntoResponse,AuthError> {
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
    Ok(common::response::success(body))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
        write!(f, "Email: {}\nCompany: {}", self.sub, "")
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: TOKEN_TYPE.to_string(),
        }
    }
}

// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
//     where
//         S: Send + Sync,
// {
//     type Rejection = AuthError;
//
//     // 中间件拦截
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         // Extract the token from the authorization header
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken)?;
//         // 解码拿到token中的用户信息
//         let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//
//         Ok(token_data.claims)
//     }
// }

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let bd: Resp = Resp {
            code: status.as_u16(),
            message: error_message.to_string(),
        };
        (status, Json(bd)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: u16,
    pub message: String,
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Validate, Deserialize)]
struct AuthPayload {
    #[validate(length(min = 1, message = "username can not be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "password can not be empty"))]
    pub password: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("WrongCredentials")]
    WrongCredentials,
    #[error("MissingCredentials")]
    MissingCredentials,
    #[error("TokenCreation")]
    TokenCreation,
    #[error("InvalidToken")]
    InvalidToken,
}
