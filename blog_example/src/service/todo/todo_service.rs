use chrono::NaiveDateTime;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config::api_config::AppError;

use super::todo_model::Todo;

pub struct TodoService {
    pool: Pool<Postgres>,
}

static INSTANCE: OnceCell<TodoService> = OnceCell::new();

impl TodoService {
    pub fn init(pool: Pool<Postgres>) -> Result<(), String> {
        let service = Self { pool };
        INSTANCE
            .set(service)
            .map_err(|_| "TodoService already initialized".to_string())
    }

    pub fn get_instance() -> &'static TodoService {
        INSTANCE.get().expect("TodoService not initialized")
    }

    // Private constructor
    fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_todo(
        &self,
        title: String,
        user_id: i32,
        body: String,
    ) -> Result<Todo, AppError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (title, "userId", body)
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

    pub async fn get_todo(&self, id: i32) -> Result<Option<Todo>, AppError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title, "userId" as user_id, body
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<Option<i32>, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE id = $1
            RETURNING id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.id))
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
    ) -> Result<Option<Todo>, AppError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
            UPDATE todos
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

    pub async fn list_todos(&self, page: i64, per_page: i64) -> Result<Vec<Todo>, AppError> {
        let offset = (page - 1) * per_page;
        let result = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title, "userId" as user_id, body
            FROM todos
            ORDER BY id DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}
