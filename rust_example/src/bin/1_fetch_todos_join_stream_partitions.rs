use futures::stream::{self, StreamExt};
use internal::error::AppError;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug, Deserialize)]
struct Todo {
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

async fn fetch_todos() -> Vec<Result<Todo, reqwest::Error>> {
    let client = Client::builder()
        .pool_max_idle_per_host(20)
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to build client");

    let start = Instant::now();

    let todos: Vec<Result<Todo, reqwest::Error>> = stream::iter(1..=100)
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
        .collect()
        .await;

    let duration = start.elapsed();
    println!("Fetched in {:?}", duration);

    todos
}

#[tokio::main]
async fn main() {
    let todos = fetch_todos().await;
    let (successes, failures): (Vec<_>, Vec<_>) = todos.into_iter().partition(Result::is_ok);

    println!("Successfully fetched {} todos", successes.len());
    println!("Failed to fetch {} todos", failures.len());
}
