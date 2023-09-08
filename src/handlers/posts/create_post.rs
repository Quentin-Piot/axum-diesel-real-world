use axum::extract::State;
use axum::Json;

use crate::domain::models::post::PostError;
use crate::handlers::posts::{CreatePostRequest, PostResponse};
use crate::infra::repositories::post_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_post(
    State(state): State<AppState>,
    JsonExtractor(new_post): JsonExtractor<CreatePostRequest>,
) -> Result<Json<PostResponse>, PostError> {
    let new_post_db = post_repository::NewPostDb {
        title: new_post.title,
        body: new_post.body,
        published: false,
    };

    let created_post = post_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(PostError::InfraError)?;

    let post_response = PostResponse {
        id: created_post.id,
        title: created_post.title,
        body: created_post.body,
        published: created_post.published,
    };

    Ok(Json(post_response))
}
