use axum::{
    body::Body,
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

mod create_task;
use create_task::create_task;

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/hello_world", get(|| async { "Hello, World!" }))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
