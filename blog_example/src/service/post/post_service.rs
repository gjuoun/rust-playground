use once_cell::sync::OnceCell;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::config::api_config::AppError;

use super::post_model::Post;

pub struct PostService {
    pool: Pool<Postgres>,
}

static INSTANCE: OnceCell<PostService> = OnceCell::new();

#[derive(Serialize)]
pub struct PaginatedPosts {
    pub posts: Vec<Post>,
    pub total_count: i64,
    pub current_page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl PostService {
    pub fn init(pool: Pool<Postgres>) -> Result<(), String> {
        let service = Self { pool };
        INSTANCE
            .set(service)
            .map_err(|_| "PostService already initialized".to_string())
    }

    pub fn get_instance() -> &'static PostService {
        INSTANCE.get().expect("PostService not initialized")
    }

    pub async fn create_post(
        &self,
        title: String,
        user_id: i32,
        body: String,
    ) -> Result<Post, AppError> {
        let result = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, "userId", body)
            VALUES ($1, $2, $3)
            RETURNING id, title, "userId" as user_id, body
            "#,
            title,
            user_id,
            body
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_post(&self, id: i32) -> Result<Option<Post>, AppError> {
        let result = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, "userId" as user_id, body
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_post(&self, id: i32) -> Result<Option<i32>, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            RETURNING id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.id))
    }

    pub async fn update_post(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
    ) -> Result<Option<Post>, AppError> {
        let result = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = COALESCE($1, title),
                body = COALESCE($2, body)
            WHERE id = $3
            RETURNING id, title, "userId" as user_id, body
            "#,
            title,
            body,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn list_posts(&self, page: i64, per_page: i64) -> Result<PaginatedPosts, AppError> {
        let offset = (page - 1) * per_page;

        // Get total count first
        let total_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM posts
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0) as i64;

        let total_pages = (total_count + per_page - 1) / per_page;

        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, "userId" as user_id, body
            FROM posts
            ORDER BY id DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(PaginatedPosts {
            posts,
            total_count,
            current_page: page,
            per_page,
            total_pages,
        })
    }
}
