use axum::Router;
use std::net::SocketAddr;
use tracing::instrument;
use tracing_subscriber;
use tracing_subscriber::{fmt, Layer};
mod nesting_with_derive;

#[instrument]
pub(crate) async fn serve(socket_addr: &SocketAddr, app: Router) {
    tracing::info!("Starting server at http://{}", socket_addr);
    tracing::info!("Explore the API at http://{}/swagger-ui", socket_addr);
    let svc = app.into_make_service();
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, svc).await.unwrap();
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    let default_collector = tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug,webapp=debug")
        // build but do not install the subscriber.
        .finish();

    tracing::subscriber::set_global_default(default_collector)
        .expect("setting default subscriber failed");

    let socket_addr: &SocketAddr = &"127.0.0.1:10000".parse().unwrap();
    serve(socket_addr, nesting_with_derive::router()).await;
}
