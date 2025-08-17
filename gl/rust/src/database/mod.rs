use crate::models::{
    category::{Category, CreateCategory, ReorderCategory, UpdateCategory},
    grocery_entry::{
        CreateGroceryListEntry, GroceryListEntry, ReorderEntry, UpdateGroceryListEntry,
    },
};
use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Row, Sqlite};

/// DEFAULT_CATEGORY_ID is the category of the default category, which must exist and must be the first category order by id
pub const DEFAULT_CATEGORY_ID: i64 = 1;
pub const ORDERABLE_LIST_REORDER_TEMPORARY_POSITION: i64 = 0;
pub const ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION: i64 =
    ORDERABLE_LIST_REORDER_TEMPORARY_POSITION + 1;

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
        sqlx::query(&format!(
            r#"
            CREATE TABLE IF NOT EXISTS grocery_list_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                completed_at TIMESTAMP,
                position INTEGER NOT NULL,
                quantity TEXT NOT NULL DEFAULT '',
                notes TEXT NOT NULL DEFAULT '',
                category_id INTEGER NOT NULL DEFAULT {DEFAULT_CATEGORY_ID},
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(category_id) REFERENCES categories(id)
                UNIQUE(category_id, position)
            );

            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                is_default_category BOOLEAN NOT NULL DEFAULT FALSE,
                position INTEGER NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO categories (name, is_default_category, position)
            SELECT "Uncategorized", TRUE, 0
            WHERE NOT EXISTS (SELECT 1 FROM categories WHERE id = 1);
            "#
        ))
        .bind(DEFAULT_CATEGORY_ID)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_entry(&self, id: i64) -> Result<GroceryListEntry> {
        let entries = sqlx::query_as::<_, GroceryListEntry>(
            "SELECT id, description, completed_at, position, quantity, notes, category_id, updated_at FROM grocery_list_entries WHERE id = ? LIMIT 1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(entries)
    }

    pub async fn get_all_entries(&self) -> Result<Vec<GroceryListEntry>> {
        let entries = sqlx::query_as::<_, GroceryListEntry>(
            "SELECT id, description, completed_at, position, quantity, notes, category_id, updated_at FROM grocery_list_entries ORDER BY position"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    /// get_next_position_for_item_in_category gets the next position available
    /// for an item going into the specified category (i.e. what position to
    /// append it to the end of the list)
    pub async fn get_next_position_for_item_in_category(&self, category_id: i64) -> Result<i64> {
        Ok(sqlx::query("SELECT position + 1 as next_position FROM grocery_list_entries WHERE category_id = ? ORDER BY position DESC LIMIT 1")
        .bind(category_id)
        .fetch_optional(&self.pool)
        .await?
        .map(|r|r.get("next_position"))
        .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION))
    }

    pub async fn get_last_category_for_description(
        &self,
        description: &str,
    ) -> Result<Option<i64>> {
        Ok(sqlx::query(
            "SELECT category_id FROM grocery_list_entries WHERE description = ? ORDER BY updated_at DESC LIMIT 1",
        )
        .bind(description)
        .fetch_optional(&self.pool)
        .await?
        .map(|r|r.get("category_id"))
    )
    }

    pub async fn create_entry(&self, entry: CreateGroceryListEntry) -> Result<GroceryListEntry> {
        let quantity = entry.quantity.unwrap_or_default();
        let notes = entry.notes.unwrap_or_default();
        let category_id = match entry.category_id {
            Some(c) => c,
            None => self
                .get_last_category_for_description(&entry.description)
                .await?
                .unwrap_or(1),
        };

        let row = sqlx::query(
            "INSERT INTO grocery_list_entries (description, position, quantity, notes, category_id)
             VALUES (?, ?, ?, ?, ?) 
             RETURNING id, description, completed_at, position, quantity, notes, category_id, updated_at"
        )
        .bind(&entry.description)
        .bind(entry.position)
        .bind(&quantity)
        .bind(&notes)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(GroceryListEntry {
            id: row.get("id"),
            description: row.get("description"),
            completed_at: row.get("completed_at"),
            position: row.get("position"),
            quantity: row.get("quantity"),
            notes: row.get("notes"),
            category_id: row.get("category_id"),
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
        let mut must_reorder = false;

        if let Some(description) = &entry.description {
            separated
                .push("description = ")
                .push_bind_unseparated(description);
        }
        if let Some(is_completed) = entry.completed {
            if is_completed {
                separated.push("completed_at = CURRENT_TIMESTAMP");
            } else {
                separated.push("completed_at = NULL");
            }
        }
        if let Some(quantity) = &entry.quantity {
            separated
                .push("quantity = ")
                .push_bind_unseparated(quantity);
        }
        if let Some(notes) = &entry.notes {
            separated.push("notes = ").push_bind_unseparated(notes);
        }
        if entry.category_id.is_some() || entry.position.is_some() {
            must_reorder = true;
        }

        separated.push("updated_at = CURRENT_TIMESTAMP");

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder
            .push(" RETURNING id, description, completed_at, position, quantity, notes, category_id, updated_at");

        let mut tx = self.pool.begin().await?;

        // update non-position fields of this entry
        let row = query_builder.build().fetch_one(&mut *tx).await?;

        // then update positional/category fields for this and affected entries
        if must_reorder {
            // if no position is provided, we hit this code path because we are
            // changing categories, in which case we want to default to the
            // front of the list
            let new_position = entry
                .position
                .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION);

            // if no category is provided, we are just repositioning within the category
            let new_category_id = entry.category_id.unwrap_or_else(|| row.get("category_id"));

            self.update_category_and_position(id, new_position, new_category_id, &mut tx)
                .await?;
        }

        tx.commit().await?;

        Ok(Some(self.get_entry(id).await?))
    }

    pub async fn delete_entry(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM grocery_list_entries WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// get_prior_position_and_category is a helper for reorder_entries{_with_transaction}
    async fn get_prior_position_and_category(&self, entry_id: i64) -> Result<(i64, i64)> {
        let query =
            sqlx::query("SELECT position, category_id FROM grocery_list_entries WHERE id = ?")
                .bind(entry_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!(
                        "failed to get entry for grocery_list_entries.id: {}",
                        entry_id
                    );
                    e
                })?;

        Ok((query.get("position"), query.get("category_id")))
    }

    /// reorder_entries will assign the specific entry.new_position to the entry
    /// with entry.id, adjusting all other entries in the same category up or
    /// down as needed to make room for the updated entry
    pub async fn reorder_entries(&self, reorder_request: ReorderEntry) -> Result<()> {
        let entry = self.get_entry(reorder_request.id).await?;

        // if no position is provided, we hit this code path because we are
        // changing categories, in which case we want to default to the
        // front of the list
        let new_position = reorder_request
            .new_position
            .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION);

        // if no category is provided, we are just repositioning within the category
        let new_category_id = reorder_request.new_category_id.unwrap_or(entry.category_id);

        let mut tx = self.pool.begin().await?;

        self.update_category_and_position(
            reorder_request.id,
            new_position,
            new_category_id,
            &mut tx,
        )
        .await
        .inspect_err(|e| tracing::error!("failed to update_category_and_position: {}", e))?;

        tx.commit().await?;

        Ok(())
    }

    async fn update_category_and_position(
        &self,
        entry_id: i64,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        let (prior_position, prior_category_id) = self
            .get_prior_position_and_category(entry_id)
            .await
            .inspect_err(|e| {
                tracing::error!("failed to get prior position and category id: {}", e)
            })?;

        self.remove_from_ordering(entry_id, tx)
            .await
            .inspect_err(|e| tracing::error!("failed to remove from ordering: {}", e))?;

        self.decrement_positions_gt(prior_position, prior_category_id, tx)
            .await
            .inspect_err(|e| tracing::error!("failed to decrement positions: {}", e))?;

        self.snapshot(tx).await;

        tracing::error!("incrementing >= {} in category {}", new_position, new_category_id);
        self.increment_positions_ge(new_position, new_category_id, tx)
            .await
            .inspect_err(|e| tracing::error!("failed to increment positions: {}", e))?;

        self.insert_in_ordering(entry_id, new_position, new_category_id, tx)
            .await
            .inspect_err(|e| tracing::error!("failed to insert in ordering: {}", e))?;

        Ok(())
    }

    async fn snapshot(&self, tx: &mut sqlx::Transaction<'static, Sqlite>) {
        let rows = sqlx::query("select description,category_id,position from grocery_list_entries order by category_id, position asc").fetch_all(&mut **tx).await.unwrap();
        for row in rows {
            tracing::info!("{},{},{}", row.get::<String, &str>("description"), row.get::<i64, &str>("category_id"), row.get::<i64, &str>("position"));
        }
    }
    async fn remove_from_ordering(
        &self,
        entry_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query("UPDATE grocery_list_entries SET position = ? where id = ?")
            .bind(ORDERABLE_LIST_REORDER_TEMPORARY_POSITION)
            .bind(entry_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn insert_in_ordering(
        &self,
        entry_id: i64,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query("UPDATE grocery_list_entries SET position = ?, category_id = ? where id = ?")
            .bind(new_position)
            .bind(new_category_id)
            .bind(entry_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    /// decrement_positions_gt decrements the position field if the position is greater than the provided position and the category_id matches
    async fn decrement_positions_gt(
        &self,
        prior_position: i64,
        prior_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE grocery_list_entries 
                SET position = position + 99999 
                WHERE position > ? AND category_id = ?",
        )
        .bind(prior_position)
        .bind(prior_category_id)
        .execute(&mut **tx)
        .await?;

    sqlx::query(
            "UPDATE grocery_list_entries 
                SET position = position - 100000 
                WHERE position > ? AND category_id = ?",
        )
        .bind(prior_position)
        .bind(prior_category_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// increment_positions_ge increments the position field if the position is greather than OR equal to the provided position and the category_id matches
    async fn increment_positions_ge(
        &self,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE grocery_list_entries 
                SET position = position + 100000
                WHERE position >= ? AND category_id = ?",
        )
        .bind(new_position)
        .bind(new_category_id)
        .execute(&mut **tx)
        .await?;

            sqlx::query(
            "UPDATE grocery_list_entries 
                SET position = position -99999
                WHERE position >= ? AND category_id = ?",
        )
        .bind(new_position)
        .bind(new_category_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT id, name, is_default_category, position, updated_at FROM categories ORDER BY position"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    /// get_next_position_for_category gets the next position available for a
    /// category (i.e. what position to append it to the end of the list)
    pub async fn get_next_position_for_category(&self) -> Result<i64> {
        Ok(sqlx::query(
            "SELECT position + 1 as next_position FROM categories ORDER BY position DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.get("next_position"))
        .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION))
    }

    pub async fn create_category(&self, category: CreateCategory) -> Result<Category> {
        let row = sqlx::query(
            "INSERT INTO categories (name, position) VALUES (?, ?) RETURNING id, name, is_default_category, position, updated_at"
        )
        .bind(&category.name)
        .bind(self.get_next_position_for_category().await?)
        .fetch_one(&self.pool)
        .await?;

        Ok(Category {
            id: row.get("id"),
            name: row.get("name"),
            position: row.get("position"),
            is_default_category: row.get("is_default_category"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_category(
        &self,
        id: i64,
        category: UpdateCategory,
    ) -> Result<Option<Category>> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE categories SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(name) = &category.name {
            separated.push("name = ").push_bind_unseparated(name);
        }

        separated.push("updated_at = CURRENT_TIMESTAMP");

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder.push(" RETURNING id, name, is_default_category, position, updated_at");

        let row = query_builder.build().fetch_optional(&self.pool).await?;

        if let Some(row) = row {
            Ok(Some(Category {
                id: row.get("id"),
                name: row.get("name"),
                position: row.get("position"),
                is_default_category: row.get("is_default_category"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_category(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// TODO this is not a good reordering strategy, instead adjust every row between the old position and the new position by 1
    pub async fn reorder_categories(&self, entries: Vec<ReorderCategory>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for entry in entries {
            sqlx::query(
                "UPDATE categories SET position = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
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
            }
            false => ("".to_string(), query.to_string()),
        };

        let suggestions = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT description FROM grocery_list_entries WHERE description LIKE ? AND description != '' ORDER BY description LIMIT 10"
        )
        .bind(format!("{}%", match_query))
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|s| format!("{} {}", quantity, s))
            .collect())
    }
}
