use axum::{
    body::Body,
    routing::{get, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

mod create_task;
mod get_tasks;
mod update_tasks;

use create_task::create_task;
use get_tasks::get_all_tasks;
use get_tasks::get_one_task;
use update_tasks::atomic_update;

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/hello_world", get(|| async { "Hello, World!" }))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .layer(Extension(database))
}
