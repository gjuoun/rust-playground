use std::time::Duration;

use futures::stream::{self, StreamExt, TryStreamExt};
use garde::Validate;
use internal::error::AppError;
use serde::Deserialize;
use tokio::time::Instant;

#[derive(Debug, Deserialize, Validate)]
#[garde(allow_unvalidated)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

async fn fetch_todos() -> Result<Vec<Todo>, AppError> {
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(20)
        .timeout(Duration::from_secs(15))
        .build()?;

    let start = Instant::now();

    let todos: Vec<Todo> = stream::iter(1..=100)
        .map(|id| {
            let client = client.clone();
            async move {
                let url = format!("https://jsonplaceholder.typicode.com/todos/{}", id);
                client
                    .get(&url)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<Todo>()
                    .await
            }
        })
        .buffer_unordered(20)
        // may throw error immediately
        .try_collect::<Vec<Todo>>()
        .await?;

    let duration = start.elapsed();

    println!("Fetched in {:?}", duration);
    Ok(todos)
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!(
        "Runtime threads: {}",
        tokio::runtime::Handle::current().metrics().num_workers()
    );
    let todos = fetch_todos().await?;
    println!("Successfully fetched {} todos", todos.len());
    Ok(())
}
