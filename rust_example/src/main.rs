mod error;

use error::AppError;
use garde::Validate;
use serde::Deserialize;
use tokio::task;
use tokio::time::Instant;

#[derive(Debug, Deserialize, Validate)]
#[garde(allow_unvalidated)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,

    // #[garde(length(max = 15))]
    title: String,
    completed: bool,
}

async fn fetch_todos() -> Result<Vec<Todo>, AppError> {
    let client = reqwest::Client::new();
    let start = Instant::now();

    // Create a vector to hold the task handles
    let handles: Vec<_> = (1..=10)
        .map(|id| {
            let client = client.clone();
            tokio::spawn(async move {
                let response = client
                    .get(&format!(
                        "https://jsonplaceholder.typicode.com/todos/{}",
                        id
                    ))
                    .send()
                    .await?;

                let status = response.status();
                if !status.is_success() {
                    return Err(AppError::HttpStatus { status });
                }

                let todo: Todo = response.json().await?;
                todo.validate()?;

                Ok::<Todo, AppError>(todo)
            })
        })
        .collect();

    // Await all tasks and collect the results
    let mut todos = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Ok(todo)) => todos.push(todo),
            Ok(Err(e)) => println!("Task error: {}", e),
            Err(e) => println!("Join error: {}", e),
        }
    }

    let duration = start.elapsed();
    println!("Fetched {} todos in {:?}", todos.len(), duration);
    Ok(todos)
}

#[tokio::main]
async fn main() {
    match fetch_todos().await {
        Ok(todos) => println!("Successfully fetched todos: {:?}", todos),
        Err(err) => eprintln!("{}", err),
    }
}
