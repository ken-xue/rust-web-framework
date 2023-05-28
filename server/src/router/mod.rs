pub mod router;

use std::convert::Infallible;
use axum::body::Body;
use axum::handler::Handler;
use axum::http::{Request, response, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::{get, get_service};



pub fn initialize() -> Router {
    let app = Router::new();
    let app = app.route("/healthz", (|| async { "Hello,It works. " }).into_service());
    app
}