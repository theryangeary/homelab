use crate::models::grocery_entry::{
    CreateGroceryListEntry, GroceryListEntry, ReorderEntry, UpdateGroceryListEntry,
};
use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Row, Sqlite};

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
            CREATE TABLE IF NOT EXISTS grocery_list_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                position INTEGER NOT NULL,
                quantity TEXT NOT NULL DEFAULT '',
                notes TEXT NOT NULL DEFAULT '',
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_entries(&self) -> Result<Vec<GroceryListEntry>> {
        let entries = sqlx::query_as::<_, GroceryListEntry>(
            "SELECT id, description, completed, position, quantity, notes, updated_at FROM grocery_list_entries ORDER BY position"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    pub async fn create_entry(&self, entry: CreateGroceryListEntry) -> Result<GroceryListEntry> {
        let quantity = entry.quantity.unwrap_or_default();
        let notes = entry.notes.unwrap_or_default();

        let row = sqlx::query(
            "INSERT INTO grocery_list_entries (description, position, quantity, notes) VALUES (?, ?, ?, ?) RETURNING id, description, completed, position, quantity, notes, updated_at"
        )
        .bind(&entry.description)
        .bind(entry.position)
        .bind(&quantity)
        .bind(&notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(GroceryListEntry {
            id: row.get("id"),
            description: row.get("description"),
            completed: row.get("completed"),
            position: row.get("position"),
            quantity: row.get("quantity"),
            notes: row.get("notes"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_entry(
        &self,
        id: i64,
        entry: UpdateGroceryListEntry,
    ) -> Result<Option<GroceryListEntry>> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE grocery_list_entries SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(description) = &entry.description {
            separated
                .push("description = ")
                .push_bind_unseparated(description);
        }
        if let Some(completed) = entry.completed {
            separated
                .push("completed = ")
                .push_bind_unseparated(completed);
        }
        if let Some(quantity) = &entry.quantity {
            separated
                .push("quantity = ")
                .push_bind_unseparated(quantity);
        }
        if let Some(notes) = &entry.notes {
            separated.push("notes = ").push_bind_unseparated(notes);
        }

        separated.push("updated_at = CURRENT_TIMESTAMP");

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder
            .push(" RETURNING id, description, completed, position, quantity, notes, updated_at");

        let row = query_builder.build().fetch_optional(&self.pool).await?;

        if let Some(row) = row {
            Ok(Some(GroceryListEntry {
                id: row.get("id"),
                description: row.get("description"),
                completed: row.get("completed"),
                position: row.get("position"),
                quantity: row.get("quantity"),
                notes: row.get("notes"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_entry(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM grocery_list_entries WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn reorder_entries(&self, entries: Vec<ReorderEntry>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for entry in entries {
            sqlx::query("UPDATE grocery_list_entries SET position = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(entry.position)
                .bind(entry.id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_suggestions(&self, query: &str) -> Result<Vec<String>> {
        let has_quantity = if let Some(first_word) = query.split_whitespace().next() {
            first_word.chars().next().unwrap_or('a').is_numeric()
        } else {
            false
        };

        let (quantity, match_query) = match has_quantity {
            true => {
                let mut words = query.split_whitespace();
                (words.next().unwrap().to_string(), words.collect())
            },
            false => ("".to_string(), query.to_string()),
        };

        let suggestions = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT description FROM grocery_list_entries WHERE description LIKE ? AND description != '' ORDER BY description LIMIT 10"
        )
        .bind(format!("{}%", match_query))
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions.into_iter().map(|s| format!("{} {}", quantity, s)).collect())
    }
}
