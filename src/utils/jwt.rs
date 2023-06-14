use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{crypto::verify, decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30); // note tokens expire after 30 seconds (good for dev)  - for testing theres a leedway
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claims = Claims { exp, iat };
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claims, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, StatusCode> {
    let secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(
        token,
        &key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(true)
}
