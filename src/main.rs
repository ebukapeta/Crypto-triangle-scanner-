use axum::{Router, routing::get, serve};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Basic app setup
    let app = Router::new()
        .route("/", get(|| async { "Triangular Arbitrage Scanner is running!" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Bind to port 8080 for local and render compatibility
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("Server running at http://0.0.0.0:8080");

    // Start server
    serve(listener, app).await.unwrap();
}
