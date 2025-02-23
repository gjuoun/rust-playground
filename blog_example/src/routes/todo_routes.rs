use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::config::api_config::{AppError, AppResponse};
use crate::service::todo::{Todo, TodoService};

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub user_id: i32,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTodosQuery {
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

async fn create_todo(Json(req): Json<CreateTodoRequest>) -> Result<AppResponse<Todo>, AppError> {
    let service = TodoService::get_instance();
    let todo = service
        .create_todo(req.title, req.user_id, req.body)
        .await?;

    Ok(todo.into())
}

async fn get_todo(Path(id): Path<i32>) -> Result<AppResponse<Todo>, AppError> {
    let service = TodoService::get_instance();
    match service.get_todo(id).await {
        Ok(Some(todo)) => Ok(todo.into()),
        Ok(None) => Err(AppError::NotFound(format!("Todo with ID {} not found", id))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

async fn update_todo(
    Path(id): Path<i32>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<AppResponse<Todo>, AppError> {
    let service = TodoService::get_instance();
    match service.update_todo(id, req.title, req.body).await {
        Ok(Some(todo)) => Ok(todo.into()),
        Ok(None) => Err(AppError::NotFound(format!("Todo with ID {} not found", id))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

async fn delete_todo(Path(id): Path<i32>) -> Result<AppResponse<i32>, AppError> {
    let service = TodoService::get_instance();
    match service.delete_todo(id).await {
        Ok(Some(id)) => Ok(id.into()),
        Ok(None) => Err(AppError::NotFound(format!("Todo with ID {} not found", id))),
        Err(e) => Err(e),
    }
}

async fn list_todos(
    Query(query): Query<ListTodosQuery>,
) -> Result<AppResponse<Vec<Todo>>, AppError> {
    let service = TodoService::get_instance();
    service
        .list_todos(query.page, query.per_page)
        .await
        .map(AppResponse::from)
        .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn todo_routes() -> Router {
    let router = Router::new()
        .route("/todos", post(create_todo))
        .route("/todos", get(list_todos))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo));

    println!("\nRegistered todo routes:");
    println!("  POST /todos");
    println!("  GET /todos");
    println!("  GET /todos/:id");
    println!("  PUT /todos/:id");
    println!("  DELETE /todos/:id\n");

    router
}
