use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::handlers::posts::{create_post, get_post, list_posts};
use crate::AppState;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/v1/posts", posts_routes(state.clone()))
        .fallback(handler_404)
        .with_state(state)
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

fn posts_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(list_posts))
        .route("/{id}", get(get_post))
        .with_state(state)
}
