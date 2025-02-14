use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[tokio::main]
async fn main() {
    // Create a user
    let user = User {
        name: String::from("John Doe"),
        age: 30,
        email: String::from("john@example.com"),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("Serialized JSON:\n{}", json);

    // Simulate async operation
    process_user(&user).await;
}

async fn process_user(user: &User) {
    // Simulate some async processing
    println!("\nProcessing user...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("User {} processed successfully!", user.name);
}
