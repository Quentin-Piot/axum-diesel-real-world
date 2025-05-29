use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    InternalServerError(String),
    BodyParsingError(String),
}

#[allow(dead_code)]
pub fn internal_error<E: std::error::Error>(err: E) -> AppError {
    tracing::error!("Internal Server Error: {:?}", err);
    AppError::InternalServerError(err.to_string())
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", message),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", message),
            ),
        };
        (status, Json(json!({ "message": err_msg }))).into_response()
    }
}
