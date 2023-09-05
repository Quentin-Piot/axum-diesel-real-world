use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::infra::db::errors::DbError;

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug)]
pub enum PostError {
    InternalServerError,
    NotFound(Uuid),
    DbError(DbError),
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Post with id {} has not been found", id),
            ),
            Self::DbError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"Post", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
