use axum::{routing::get, Router};
use hyper::{HeaderMap, StatusCode};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::log::warn;

pub fn routes() -> Router {
    let router = Router::new().route("/", get(get_hello));
    router
}
async fn get_hello(headers: HeaderMap) -> &'static str {
    let header = headers.get("X-jun").unwrap();
    let message = header.to_str().unwrap();
    warn!("header: {}", message);
    "hello it's comments".into()
}

async fn test_hello() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "hello it's me".into())
}
