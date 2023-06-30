pub mod router;

use axum::{middleware, Router};
use axum::routing::get;
use server::system;
use server::system::user::{user_router};

//初始化各个模块的路由
pub fn initialize() -> Router {
    return Router::new()
        //...
        .nest("/v1/system", user_router())
        //...

        //token验证中间件
        .route_layer(middleware::from_fn(system::auth::auth))
        //获取token接口
        .merge(system::auth::auth_router())
        //服务健康检查
        .route("/healthz",
               get(|| async { format!("Hello, It works. Version {}",
                                      std::env::var("VERSION").unwrap_or_else(|_| "unknown".to_string()))
               }))
}

mod auth;
