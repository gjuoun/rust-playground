use crate::config::api_config::{AppError, AppResponse};
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
async fn hello_world(
    Query(query): Query<HashMap<String, bool>>,
) -> Result<AppResponse<User>, AppError> {
    if let Some(true) = query.get("error") {
        Err(AppError::BadRequest("Forced error triggered".into()))
    } else {
        Ok(User {
            id: "user_123".to_owned(),
            username: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        }
        .into())
    }
}

/// GET /users/:id
async fn get_user_by_id(Path(user_id): Path<String>) -> Result<AppResponse<User>, AppError> {
    if user_id == "123" {
        Ok(User {
            id: user_id,
            username: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        }
        .into())
    } else {
        Err(AppError::NotFound(format!(
            "User with ID {} not found",
            user_id
        )))
    }
}

/// POST /users
/// body: [`CreateUser`]
async fn create_user(Json(payload): Json<CreateUser>) -> Result<AppResponse<User>, AppError> {
    if payload.email.contains('@') {
        Ok(User {
            id: "user_new".to_string(),
            username: payload.username,
            email: payload.email,
        }
        .into())
    } else {
        Err(AppError::BadRequest("Invalid email format".into()))
    }
}

/// GET /health
async fn health_check() -> Result<AppResponse<HealthStatus>, AppError> {
    Ok(get_health_status().into())
}

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/health", get(health_check));

    // Print all registered routes
    println!("\nRegistered user routes:");
    println!("  GET /");
    println!("  GET /users/:id");
    println!("  POST /users");
    println!("  GET /health\n");

    router
}
