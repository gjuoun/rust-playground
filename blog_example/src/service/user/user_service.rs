use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::config::api_config::AppError;

use super::user_model::{Address, Company, User};

pub struct UserService {
    pool: Pool<Postgres>,
}

static INSTANCE: OnceCell<UserService> = OnceCell::new();

#[derive(Serialize)]
pub struct PaginatedUsers {
    pub users: Vec<User>,
    pub total_count: i64,
    pub current_page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl UserService {
    pub fn init(pool: Pool<Postgres>) -> Result<(), String> {
        let service = Self { pool };
        INSTANCE
            .set(service)
            .map_err(|_| "UserService already initialized".to_string())
    }

    pub fn get_instance() -> &'static UserService {
        INSTANCE.get().expect("UserService not initialized")
    }

    fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                id, name, username, email, phone, website,
                address as "address!: Value",
                company as "company!: Value",
                created_at::text as "created_at!",
                updated_at::text as "updated_at!"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let address: Address = serde_json::from_value(r.address).unwrap();
            let company: Company = serde_json::from_value(r.company).unwrap();
            User {
                id: r.id,
                name: r.name,
                username: r.username,
                email: r.email,
                address,
                phone: r.phone,
                website: r.website,
                company,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        }))
    }

    pub async fn list_users(&self, page: i64, per_page: i64) -> Result<PaginatedUsers, AppError> {
        let offset = (page - 1) * per_page;

        let total_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0) as i64;

        let total_pages = (total_count + per_page - 1) / per_page;

        let rows = sqlx::query!(
            r#"
            SELECT 
                id, name, username, email, phone, website,
                address as "address!: Value",
                company as "company!: Value",
                created_at::text as "created_at!",
                updated_at::text as "updated_at!"
            FROM users
            ORDER BY id DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let users = rows
            .into_iter()
            .map(|r| {
                let address: Address = serde_json::from_value(r.address).unwrap();
                let company: Company = serde_json::from_value(r.company).unwrap();
                User {
                    id: r.id,
                    name: r.name,
                    username: r.username,
                    email: r.email,
                    address,
                    phone: r.phone,
                    website: r.website,
                    company,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }
            })
            .collect();

        Ok(PaginatedUsers {
            users,
            total_count,
            current_page: page,
            per_page,
            total_pages,
        })
    }
}
