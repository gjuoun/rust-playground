use crate::health::{get_health_status, HealthStatus};
use crate::response::ApiResponse;
use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Validate)]
pub struct User {
    #[garde(skip)]
    id: String,
    #[garde(length(min = 3, max = 255))]
    username: String,
    #[garde(email)]
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}

/// GET /
async fn hello_world(Query(query): Query<HashMap<String, bool>>) -> ApiResponse<User> {
    if let Some(true) = query.get("error") {
        return ApiResponse::error("Forced error triggered", "bad_request".to_owned());
    }

    ApiResponse::success(User {
        id: "user_123".to_owned(),
        username: "John Doe".to_owned(),
        email: "john@example.com".to_owned(),
    })
}

/// GET /users/:id
async fn get_user_by_id(Path(user_id): Path<String>) -> ApiResponse<User> {
    // Simulate a database lookup
    if user_id == "123" {
        ApiResponse::success(User {
            id: user_id,
            username: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        })
    } else {
        ApiResponse::error(
            format!("User with ID {} not found", user_id),
            "not_found".to_owned(),
        )
    }
}

/// POST /users
/// body: [`CreateUser`]
async fn create_user(Json(payload): Json<CreateUser>) -> ApiResponse<User> {
    // Simulate user creation
    if payload.email.contains('@') {
        ApiResponse::success(User {
            id: "user_new".to_string(),
            username: payload.username,
            email: payload.email,
        })
    } else {
        ApiResponse::error("Invalid email format".to_owned(), "bad_request".to_owned())
    }
}

/// GET /health
async fn health_check() -> ApiResponse<HealthStatus> {
    ApiResponse::success(get_health_status())
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/health", get(health_check))
}
