use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<u32>,
    title: String,
    body: String,
    user_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
    //     .await?
    //     .json::<HashMap<String, Value>>()
    //     .await?;

    let client = reqwest::Client::new();
    let resp = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&Post {
            id: None,
            title: "foo".to_owned(),
            body: "bar".to_owned(),
            user_id: 1,
        })
        .send()
        .await?;

    let body = resp.json::<Post>().await?;
    println!("{:#?}", body);

    Ok(())
}
