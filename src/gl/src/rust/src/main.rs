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
use handlers::grocery::{create_item, delete_item, get_items, reorder_items, update_item};

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
        .route("/api/items", get(get_items))
        .route("/api/items", post(create_item))
        .route("/api/items/:id", put(update_item))
        .route("/api/items/:id", delete(delete_item))
        .route("/api/items/reorder", put(reorder_items))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Grocery List API server running on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}