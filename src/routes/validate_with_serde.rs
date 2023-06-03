use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    // username: Option<String>, // Use Option to declare this property as optional
    username: String,
    password: String,
}

// Use serde to enforce -validate the shape of the incommingjson
pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    // dbg!(user);
}
