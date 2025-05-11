use axum::{routing::get, Router, serve::Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build app with a single route
    let app = Router::new().route("/", get(|| async { "Hello, Fly.io from Rust!" }));

    // Read PORT env var (Fly.io sets this)
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);

    // run app
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}