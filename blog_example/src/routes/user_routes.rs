use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::config::api_config::{AppError, AppResponse};
use crate::service::user::{PaginatedUsers, User, UserService};

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    10
}

async fn get_user(Path(id): Path<i32>) -> Result<AppResponse<User>, AppError> {
    let service = UserService::get_instance();
    match service.get_user(id).await {
        Ok(Some(user)) => Ok(user.into()),
        Ok(None) => Err(AppError::NotFound(format!("User with ID {} not found", id))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

async fn list_users(
    Query(query): Query<ListUsersQuery>,
) -> Result<AppResponse<PaginatedUsers>, AppError> {
    let service = UserService::get_instance();
    let result = service.list_users(query.page, query.per_page).await?;

    Ok(result.into())
}

pub fn create_user_routes() -> Router {
    let router = Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user));

    println!("\nRegistered user routes:");
    println!("  GET /users");
    println!("  GET /users/:id\n");

    router
}
