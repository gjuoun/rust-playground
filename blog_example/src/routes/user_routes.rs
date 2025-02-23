use crate::config::api_config::{ApiResponse, AppError};
use crate::health::{get_health_status, HealthStatus};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
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
        ApiResponse::error(AppError::BadRequest("Forced error triggered".into()))
    } else {
        ApiResponse::success(User {
            id: "user_123".to_owned(),
            username: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        })
    }
}

/// GET /users/:id
async fn get_user_by_id(Path(user_id): Path<String>) -> ApiResponse<User> {
    if user_id == "123" {
        ApiResponse::success(User {
            id: user_id,
            username: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        })
    } else {
        ApiResponse::error(AppError::NotFound(format!(
            "User with ID {} not found",
            user_id
        )))
    }
}

/// POST /users
/// body: [`CreateUser`]
async fn create_user(Json(payload): Json<CreateUser>) -> ApiResponse<User> {
    if payload.email.contains('@') {
        ApiResponse::success(User {
            id: "user_new".to_string(),
            username: payload.username,
            email: payload.email,
        })
    } else {
        ApiResponse::error(AppError::BadRequest("Invalid email format".into()))
    }
}

/// GET /health
async fn health_check() -> ApiResponse<HealthStatus> {
    ApiResponse::success(get_health_status())
}

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/health", get(health_check));

    // Print all registered routes
    println!("\nRegistered routes:");
    println!("  GET /");
    println!("  GET /users/:id");
    println!("  POST /users");
    println!("  GET /health\n");

    router
}
