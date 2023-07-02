pub mod router;

use axum::{middleware, Router, async_trait, Json};
use axum::body::Bytes;
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use hyper::Body;
use server::{common, system};
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
        //
        // .route_layer(middleware::from_fn(print_request_body))
        //服务健康检查
        .route("/healthz",
               get(|| async { format!("Hello, It works. Version {}",
                                      std::env::var("VERSION").unwrap_or_else(|_| "unknown".to_string()))
               }))
        .route("/anyhow", get(handler));
}

mod auth;

// // middleware that shows how to consume the request body upfront
// async fn print_request_body<B>(request: Request<B>,next: Next<B>,) -> Result<impl IntoResponse, Response> {
//     let request = buffer_request_body(request).await?;
//     Ok(next.run(request).await)
// }
//
// // the trick is to take the request apart, buffer the body, do what you need to do, then put
// // the request back together
// async fn buffer_request_body<T>(request: Request<T>) -> Result<Request<T>, Response> {
//     let (parts, body) = request.into_parts();
//
//     // this wont work if the body is an long running stream
//     let bytes = hyper::body::to_bytes(body)
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
//
//     do_thing_with_request_body(bytes.clone());
//
//     Ok(Request::from_parts(parts, Body::from(bytes)))
// }
//
// fn do_thing_with_request_body(bytes: Bytes) {
//     tracing::debug!(body = ?bytes);
// }
//
// async fn handler(BufferRequestBody(body): BufferRequestBody) {
//     tracing::debug!(?body, "handler received body");
// }
//
// // extractor that shows how to consume the request body upfront
// struct BufferRequestBody(Bytes);
//
// // we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
// #[async_trait]
// impl<S,B> FromRequest<S, B> for BufferRequestBody
//     where
//         S: Send + Sync,
// {
//     type Rejection = Response;
//
//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let body = Bytes::from_request(req, state).await.map_err(|err| err.into_response())?;
//         do_thing_with_request_body(body.clone());
//         Ok(Self(body))
//     }
// }



async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// Make our own error that wraps `anyhow::Error`.
// struct AppError(anyhow::Error);
// struct AppError {
//     inner: anyhow::Error,
//     status: StatusCode,
// }

use serde::Serialize;
#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
// Tell axum how to convert `AppError` into a response.
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//
//         let message = format!("Something went wrong: {}", self.inner);
//         let response = ErrorResponse {
//             code: self.status.as_u16(),
//             message,
//         };
//         (self.status, Json(response)).into_response()
//     }
// }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_code = self.error_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR.as_u16());
        let message = format!("Something went wrong: {}", self.inner);
        let response = ErrorResponse {
            code: error_code,
            message,
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
struct AppError {
    inner: anyhow::Error,
    error_code: Option<u16>,  // 可选状态码
}

impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let e = err.into();
        let code = match e.downcast_ref::<u16>() {
            Some(e) => Some(*e),
            None => None,   // 没有状态码,所以是 `None`
        };
        Self {
            inner: e,
            error_code: code,
        }
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded(u64),
}