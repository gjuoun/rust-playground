use anyhow::{Context, Result};
use garde::Validate;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
#[garde(allow_unvalidated)]
struct Todo {
    id: i32,

    #[garde(length(max = 15))]
    title: String,

    completed: bool,

    #[serde(rename = "userId")]
    user_id: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    fetch_todo().await.context("Failed to fetch todo")?;

    Ok(())
}

async fn fetch_todo() -> Result<Todo> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await
        .context("Failed to make request")?;

    let todo: Todo = res.json().await.context("Failed to parse JSON")?;

    todo.validate().context("Validation failed")?;

    Ok(todo)
}
