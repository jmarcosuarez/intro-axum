use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Iam in data".to_owned(),
        count: 32394,
        username: "I have got an username".to_owned(),
    };

    Json(data)
}
