use axum::{Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;

mod routes;
mod exchanges;
mod scanner;

#[tokio::main]
async fn main() {
    // Build the app routes from routes.rs
    let app = routes::create_routes();

    // Address binding for Render or local use
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    // Start the server
    axum::serve(listener, app)
        .await
        .unwrap();
}
