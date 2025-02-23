mod config;
mod routes;
mod service;

use axum::Router;
use config::env_config::EnvConfig;
use routes::todo_routes::todo_routes;
use service::todo::TodoService;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    EnvConfig::init()?;
    let config = EnvConfig::get_instance();

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Initialize TodoService
    TodoService::init(pool.clone())?;

    // Build application
    let app = Router::new().merge(todo_routes());

    // Start server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    println!("Server running on http://{}:{}", config.host, config.port);
    axum::serve(listener, app).await?;

    Ok(())
}
