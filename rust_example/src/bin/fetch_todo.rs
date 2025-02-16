use garde::Validate;
use internal::error::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
#[garde(allow_unvalidated)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,

    #[garde(length(max = 15))]
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    match fetch_todo(&reqwest::Client::new()).await {
        Ok(todo) => println!("Successfully fetched todo: {:?}", todo),
        Err(err) => eprintln!("{}", err),
    }
}

async fn fetch_todo(client: &reqwest::Client) -> Result<Todo, AppError> {
    let response = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await?; // Error automatically converted via From trait

    let status = response.status();
    if !status.is_success() {
        return Err(AppError::HttpStatus { status });
    }

    let todo: Todo = response.json().await?; // JSON errors auto-converted

    // Validate the todo
    todo.validate()?; // Validation errors converted via From trait

    Ok(todo)
}
