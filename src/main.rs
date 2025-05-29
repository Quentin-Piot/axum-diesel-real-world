use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::net::SocketAddr;
use tracing_subscriber::prelude::*;
use diesel_migrations::MigrationHarness;

mod config;
mod domain;
mod errors;
mod handlers;
mod infra;
mod routes;
mod state;
mod utils;

use crate::config::config;
use crate::errors::{internal_error, AppError};
use crate::routes::app_router;
use crate::state::AppState;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    init_tracing();

    let app_config = config().await;

    let manager = Manager::new(app_config.db_url().to_string(), Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create connection pool");

    if let Err(err) = run_migrations(&pool).await {
        tracing::error!("Failed to run migrations: {:?}", err);
        // Consider exiting the application or handling the error appropriately
        return;
    }

    let state = AppState { pool };

    let app = app_router(state.clone());

    let host = app_config.server_host();
    let port = app_config.server_port();
    let address = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = address.parse().expect("Unable to parse socket address");

    tracing::info!("listening on http://{}", socket_addr);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "example_tokio_postgres=debug,axum_diesel_real_world=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn run_migrations(pool: &Pool) -> Result<(), AppError> {
    let conn = pool.get().await.map_err(internal_error)?;
    conn.interact(|conn_inner| conn_inner.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(())
}
