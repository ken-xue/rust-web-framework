pub mod router;

use axum::{middleware, Router};
use axum::routing::get;
use server::{system};

//初始化各个模块的路由
pub fn initialize() -> Router {
    Router::new()
        .nest("/api/v1", Router::new()
            .nest("/system", system::system_router())
              //...
              // .nest("/xxxxx", system::system_router())
        )
        //token验证中间件
        .route_layer(middleware::from_fn(system::auth::auth))
        //获取token接口
        .merge(system::auth::auth_router())
        // .route_layer(middleware::from_fn(print_request_body))
        //服务健康检查
        .route("/healthz",
               get(|| async { format!("Hello, It works. Version {}",
                                      std::env::var("VERSION").unwrap_or_else(|_| "unknown".to_string()))
               }))
    // return Router::new()
    //     //...
    //     .nest("/api/system", system::system_router())
    //     //...
    //
    //     //token验证中间件
    //     .route_layer(middleware::from_fn(system::auth::auth))
    //     //获取token接口
    //     .merge(system::auth::auth_router())
    //     //
    //     // .route_layer(middleware::from_fn(print_request_body))
    //     //服务健康检查
    //     .route("/healthz",
    //            get(|| async { format!("Hello, It works. Version {}",
    //                                   std::env::var("VERSION").unwrap_or_else(|_| "unknown".to_string()))
    //            }))
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
//     // this wont work if the body is an long running stream
//     let bytes = hyper::body::to_bytes(body)
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
//     do_thing_with_request_body(bytes.clone());
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
//         S: Send + Sync, B: axum::body::HttpBody
// {
//     type Rejection = Response;
//
//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let body = Bytes::from_request(req, state).await.map_err(|err| err.into_response())?;
//         do_thing_with_request_body(body.clone());
//         Ok(Self(body))
//     }
// }