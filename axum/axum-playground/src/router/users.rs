use axum::{
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
}

pub fn routes() -> Router {
    let router = Router::new()
        .route("/", get(get_hello).post(create_user))
        .route("/echo", post(echo_json));
    router
}
async fn get_hello() -> &'static str {
    "hello it's users".into()
}

async fn create_user() -> (StatusCode, Json<User>) {
    (
        StatusCode::CREATED,
        Json(User {
            id: 1,
            name: "test".into(),
        }),
    )
}

async fn echo_json(body: Json<User>) -> Json<User> {
    body
}
