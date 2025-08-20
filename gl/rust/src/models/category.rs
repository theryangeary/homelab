use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub position: i64,
    pub name: String,
    pub is_default_category: bool,
}


#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderCategory {
    pub id: i64,
    pub new_position: i64,
}
