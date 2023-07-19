mod handler;

use std::collections::HashSet;
use axum::{Router, extract::TypedHeader, http::StatusCode, headers::authorization::{Authorization, Bearer}, http::Request, middleware::{Next}, response::Response, RequestPartsExt, Json, middleware};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use axum::http::header::HeaderValue;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use jsonwebtoken::errors::ErrorKind;
use lazy_static::lazy_static;
use thiserror::Error;
use validator::{Validate};
use crate::database;

pub const X_REFRESH_TOKEN: &str = "x-refresh-token";
pub const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRATION_BUFFER: i64 = 60 * 10; // JWT令牌过期缓冲时间，单位是秒

thread_local! {
    pub static CURRENT_USER: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}


lazy_static! {
    static ref BLACKLIST: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("/api/auth/logout:get");//接口授权白名单
        set.insert("/api/auth/menus:get");//获取菜单列表
        set
    };
}

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>, next: Next<B>,
) -> Result<Response, AuthError> {
    //验证token
    let token_data = decode::<Claims>(auth.token(), &KEYS.decoding, &Validation::default()).map_err(
        |e| {
            match e.kind() {//see : jsonwebtoken-8.3.0/src/errors.rs
                ErrorKind::InvalidToken => AuthError::InvalidToken,
                ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                &_=> AuthError::Unknown(e.to_string()),
            }
        }
    )?;
    let claims = token_data.claims;
    let username = claims.sub.clone();
    //检查是否有该接口的访问权限
    let path = request.uri().path();
    let method = request.method().to_string().to_lowercase();
    let key = format!("{}:{}", path, method);
    //没有权限直接响应
    if !BLACKLIST.contains(key.clone().as_str()) && !database::redis::exist(username.clone(), key.clone().to_string()) {
        return Err(AuthError::MissingPermission(key));
    }
    //当前线程存入用户信息,方便在更新数据时写入当前操作者是谁
    CURRENT_USER.with(|cell| {
        *cell.borrow_mut() = Some(username);
    });
    let mut response = next.run(request).await;
    // 获取当前时间戳
    let current_time = chrono::Utc::now().timestamp();
    //如果token将要过期则颁布新的token在请求头中
    if claims.exp < (current_time + TOKEN_EXPIRATION_BUFFER) as usize {
        let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::InvalidToken)?;
        let val = HeaderValue::from_str(token.as_str());
        response.headers_mut().append(X_REFRESH_TOKEN, val.unwrap());
    }
    //响应
    Ok(response)
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = "secret";
    Keys::new(secret.as_bytes())
});

pub fn auth_router() -> Router {
    Router::new()
        //退出登录需要取token里面的用户名所以必须加上中间件
        .route("/api/auth/logout", get(handler::logout))
        .route("/api/auth/menus", get(handler::menus))
        //token验证中间件
        .route_layer(middleware::from_fn(auth))
        .route("/api/auth/login", post(handler::login))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "username: {}\n exp: {}", self.sub, self.exp)
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

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (1, String::from("Wrong credentials")),
            AuthError::MissingCredentials => (2, String::from("Missing credentials")),
            AuthError::TokenCreation => (3, String::from("Token creation error")),
            AuthError::InvalidToken => (5, String::from("Invalid token")),
            AuthError::Unknown(m) => (50001, String::from(format!("Unknown Error : {}", m))),
            AuthError::ExpiredToken => (40001, String::from("ExpiredToken")),
            AuthError::MissingPermission(m) => (9, String::from(format!("Missing Permission : {}", m))),
        };
        let bd: Resp = Resp {
            code: status,
            message: error_message.to_string(),
        };
        (StatusCode::OK, Json(bd)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: u16,
    pub message: String,
}

pub struct Keys {
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
pub struct Claims {
    pub sub: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct AuthPayload {
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
    #[error("ExpiredToken")]
    ExpiredToken,
    #[error("Missing Permission Path: {0}")]
    MissingPermission(String),
    #[error("unknown error : {0}")]
    Unknown(String),
}
