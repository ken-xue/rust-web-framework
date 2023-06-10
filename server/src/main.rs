mod config;
mod database;
mod router;
mod util;

use std::net::SocketAddr;
use tokio::signal;

#[tokio::main]
async fn main() {
    //load config
    let config = config::initialize();
    // initialize tracing
    tracing_subscriber::fmt::init();
    // init database
    database::initialize(config);
    // build our application with a route
    let router = router::initialize();
    // `axum::Server` is a re-export of `hyper::Server`
    let address = SocketAddr::from(([127, 0, 0, 1], 8088));
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
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