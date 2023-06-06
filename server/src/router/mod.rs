pub mod router;

use axum::handler::Handler;
use axum::Router;
use server::system::{get_user_router, sys_handler};

//初始化各个模块的路由
pub fn initialize() -> Router {
    return Router::new()
        .route("/healthz", (|| async { "Hello,It works. " }).into_service())
        //...
        .nest("system", get_user_router);
}
