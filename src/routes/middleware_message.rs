use axum::Extension;

use super::SharedData;

pub async fn middleware_message(State(state): State<String>) -> String {
    message
}
