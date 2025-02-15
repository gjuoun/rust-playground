use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
    #[serde(rename = "userId")]
    user_id: i32,
}

impl Todo {
    fn validate(&self) -> Result<()> {
        if self.user_id <= 0 {
            anyhow::bail!("Invalid user_id: {}", self.user_id);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await
        .context("Failed to make request")?;

    let todo: Todo = res.json()
        .await
        .context("Failed to parse JSON")?;

    todo.validate()?;

    println!("Todo: {:#?}", todo);
    Ok(())
}
