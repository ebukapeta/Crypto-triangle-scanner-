use axum::{routing::get, Router};
use axum::response::Html;
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        // Serve static files (like CSS, JS, images) from /static
        .nest_service("/static", ServeDir::new("static"))
}

async fn index() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Crypto Arbitrage Scanner</title>
                <link rel="stylesheet" href="/static/style.css">
            </head>
            <body>
                <h1>Crypto Arbitrage Scanner</h1>
                <p>Welcome! The scanner is running and will update as new data is fetched.</p>
            </body>
        </html>
    "#)
    }
