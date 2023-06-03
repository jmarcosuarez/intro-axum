use super::read_middleware_custom_header::HeaderMessage;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
// Custom middleware
// This middleware does Insert headerMessage into Extensions - now previous routes will have this message
pub async fn set_middleware_custom_header<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_err| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = request.extensions_mut();

    extensions.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
