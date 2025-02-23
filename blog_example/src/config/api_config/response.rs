use super::error::AppError;
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
struct ApiError {
    message: String,
    status: u16,
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

    pub fn error(err: AppError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                message: err.to_string(),
                status: err.status_code().as_u16(),
            }),
            request_id: generate_request_id(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            self.error
                .as_ref()
                .and_then(|e| StatusCode::from_u16(e.status).ok())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        };

        (status, Json(self)).into_response()
    }
}

fn generate_request_id() -> String {
    let timestamp = Utc::now().timestamp_millis();
    format!("req_{}_{}", Uuid::new_v4().simple(), timestamp)
}
