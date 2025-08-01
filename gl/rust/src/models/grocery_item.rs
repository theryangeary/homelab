use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroceryItem {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub position: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroceryItem {
    pub text: String,
    pub position: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroceryItem {
    pub text: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItem {
    pub id: i64,
    pub position: i64,
}