use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: Option<u32>,
    title: String,
    body: Option<String>,
    user_id: u32,
    completed: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .json::<Post>()
        .await?;

    println!("{:#?}", resp);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&Post {
            id: None,
            title: "foo".to_owned(),
            body: Some("bar".to_owned()),
            user_id: 1,
            completed: Some(true),
        })
        .send()
        .await?;

    let body = resp.json::<Post>().await?;
    println!("{:#?}", body);

    Ok(())
}
