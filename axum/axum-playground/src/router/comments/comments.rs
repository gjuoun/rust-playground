use axum::{routing::get, Router};
use hyper::StatusCode;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/** This is a middleware practice */
pub fn routes() -> Router {
    let router = Router::new()
        .route("/", get(get_comments))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    router
}
async fn get_comments() -> &'static str {
    "hello it's comments".into()
}

async fn test_hello() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "hello it's me".into())
}
