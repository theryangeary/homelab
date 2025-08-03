use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::database::Database;
use crate::models::category::{CreateCategory, Category, ReorderCategory, UpdateCategory};

#[derive(Deserialize)]
pub struct SuggestionsQuery {
    query: String,
}


pub async fn get_categories(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    tracing::info!("GET /api/categories called");
    match db.get_all_categories().await {
        Ok(categories) => {
            tracing::info!("Successfully retrieved {} categories", categories.len());
            Ok(Json(categories))
        },
        Err(e) => {
            tracing::error!("Failed to get categories: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn create_category(
    State(db): State<Arc<Database>>,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<Category>, StatusCode> {
    tracing::info!("POST /api/categories called with name: '{}'", payload.name);
    match db.create_category(payload).await {
        Ok(category) => {
            tracing::info!("Successfully created category with id: {}", category.id);
            Ok(Json(category))
        },
        Err(e) => {
            tracing::error!("Failed to create category: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn update_category(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<Category>, StatusCode> {
    match db.update_category(id, payload).await {
        Ok(Some(category)) => Ok(Json(category)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_category(
    State(db): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    match db.delete_category(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn reorder_categories(
    State(db): State<Arc<Database>>,
    Json(payload): Json<Vec<ReorderCategory>>,
) -> Result<StatusCode, StatusCode> {
    match db.reorder_categories(payload).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_suggestions(
    State(db): State<Arc<Database>>,
    Query(params): Query<SuggestionsQuery>,
) -> Result<Json<Vec<String>>, StatusCode> {
    tracing::info!("GET /api/categories/suggestions called with query: '{}'", params.query);
    match db.get_suggestions(&params.query).await {
        Ok(suggestions) => {
            tracing::info!("Successfully retrieved {} suggestions", suggestions.len());
            Ok(Json(suggestions))
        },
        Err(e) => {
            tracing::error!("Failed to get suggestions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
