mod database;
mod handlers;
mod models;

use axum::{
    http::{header, StatusCode, Uri}, response::{Html, IntoResponse, Response}, routing::{delete, get, post, put}, Json, Router
};
use std::{env, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use database::Database;
use handlers::{
    create_entry, delete_entry, get_categories, get_entries, grocery, reorder_categories,
    reorder_entries, update_entry, create_category, update_category, delete_category, category,
};
use rust_embed::Embed;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "./ts/dist"]

struct Assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "grocery_list_backend=debug,tower_http=debug,axum::rejection=trace,sqlx=debug".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true)
                .pretty() // Makes it more readable
        )
        .init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:grocery.db".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());

    tracing::info!("Starting grocery list backend on port {}", port);
    tracing::info!("Database URL: {}", database_url);

    let db = Arc::new(Database::new(&database_url).await?);

    let app = Router::new().fallback(static_handler)
        .route("/api/entries", get(get_entries))
        .route("/api/entries", post(create_entry))
        .route("/api/entries/:id", put(update_entry))
        .route("/api/entries/:id", delete(delete_entry))
        .route("/api/entries/reorder", put(reorder_entries))
        .route("/api/entries/suggestions", get(grocery::get_suggestions))
        .route("/api/categories", get(get_categories))
        .route("/api/categories", post(create_category))
        .route("/api/categories/:id", put(update_category))
        .route("/api/categories/:id", delete(delete_category))
        .route("/api/categories/reorder", put(reorder_categories))
        .route("/api/categories/suggestions", get(category::get_suggestions))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(db);


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Grocery List API server running on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
  let path = uri.path().trim_start_matches('/');

  if path.is_empty() || path == INDEX_HTML {
    return index_html().await;
  }

  match Assets::get(path) {
    Some(content) => {
      let mime = mime_guess::from_path(path).first_or_octet_stream();

      ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    }
    None => {
      if path.contains('.') {
        return not_found().await;
      }

      index_html().await
    }
  }
}

async fn index_html() -> Response {
  match Assets::get(INDEX_HTML) {
    Some(content) => Html(content.data).into_response(),
    None => not_found().await,
  }
}

async fn not_found() -> Response {
  (StatusCode::NOT_FOUND, "404").into_response()
}
