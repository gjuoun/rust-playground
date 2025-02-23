use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    message: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}

async fn hello_world() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

async fn get_user_by_id(Path(user_id): Path<String>) -> Json<Message> {
    Json(Message {
        message: format!("Getting user with ID: {}", user_id),
    })
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<Message> {
    Json(Message {
        message: format!(
            "Created user: {} with email: {}",
            payload.username, payload.email
        ),
    })
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
}
