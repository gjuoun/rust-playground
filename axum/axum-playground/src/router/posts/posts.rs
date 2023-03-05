use super::{posts_type::Post, posts_error::PostError};
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    let router = Router::new().route("/", get(get_posts).post(create_post));
    router
}

async fn get_posts() -> Result<Json<Value>, PostError> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/posts").await?;
    let body = res.json::<Value>().await?;
    Ok(Json(body))
}

async fn create_post(body: Json<Post>) -> Result<Json<Value>, PostError> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&body.0)
        .send()
        .await?;

    let body = res.json().await?;
    Ok(Json(body))
}
