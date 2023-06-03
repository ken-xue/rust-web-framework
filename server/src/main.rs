mod config;
mod router;
mod initialize;
mod util;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use mysql::OptsBuilder;
use tokio::signal;

#[tokio::main]
async fn main() {
    //load config
    let config = config::initialize();
    // initialize tracing
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = router::initialize();
    // let app = app.with_state(pool);
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let address = SocketAddr::from(([127, 0, 0, 1], 8088));
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}


async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
