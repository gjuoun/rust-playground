use hyper::{Client, Request, Method, Body};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    origin: String,
}
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
    .method(Method::POST)
    .uri("http://httpbin.org/post")
    .header("content-type", "application/json")
    .body(Body::from(r#"{"library":"hyper"}"#))?;

// We'll send it in a second...


    Ok(())
}

