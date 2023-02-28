use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use serde_json::{json, Value};

pub fn routes() -> Router {
    let router = Router::new().route("/", get(get_hello));
    router
}

async fn get_hello() -> Result<Json<Value>, PostError> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/").await?;
    let body = res.json::<Value>().await?;
    Ok(Json(body))
}

struct PostError(reqwest::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for PostError {
    fn into_response(self) -> Response {
        let e = self.0;
        // e.inner.
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let body = json!({
            "error": e.to_string(),
        });

        (status, Json(body)).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for PostError
where
    E: Into<reqwest::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
