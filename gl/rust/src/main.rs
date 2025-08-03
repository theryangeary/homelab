mod database;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::{env, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use database::Database;
use handlers::grocery::{create_entry, delete_entry, get_entries, get_suggestions, reorder_entries, update_entry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "grocery_list_backend=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:grocery.db".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());

    tracing::info!("Starting grocery list backend on port {}", port);
    tracing::info!("Database URL: {}", database_url);

    let db = Arc::new(Database::new(&database_url).await?);

    let app = Router::new()
        .route("/api/entries", get(get_entries))
        .route("/api/entries", post(create_entry))
        .route("/api/entries/:id", put(update_entry))
        .route("/api/entries/:id", delete(delete_entry))
        .route("/api/entries/reorder", put(reorder_entries))
        .route("/api/entries/suggestions", get(get_suggestions))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Grocery List API server running on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}