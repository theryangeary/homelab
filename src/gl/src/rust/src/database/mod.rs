use sqlx::{sqlite::SqlitePool, migrate::MigrateDatabase, Sqlite, Row};
use anyhow::Result;
use crate::models::grocery_item::{GroceryItem, CreateGroceryItem, UpdateGroceryItem, ReorderItem};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            Sqlite::create_database(database_url).await?;
        }

        let pool = SqlitePool::connect(database_url).await?;
        
        let db = Database { pool };
        db.migrate().await?;
        
        Ok(db)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS grocery_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                position INTEGER NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_items(&self) -> Result<Vec<GroceryItem>> {
        let items = sqlx::query_as::<_, GroceryItem>(
            "SELECT id, text, completed, position, updated_at FROM grocery_items ORDER BY position"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn create_item(&self, item: CreateGroceryItem) -> Result<GroceryItem> {
        let row = sqlx::query(
            "INSERT INTO grocery_items (text, position) VALUES (?, ?) RETURNING id, text, completed, position, updated_at"
        )
        .bind(&item.text)
        .bind(item.position)
        .fetch_one(&self.pool)
        .await?;

        Ok(GroceryItem {
            id: row.get("id"),
            text: row.get("text"),
            completed: row.get("completed"),
            position: row.get("position"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_item(&self, id: i64, item: UpdateGroceryItem) -> Result<Option<GroceryItem>> {
        match (item.text, item.completed) {
            (Some(text), Some(completed)) => {
                let row = sqlx::query(
                    "UPDATE grocery_items SET text = ?, completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, text, completed, position, updated_at"
                )
                .bind(text)
                .bind(completed)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(row) = row {
                    Ok(Some(GroceryItem {
                        id: row.get("id"),
                        text: row.get("text"),
                        completed: row.get("completed"),
                        position: row.get("position"),
                        updated_at: row.get("updated_at"),
                    }))
                } else {
                    Ok(None)
                }
            },
            (Some(text), None) => {
                let row = sqlx::query(
                    "UPDATE grocery_items SET text = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, text, completed, position, updated_at"
                )
                .bind(text)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(row) = row {
                    Ok(Some(GroceryItem {
                        id: row.get("id"),
                        text: row.get("text"),
                        completed: row.get("completed"),
                        position: row.get("position"),
                        updated_at: row.get("updated_at"),
                    }))
                } else {
                    Ok(None)
                }
            },
            (None, Some(completed)) => {
                let row = sqlx::query(
                    "UPDATE grocery_items SET completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, text, completed, position, updated_at"
                )
                .bind(completed)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(row) = row {
                    Ok(Some(GroceryItem {
                        id: row.get("id"),
                        text: row.get("text"),
                        completed: row.get("completed"),
                        position: row.get("position"),
                        updated_at: row.get("updated_at"),
                    }))
                } else {
                    Ok(None)
                }
            },
            (None, None) => {
                // Just update timestamp
                let row = sqlx::query(
                    "UPDATE grocery_items SET updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, text, completed, position, updated_at"
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(row) = row {
                    Ok(Some(GroceryItem {
                        id: row.get("id"),
                        text: row.get("text"),
                        completed: row.get("completed"),
                        position: row.get("position"),
                        updated_at: row.get("updated_at"),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn delete_item(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM grocery_items WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn reorder_items(&self, items: Vec<ReorderItem>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for item in items {
            sqlx::query("UPDATE grocery_items SET position = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(item.position)
                .bind(item.id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}