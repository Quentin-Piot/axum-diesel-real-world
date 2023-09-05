use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::AppState;
use crate::domain::models::{Post, PostError};
use crate::handlers::posts::PostResponse;
use crate::infra::db::errors::DbError;
use crate::infra::repositories::post_repository;
use crate::utils::PathExtractor;

pub async fn get_post(
    State(state): State<AppState>,
    PathExtractor(post_id): PathExtractor<Uuid>,
) -> Result<Json<PostResponse>, PostError> {
    let post = post_repository::get(&state.pool, post_id)
        .await
        .map_err(|db_error| {
            match db_error {
                DbError::InternalServerError => PostError::InternalServerError,
                DbError::NotFound => PostError::NotFound(post_id)
            }
        })?;

    Ok(Json(adapt_post_to_post_response(post)))
}


fn adapt_post_to_post_response(post: Post) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}


