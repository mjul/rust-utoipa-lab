use axum::Router;
use std::net::SocketAddr;
use tokio::join;
use tracing::instrument;
use tracing_subscriber;
use tracing_subscriber::{fmt, Layer};
mod enum_mapping;
mod enum_of_js_simple_types;
mod nesting_with_derive;
mod nesting_with_openapirouter;

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

    // Run the various examples
    let socket_addr: &SocketAddr = &"127.0.0.1:10000".parse().unwrap();
    let s1 = serve(socket_addr, nesting_with_derive::router());
    let socket_addr: &SocketAddr = &"127.0.0.1:10001".parse().unwrap();
    let s2 = serve(socket_addr, nesting_with_openapirouter::router());
    let socket_addr: &SocketAddr = &"127.0.0.1:10002".parse().unwrap();
    let s3 = serve(socket_addr, enum_of_js_simple_types::router());
    let socket_addr: &SocketAddr = &"127.0.0.1:10003".parse().unwrap();
    let s4 = serve(socket_addr, enum_mapping::router());

    // Wait for the servers to exit
    join!(s1, s2, s3, s4);
}
