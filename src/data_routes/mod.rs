use axum::{body::Body, routing::get, Router};

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/test", get(|| async { "Hello, World!" }))
}
