use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

use crate::database::Database;
use crate::models::grocery_item::{CreateGroceryListEntry, GroceryListEntry, ReorderItem, UpdateGroceryListEntry};

pub async fn get_items(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<GroceryListEntry>>, StatusCode> {
    tracing::info!("GET /api/entries called");
    match db.get_all_items().await {
        Ok(items) => {
            tracing::info!("Successfully retrieved {} items", items.len());
            Ok(Json(items))
        },
        Err(e) => {
            tracing::error!("Failed to get items: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn create_item(
    State(db): State<Arc<Database>>,
    Json(payload): Json<CreateGroceryListEntry>,
) -> Result<Json<GroceryListEntry>, StatusCode> {
    tracing::info!("POST /api/entries called with description: '{}', position: {}", payload.description, payload.position);
    match db.create_item(payload).await {
        Ok(item) => {
            tracing::info!("Successfully created item with id: {}", item.id);
            Ok(Json(item))
        },
        Err(e) => {
            tracing::error!("Failed to create item: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn update_item(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateGroceryListEntry>,
) -> Result<Json<GroceryListEntry>, StatusCode> {
    match db.update_item(id, payload).await {
        Ok(Some(item)) => Ok(Json(item)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_item(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match db.delete_item(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn reorder_items(
    State(db): State<Arc<Database>>,
    Json(payload): Json<Vec<ReorderItem>>,
) -> Result<StatusCode, StatusCode> {
    match db.reorder_items(payload).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}