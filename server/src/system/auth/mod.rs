mod handler;
mod domain;

use std::borrow::Borrow;
use axum::{
    async_trait,Router, extract::TypedHeader, http::StatusCode,
    headers::authorization::{Authorization, Bearer}, http::Request, middleware::{Next},
    response::Response, routing::get, RequestPartsExt, Json
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::ops::Deref;
use axum::extract::FromRequestParts;
use axum::http::header::HeaderValue;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::routing::post;
use crate::system;

pub const X_REFRESH_TOKEN: &str = "x-refresh-token";

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,next: Next<B>,
) -> Result<Response, StatusCode> {
    //验证token
    let token_data = decode::<Claims>(auth.token(), &KEYS.decoding, &Validation::default()).map_err(|_| AuthError::InvalidToken);
    if token_is_valid(auth.token()) {
        let mut response = next.run(request).await;
        //如果token将要过期则颁布新的token在请求头中refresh
        let val = HeaderValue::from_static("hello");
        response.headers_mut().append(X_REFRESH_TOKEN, val);
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

 fn token_is_valid(token: &str) -> bool {
    // let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default()).map_err(|_| AuthError::InvalidToken);
    return true
}

///////////////

static KEYS: Lazy<Keys> = Lazy::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = "secret";
    Keys::new(secret.as_bytes())
});

pub fn get_auth_router() -> Router {
    let router = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize));
    router
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!("Welcome to the protected area :)\nYour data:\n{}", claims))
}

// 授权
async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    // 查询用户
    let domain = system::user::domain::USER_DOMAIN.lock().unwrap();
    // domain.get_by_id()
    if payload.username != "foo" || payload.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        // exp: 2000000000, // May 2033
        exp: 2, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = AuthError;

    // 中间件拦截
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // 解码拿到token中的用户信息
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
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
        let body = Json(json!({"error": error_message,}));
        (status, body).into_response()
    }
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
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    username: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
