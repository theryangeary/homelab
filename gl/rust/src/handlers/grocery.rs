use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

use crate::database::Database;
use crate::models::grocery_entry::{CreateGroceryListEntry, GroceryListEntry, ReorderEntry, UpdateGroceryListEntry};

pub async fn get_entries(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<GroceryListEntry>>, StatusCode> {
    tracing::info!("GET /api/entries called");
    match db.get_all_entries().await {
        Ok(entries) => {
            tracing::info!("Successfully retrieved {} entries", entries.len());
            Ok(Json(entries))
        },
        Err(e) => {
            tracing::error!("Failed to get entries: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn create_entry(
    State(db): State<Arc<Database>>,
    Json(payload): Json<CreateGroceryListEntry>,
) -> Result<Json<GroceryListEntry>, StatusCode> {
    tracing::info!("POST /api/entries called with description: '{}', position: {}", payload.description, payload.position);
    match db.create_entry(payload).await {
        Ok(entry) => {
            tracing::info!("Successfully created entry with id: {}", entry.id);
            Ok(Json(entry))
        },
        Err(e) => {
            tracing::error!("Failed to create entry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn update_entry(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateGroceryListEntry>,
) -> Result<Json<GroceryListEntry>, StatusCode> {
    match db.update_entry(id, payload).await {
        Ok(Some(entry)) => Ok(Json(entry)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_entry(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match db.delete_entry(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn reorder_entries(
    State(db): State<Arc<Database>>,
    Json(payload): Json<Vec<ReorderEntry>>,
) -> Result<StatusCode, StatusCode> {
    match db.reorder_entries(payload).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}