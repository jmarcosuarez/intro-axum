use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

mod create_task;
mod delete_task;
mod get_tasks;
mod guard;
mod partial_update_tasks;
mod update_tasks;
mod user;
use create_task::create_task;
use delete_task::delete_task;
use get_tasks::get_all_tasks;
use get_tasks::get_one_task;
use guard::guard;
use partial_update_tasks::partial_tasks;
use update_tasks::atomic_update;
use user::{create_user, login, logout};

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/users/logout", post(logout))
        .route("/hello_world", get(|| async { "Hello, World!" }))
        // only routes above guard will check on auth heather token in db to run
        .route_layer(middleware::from_fn(guard))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_tasks))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .layer(Extension(database))
}
