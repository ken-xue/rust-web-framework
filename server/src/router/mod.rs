pub mod router;

use axum::{middleware, Router};
use axum::routing::get;
use server::system;
use server::system::user::{user_router};

//初始化各个模块的路由
pub fn initialize() -> Router {
    return Router::new()
        .route("/healthz", get(|| async { "Hello,It works. " }))
        //...
        .nest("/system", user_router())

        .merge(system::auth::get_auth_router())
        //token验证中间件
        .route_layer(middleware::from_fn(system::auth::auth));
}

mod auth;
