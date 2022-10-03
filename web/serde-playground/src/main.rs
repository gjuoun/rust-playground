use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
    //     .await?
    //     .json::<Todo>()
    //     .await?;
    // let client = reqwest::Client::new();

    // let resp = client
    //     .post("https://jsonplaceholder.typicode.com/todos")
    //     .json(&Todo {
    //         user_id: 1,
    //         id: 1,``
    //         title: "test".to_owned(),
    //         completed: false,
    //     })
    //     .send()
    //     .await?;
    // // println!("{:#?}", resp.json().await?);

    // let content_type = resp
    //     .headers()
    //     .get("content-type")
    //     .ok_or("missing content-type header")?
    //     .to_str()?;
    // println!("{}", content_type);

    // let return_todo = resp.json::<Todo>().await?;
    // println!("{:#?}", return_todo);

    let url = Url::parse("https://example.com/products?page=2&sort=desc")?;
    let pairs: HashMap<_,_> = url.query_pairs().into_owned().collect();

    println!("{}", pairs.get("page").ok_or("no page")?);
    println!("{}", pairs.get("sort").ok_or("no sort")?);

    Ok(())
}
