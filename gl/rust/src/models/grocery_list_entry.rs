use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroceryListEntry {
    pub id: i64,
    pub grocery_item_id: i64,
    pub completed: bool,
    pub updated_at: DateTime<Utc>,
    pub position: i64,
    pub quantity: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroceryListEntry {
    pub grocery_item_id: i64,
    pub position: i64,
    pub quantity: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroceryListEntry {
    pub completed: Option<bool>,
    pub quantity: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderEntry {
    pub id: i64,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroceryListEntryWithItem {
    pub id: i64,
    pub grocery_item_id: i64,
    pub completed: bool,
    pub updated_at: DateTime<Utc>,
    pub position: i64,
    pub quantity: Option<String>,
    pub notes: Option<String>,
    pub description: String,
}