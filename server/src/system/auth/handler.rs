use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use axum::response::IntoResponse;
use crate::{common, database, system};
use crate::common::validator::Validated;
use crate::system::auth::{AuthBody, AuthError, AuthPayload, Claims, CURRENT_USER, KEYS};

// 登录授权
pub async fn login(Validated(payload): Validated<AuthPayload>) -> Result<impl IntoResponse,AuthError> {
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
    // TODO : find all permission
    
    let permissions: Vec<&str> = Vec::new();
    // save permission to cached
    // database::redis::sadd(user.username, permissions.deref()).map_err(|_| AuthError::InvalidToken)?;
    database::redis::sadd(user.username, &["apple", "banana", "orange"]).map_err(|_| AuthError::InvalidToken)?;
    //
    Ok(common::response::success(body))
}


// 退出登录
pub async fn logout() -> Result<impl IntoResponse, AuthError> {
    let username = CURRENT_USER.with(|cell| {
        cell.borrow().clone()
    });
    //清理缓存
    database::redis::del(username.unwrap()).map_err(|_| AuthError::InvalidToken)?;
    Ok(common::response::success(""))
}
