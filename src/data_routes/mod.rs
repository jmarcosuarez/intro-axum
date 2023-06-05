use axum::{
    body::Body,
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

mod create_task;
mod get_one_task;

use create_task::create_task;
use get_one_task::get_one_task;

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/hello_world", get(|| async { "Hello, World!" }))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
