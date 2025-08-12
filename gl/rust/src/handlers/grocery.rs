use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::database::Database;
use crate::models::grocery_entry::{CreateGroceryListEntry, GroceryListEntry, ReorderEntry, UpdateGroceryListEntry};

#[derive(Deserialize)]
pub struct SuggestionsQuery {
    query: String,
}

fn parse_entry_input(input: &str) -> (String, String, String) {
    let quantity_regex = regex::Regex::new(r"^(\d+[a-z]*)\s+(.+)$").unwrap();

    if let Some(captures) = quantity_regex.captures(input.trim()) {
        let quantity = captures.get(1).unwrap().as_str().to_string();
        let rest = captures.get(2).unwrap().as_str();

        // Check for notes in parentheses or after dash
        if let Some(dash_pos) = rest.find(" - ") {
            let description = rest[..dash_pos].trim().to_string();
            let notes = rest[dash_pos + 3..].trim().to_string();
            (quantity, description, notes)
        } else if let Some(paren_start) = rest.find(" (") {
            if let Some(paren_end) = rest.rfind(')') {
                let description = rest[..paren_start].trim().to_string();
                let notes = rest[paren_start + 2..paren_end].trim().to_string();
                (quantity, description, notes)
            } else {
                (quantity, rest.to_string(), String::new())
            }
        } else {
            (quantity, rest.to_string(), String::new())
        }
    } else {
        // No quantity found, check for notes in the whole input
        if let Some(dash_pos) = input.find(" - ") {
            let description = input[..dash_pos].trim().to_string();
            let notes = input[dash_pos + 3..].trim().to_string();
            (String::new(), description, notes)
        } else if let Some(paren_start) = input.find(" (") {
            if let Some(paren_end) = input.rfind(')') {
                let description = input[..paren_start].trim().to_string();
                let notes = input[paren_start + 2..paren_end].trim().to_string();
                (String::new(), description, notes)
            } else {
                (String::new(), input.trim().to_string(), String::new())
            }
        } else {
            (String::new(), input.trim().to_string(), String::new())
        }
    }
}

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

    // Parse the raw input if no quantity/notes are provided
    let (quantity, description, notes) = if payload.quantity.is_none() && payload.notes.is_none() {
        let (parsed_quantity, parsed_description, parsed_notes) = parse_entry_input(&payload.description);
        (
            if parsed_quantity.is_empty() { None } else { Some(parsed_quantity) },
            parsed_description,
            if parsed_notes.is_empty() { None } else { Some(parsed_notes) }
        )
    } else {
        (payload.quantity, payload.description, payload.notes)
    };

    let parsed_payload = CreateGroceryListEntry {
        description,
        position: payload.position,
        quantity,
        notes,
        category_id: payload.category_id,
    };

    match db.create_entry(parsed_payload).await {
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
    // Parse the description if it's provided but quantity/notes are not
    let parsed_payload = if let Some(ref description) = payload.description {
        if payload.quantity.is_none() && payload.notes.is_none() {
            let (parsed_quantity, parsed_description, parsed_notes) = parse_entry_input(description);
            UpdateGroceryListEntry {
                description: Some(parsed_description),
                completed: payload.completed,
                quantity: if parsed_quantity.is_empty() { None } else { Some(parsed_quantity) },
                notes: if parsed_notes.is_empty() { None } else { Some(parsed_notes) },
                category_id: payload.category_id,
            }
        } else {
            payload
        }
    } else {
        payload
    };

    match db.update_entry(id, parsed_payload).await {
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
    Json(payload): Json<ReorderEntry>,
) -> Result<StatusCode, StatusCode> {
    match db.reorder_entries(payload).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {tracing::error!("Failed to reorder: {}", e); Err(StatusCode::INTERNAL_SERVER_ERROR)},
    }
}

pub async fn get_suggestions(
    State(db): State<Arc<Database>>,
    Query(params): Query<SuggestionsQuery>,
) -> Result<Json<Vec<String>>, StatusCode> {
    tracing::info!("GET /api/entries/suggestions called with query: '{}'", params.query);
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
