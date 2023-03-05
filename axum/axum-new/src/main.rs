use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::{Value, json};

async fn hello_handler() -> &'static str {
    "Hello"
}

async fn json_handler() -> Json<Value> {
    Json(json!({ "jun": "guo"}))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World s!" }))
        .route("/hello", get(hello_handler))
        .route("/json", get(json_handler));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
