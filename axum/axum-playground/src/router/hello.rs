use axum::{routing::get, Router};
use hyper::StatusCode;

pub fn routes() -> Router {
    let router = Router::new()
        .route("/", get(get_hello))
        .route("/test", get(test_hello));
    router
}
async fn get_hello() -> &'static str {
    "hello it's me".into()
}

async fn test_hello() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "hello it's me".into())
}
