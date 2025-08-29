mod routes;
mod scanner;
mod models;
mod utils;
mod exchanges;

use axum::Router;
use std::net::SocketAddr;
use routes::create_router;
use tokio;

#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
