use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;

use crate::service::post::{Post, PostService};
use crate::{
    config::api_config::{AppError, AppResponse},
    service::post::PaginatedPosts,
};

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub user_id: i32,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
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

async fn create_post(Json(req): Json<CreatePostRequest>) -> Result<AppResponse<Post>, AppError> {
    let service = PostService::get_instance();
    let post = service
        .create_post(req.title, req.user_id, req.body)
        .await?;

    Ok(post.into())
}

async fn get_post(Path(id): Path<i32>) -> Result<AppResponse<Post>, AppError> {
    let service = PostService::get_instance();
    match service.get_post(id).await {
        Ok(Some(post)) => Ok(post.into()),
        Ok(None) => Err(AppError::NotFound(format!("Post with ID {} not found", id))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

async fn update_post(
    Path(id): Path<i32>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<AppResponse<Post>, AppError> {
    let service = PostService::get_instance();
    match service.update_post(id, req.title, req.body).await {
        Ok(Some(post)) => Ok(post.into()),
        Ok(None) => Err(AppError::NotFound(format!("Post with ID {} not found", id))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

async fn delete_post(Path(id): Path<i32>) -> Result<AppResponse<i32>, AppError> {
    let service = PostService::get_instance();
    match service.delete_post(id).await {
        Ok(Some(id)) => Ok(id.into()),
        Ok(None) => Err(AppError::NotFound(format!("Post with ID {} not found", id))),
        Err(e) => Err(e),
    }
}

async fn list_posts(
    Query(query): Query<ListPostsQuery>,
) -> Result<AppResponse<PaginatedPosts>, AppError> {
    let service = PostService::get_instance();
    let result = service.list_posts(query.page, query.per_page).await?;

    Ok(result.into())
}

pub fn create_post_routes() -> Router {
    let router = Router::new()
        .route("/posts", post(create_post))
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post));

    println!("\nRegistered post routes:");
    println!("  POST /posts");
    println!("  GET /posts");
    println!("  GET /posts/:id");
    println!("  PUT /posts/:id");
    println!("  DELETE /posts/:id\n");

    router
}
