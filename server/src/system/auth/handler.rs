use std::collections::{HashSet};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use axum::response::IntoResponse;
use crate::{database, system};
use crate::common::error::AppError;
use crate::common::response;
use crate::common::validator::Validated;
use crate::system::auth::{AuthBody, AuthError, AuthPayload, Claims, CURRENT_USER, KEYS};
use crate::system::menu::response::MenuResponse;
use crate::system::{menu, user};

// 登录授权
pub async fn login(Validated(payload): Validated<AuthPayload>) -> Result<impl IntoResponse, AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    let mut domain = system::user::service::UserService::default();
    // check
    let user = domain.authorize(payload.username, payload.password).map_err(|e| AuthError::WrongCredentials)?;
    // token
    let current_time = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user.username.to_owned(),
        exp: (current_time + 60 * 1000) as usize, // Mandatory expiry time as UTC timestamp
        // exp: (current_time + 10) as usize, // Mandatory expiry time as UTC timestamp
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
    // Send the authorized token
    let body = AuthBody::new(token);
    // distinct all permission
    let mut permissions_set: HashSet<String> = HashSet::new();
    // 遍历user.roles，中的每一个 menus 的 perms字段加入permissions
    if let Some(roles) = &user.roles {
        for role in roles.iter() {
            if let Some(menus) = &role.menus {
                for menu in menus {
                    if let Some(permission) = &menu.permission {
                        let apis = permission.split(",");
                        for key in apis {
                            permissions_set.insert(key.to_string());
                        }
                    }
                }
            }
        }
    }
    let permissions: Vec<&str> = permissions_set.iter().map(|s| s.as_str()).collect();
    // save permission to cached
    database::redis::sadd(user.username, permissions.as_slice()).map_err(|e| AuthError::Unknown(e.to_string()))?;
    // response
    Ok(response::success(body))
}

// 退出登录
pub async fn logout() -> Result<impl IntoResponse, AuthError> {
    let username = CURRENT_USER.with(|cell| {
        cell.borrow().clone()
    });
    //清理缓存
    database::redis::del(username.unwrap()).map_err(|_| AuthError::InvalidToken)?;
    Ok(response::success(""))
}

// 获取当前用户的菜单权限
pub async fn menus() -> Result<impl IntoResponse, AppError> {
    let username = CURRENT_USER.with(|cell| {
        cell.borrow().clone()
    });
    // 查询角色和菜单
    let user = user::service::UserService::default().get_by_username(username.unwrap())?;
    let mut menu_set: HashSet<MenuResponse> = HashSet::new();
    if let Some(roles) = &user.roles {
        for role in roles.iter() {
            if let Some(menus) = &role.menus {
                menu_set.extend(menus.iter().cloned());
            }
        }
    }
    let menus: Vec<MenuResponse> = menu_set.into_iter().collect();
    // 构建成菜单树
    let tree_menus = menu::service::MenuService::default().tree(menus)?;
    // 响应
    Ok(response::success(tree_menus))
}