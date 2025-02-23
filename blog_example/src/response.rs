use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiError>,
    request_id: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    code: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            request_id: generate_request_id(),
        }
    }

    pub fn error(message: impl Into<String>, code: impl Into<String>) -> ApiResponse<T> {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                message: message.into(),
                code: code.into(),
            }),
            request_id: generate_request_id(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            match self.error.as_ref().map(|e| e.code.as_str()) {
                Some("not_found") => StatusCode::NOT_FOUND,
                Some("bad_request") => StatusCode::BAD_REQUEST,
                Some("unauthorized") => StatusCode::UNAUTHORIZED,
                Some("forbidden") => StatusCode::FORBIDDEN,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };

        (status_code, Json(self)).into_response()
    }
}

fn generate_request_id() -> String {
    let timestamp = Utc::now().timestamp_millis();
    format!("req_{}_{}", Uuid::new_v4().simple(), timestamp)
}
