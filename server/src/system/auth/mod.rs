mod handler;

use axum::{
    async_trait, Router, extract::TypedHeader, http::StatusCode,
    headers::authorization::{Authorization, Bearer}, http::Request, middleware::{Next},
    response::Response, RequestPartsExt, Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use axum::extract::FromRequestParts;
use axum::http::header::HeaderValue;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use thiserror::Error;
use validator::{Validate};

pub const X_REFRESH_TOKEN: &str = "x-refresh-token";
pub const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRATION_BUFFER: i64 = 60 * 10; // JWT令牌过期缓冲时间，单位是秒

thread_local! {
    pub static CURRENT_USER: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>, next: Next<B>,
) -> Result<Response, AuthError> {
    //验证token
    let token_data = decode::<Claims>(auth.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;
    let claims = token_data.claims;
    let username = claims.sub.clone();
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
        // let val = HeaderValue::from_str((TOKEN_TYPE.to_string() + " " + token.as_str()).as_str());
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
        .route("/api/authorize", post(handler::authorize))
        .route("/api/logout", get(handler::logout))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
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

// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
//     where
//         S: Send + Sync,
// {
//     type Rejection = AuthError;
//
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         // Extract the token from the authorization header
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken)?;
//         // Decode the user data
//         let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//
//         Ok(token_data.claims)
//     }
// }

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
}
