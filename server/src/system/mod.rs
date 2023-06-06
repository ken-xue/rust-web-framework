use axum::body::Body;
use axum::http::{Method, Request, Response};
use axum::Router;

pub mod sys_handler;

pub fn get_user_router() -> Router {
    return  Router::new()
        .route("/",  get_users_handler)
        .route("/",  create_user_handler)
        .route("/:id", Method::GET, get_user_handler)
        .route("/:id", Method::PUT, update_user_handler)
        .route("/:id", Method::DELETE, delete_user_handler);
}



async fn get_users_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 处理 GET /users 请求
}

async fn create_user_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 处理 POST /users 请求
}

async fn get_user_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 处理 GET /users/:id 请求
}

async fn update_user_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 处理 PUT /users/:id 请求
}

async fn delete_user_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 处理 DELETE /users/:id 请求
}