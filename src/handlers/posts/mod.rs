use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use create_post::create_post;
pub use get_post::get_post;
pub use list_posts::list_posts;

mod create_post;
mod get_post;
mod list_posts;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    title: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPostsResponse {
    posts: Vec<PostResponse>,
}
