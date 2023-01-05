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

    let url = Url::parse("https://example.com/products?page=2&sort=desc")?;
    let pairs: HashMap<_,_> = url.query_pairs().into_owned().collect();

    println!("{}", pairs.get("page").ok_or("no page")?);
    println!("{}", pairs.get("sort").ok_or("no sort")?);

    Ok(())
}
