use axum::{response::Html, routing::get, Router};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let random_string = generate_random_string();
    Html(format!("<h1>Random: {}</h1>", random_string))
}

fn generate_random_string() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}