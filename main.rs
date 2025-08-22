mod config;
mod scanner;
mod routes;
mod utils;
mod exchanges;

use routes::create_router;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}