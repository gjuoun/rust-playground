use std::time::Duration;

use futures::future::join_all;
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

    // Create a Vec of JoinHandles for each task
    let handles = (1..=100).map(|id| {
        let client = client.clone();
        tokio::spawn(async move {
            let url = format!("https://jsonplaceholder.typicode.com/todos/{}", id);
            let result = client
                .get(&url)
                .send()
                .await?
                .error_for_status()?
                .json::<Todo>()
                .await;
            result
        })
    });

    // Wait for all tasks to complete
    let results = join_all(handles).await;

    // Process results, handling both task panics (JoinError) and request errors (AppError)
    let todos: Vec<Todo> = results
        .into_iter()
        .filter_map(|r| r.ok()?.map_err(AppError::from).ok())
        .collect();

    let duration = start.elapsed();
    println!("Fetched in {:?}", duration);

    Ok(todos)
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let todos = fetch_todos().await?;
    println!("Successfully fetched {} todos", todos.len());
    Ok(())
}
