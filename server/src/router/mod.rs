pub mod router;

use axum::handler::Handler;
use axum::Router;

//初始化各个模块的路由
pub fn initialize() -> Router {
    let app = Router::new();
    let app = app.route("/healthz", (|| async { "Hello,It works. " }).into_service());
    return app;
}
