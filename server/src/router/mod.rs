pub mod router;

use axum::Router;
use axum::routing::get;
use server::system::user::{user_router};

//初始化各个模块的路由
pub fn initialize() -> Router {
    return Router::new()
        .route("/healthz", get(|| async { "Hello,It works. " }))
        //...
        .nest("/system", user_router())
        .merge(auth::get_auth_router());
}

mod auth;
