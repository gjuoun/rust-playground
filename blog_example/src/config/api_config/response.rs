use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    data: T,
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "success": true,
            "data": self.data,
        }));

        (StatusCode::OK, body).into_response()
    }
}

impl<T: Serialize> From<T> for AppResponse<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}
