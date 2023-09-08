use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

use crate::handlers::posts::{create_post, get_post, list_posts};
use crate::AppState;

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/v1/posts", posts_route(state.clone()))
        .fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running!"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn posts_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(list_posts))
        .route("/:id", get(get_post))
        .with_state(state)
}
