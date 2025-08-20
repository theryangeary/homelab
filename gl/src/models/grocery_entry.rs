use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroceryListEntry {
    pub id: i64,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub position: i64,
    pub quantity: String,
    pub notes: String,
    pub category_id: i64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroceryListEntry {
    pub description: String,
    pub position: Option<i64>,
    pub quantity: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroceryListEntry {
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub quantity: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<i64>,
    pub position: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderEntry {
    pub id: i64,
    pub new_position: Option<i64>,
    pub new_category_id: Option<i64>,
}
