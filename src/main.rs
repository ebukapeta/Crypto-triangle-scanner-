mod routes;
mod utils;
mod exchanges;

use axum::{Router, Server};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use routes::create_router;

#[tokio::main]
async fn main() {
    let app = create_router().nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
               }
