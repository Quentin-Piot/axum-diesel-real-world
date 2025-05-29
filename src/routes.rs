use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

// Import handlers for post-related operations
use crate::handlers::posts::{create_post, get_post, list_posts};
// Import the application state
use crate::AppState;

// Function to create the main application router
pub fn app_router(state: AppState) -> Router {
    Router::new()
        // Define the root route
        .route("/", get(root))
        // Nest post-related routes under "/v1/posts"
        .nest("/v1/posts", posts_routes(state.clone()))
        // Define a fallback handler for 404 errors
        .fallback(handler_404)
        // Attach the application state to the router
        .with_state(state)
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

// Function to define post-related routes
fn posts_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Route for creating a new post (POST /v1/posts)
        .route("/", post(create_post))
        // Route for listing all posts (GET /v1/posts)
        .route("/", get(list_posts))
        // Route for getting a specific post by ID (GET /v1/posts/:id)
        .route("/{id}", get(get_post))
        // Attach the application state to the posts router
        .with_state(state)
}
