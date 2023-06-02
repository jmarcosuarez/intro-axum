mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variables::{hard_coded_path, path_variables};

pub fn create_routes() -> Router<(), Body> {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        // start - order is not important. Router will always find the better match
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
    // end - order is not important. Router will always find the better match
}
