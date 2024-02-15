
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{
    services::{ServeDir},
};

// inspired by: https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs

async fn api_call() -> String {
    "Hello from API".to_string()
}

#[tokio::main]
async fn main() {
    // serve the file in the "www" directory under `/`
    let app = Router::new()
        .route("/api", get(api_call))
        .nest_service("/", ServeDir::new("www"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
