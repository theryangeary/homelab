use crate::models::{
    category::{Category, CreateCategory, ReorderCategory, UpdateCategory},
    grocery_entry::{
        CreateGroceryListEntry, GroceryListEntry, ReorderEntry, UpdateGroceryListEntry,
    },
};
use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, FromRow, Row, Sqlite};

mod constants;
pub use constants::DEFAULT_CATEGORY_ID;
use constants::*;

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
            CREATE TABLE IF NOT EXISTS {TABLE_NAME_GROCERY_LIST_ENTRIES} (
                {GROCERY_LIST_ENTRIES_ID} INTEGER PRIMARY KEY AUTOINCREMENT,
                {GROCERY_LIST_ENTRIES_DESCRIPTION} TEXT NOT NULL,
                {GROCERY_LIST_ENTRIES_COMPLETED_AT} TIMESTAMP,
                {GROCERY_LIST_ENTRIES_ARCHIVED_AT} TIMESTAMP,
                {GROCERY_LIST_ENTRIES_POSITION} INTEGER NOT NULL,
                {GROCERY_LIST_ENTRIES_QUANTITY} TEXT NOT NULL DEFAULT '',
                {GROCERY_LIST_ENTRIES_NOTES} TEXT NOT NULL DEFAULT '',
                {GROCERY_LIST_ENTRIES_CATEGORY_ID} INTEGER NOT NULL DEFAULT {DEFAULT_CATEGORY_ID},
                {GROCERY_LIST_ENTRIES_UPDATED_AT} TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY({GROCERY_LIST_ENTRIES_CATEGORY_ID}) REFERENCES {TABLE_NAME_CATEGORIES}({CATEGORIES_ID})
                UNIQUE({GROCERY_LIST_ENTRIES_CATEGORY_ID}, {GROCERY_LIST_ENTRIES_POSITION})
            );

            CREATE TABLE IF NOT EXISTS {TABLE_NAME_CATEGORIES} (
                {CATEGORIES_ID} INTEGER PRIMARY KEY AUTOINCREMENT,
                {CATEGORIES_NAME} TEXT NOT NULL,
                {CATEGORIES_IS_DEFAULT_CATEGORY} BOOLEAN NOT NULL DEFAULT FALSE,
                {CATEGORIES_POSITION} INTEGER NOT NULL,
                {CATEGORIES_UPDATED_AT} TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO {TABLE_NAME_CATEGORIES} ({CATEGORIES_NAME}, {CATEGORIES_IS_DEFAULT_CATEGORY}, {CATEGORIES_POSITION})
            SELECT "{DEFAULT_CATEGORY_NAME}", TRUE, 0
            WHERE NOT EXISTS (SELECT 1 FROM {TABLE_NAME_CATEGORIES} WHERE {CATEGORIES_ID} = {DEFAULT_CATEGORY_ID});
            "#
        ))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_entry(&self, id: i64) -> Result<GroceryListEntry> {
        let entries = sqlx::query_as::<_, GroceryListEntry>(
        &format!("SELECT {} FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} WHERE {GROCERY_LIST_ENTRIES_ID} = ? LIMIT 1",
                all_fields(&GROCERY_LIST_ENTRIES_FIELDS),
            )
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(entries)
    }

    pub async fn get_active_entries(&self) -> Result<Vec<GroceryListEntry>> {
        let entries = sqlx::query_as::<_, GroceryListEntry>(&format!(
            "SELECT {} FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            WHERE {GROCERY_LIST_ENTRIES_ARCHIVED_AT} IS NULL
            ORDER BY position",
            all_fields(&GROCERY_LIST_ENTRIES_FIELDS),
        ))
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    /// get_next_position_for_item_in_category gets the next position available
    /// for an item going into the specified category (i.e. what position to
    /// append it to the end of the list)
    pub async fn get_next_position_for_item_in_category(&self, category_id: i64) -> Result<i64> {
        let next_position = "next_position";
        Ok(sqlx::query(&format!(
            "SELECT {GROCERY_LIST_ENTRIES_POSITION} + 1 as {next_position}
            FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            WHERE {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ? 
            ORDER BY {GROCERY_LIST_ENTRIES_POSITION} DESC 
            LIMIT 1",
        ))
        .bind(category_id)
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.get(next_position))
        .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION))
    }

    pub async fn get_last_category_for_description(
        &self,
        description: &str,
    ) -> Result<Option<i64>> {
        Ok(sqlx::query(&format!(
            "SELECT {GROCERY_LIST_ENTRIES_CATEGORY_ID} 
            FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            WHERE {GROCERY_LIST_ENTRIES_DESCRIPTION} = ? 
            ORDER BY {GROCERY_LIST_ENTRIES_UPDATED_AT} DESC 
            LIMIT 1",
        ))
        .bind(description)
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.get(GROCERY_LIST_ENTRIES_CATEGORY_ID)))
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

        let entry = sqlx::query_as(&format!(
            "INSERT INTO {TABLE_NAME_GROCERY_LIST_ENTRIES} 
                (
                    {GROCERY_LIST_ENTRIES_DESCRIPTION},
                    {GROCERY_LIST_ENTRIES_POSITION}, 
                    {GROCERY_LIST_ENTRIES_QUANTITY}, 
                    {GROCERY_LIST_ENTRIES_NOTES}, 
                    {GROCERY_LIST_ENTRIES_CATEGORY_ID}
                )
            VALUES (?, ?, ?, ?, ?)
            RETURNING {}",
            all_fields(&GROCERY_LIST_ENTRIES_FIELDS)
        ))
        .bind(&entry.description)
        .bind(entry.position)
        .bind(&quantity)
        .bind(&notes)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }

    pub async fn update_entry(
        &self,
        id: i64,
        entry: UpdateGroceryListEntry,
    ) -> Result<Option<GroceryListEntry>> {
        let mut query_builder =
            sqlx::QueryBuilder::new(&format!("UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES} SET "));
        let mut separated = query_builder.separated(",");
        let mut must_reorder = false;

        if let Some(description) = &entry.description {
            separated
                .push(GROCERY_LIST_ENTRIES_DESCRIPTION)
                .push_unseparated(" = ")
                .push_bind_unseparated(description);
        }
        dbg!(&entry.completed);
        if let Some(is_completed) = entry.completed {
            separated.push(GROCERY_LIST_ENTRIES_COMPLETED_AT);
            if is_completed {
                separated.push_unseparated(" = CURRENT_TIMESTAMP");
            } else {
                separated.push_unseparated(" = NULL");
            }
        }
        if let Some(quantity) = &entry.quantity {
            separated
                .push(GROCERY_LIST_ENTRIES_QUANTITY)
                .push_unseparated(" = ")
                .push_bind_unseparated(quantity);
        }
        if let Some(notes) = &entry.notes {
            separated
                .push(GROCERY_LIST_ENTRIES_NOTES)
                .push_unseparated(" = ")
                .push_bind_unseparated(notes);
        }
        if entry.category_id.is_some() || entry.position.is_some() {
            must_reorder = true;
        }

        separated.push(&format!(
            "{GROCERY_LIST_ENTRIES_UPDATED_AT} = CURRENT_TIMESTAMP"
        ));

        query_builder
            .push(&format!(" WHERE {GROCERY_LIST_ENTRIES_ID} = "))
            .push_bind(id);
        query_builder.push(&format!(
            " RETURNING {}",
            all_fields(&GROCERY_LIST_ENTRIES_FIELDS)
        ));

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
            let new_category_id = entry
                .category_id
                .unwrap_or_else(|| row.get(GROCERY_LIST_ENTRIES_CATEGORY_ID));

            self.update_category_and_position_for_entry(id, new_position, new_category_id, &mut tx)
                .await?;
        }

        tx.commit().await?;

        Ok(Some(self.get_entry(id).await?))
    }

    pub async fn delete_entry(&self, id: i64) -> Result<bool> {
        let result = sqlx::query(&format!(
            "DELETE FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} WHERE id = ?"
        ))
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// get_prior_position_and_category is a helper for reorder_entries{_with_transaction}
    async fn get_prior_position_and_category(&self, entry_id: i64) -> Result<(i64, i64)> {
        let result: (i64, i64) = sqlx::query_as(&format!(
            "SELECT {GROCERY_LIST_ENTRIES_POSITION}, {GROCERY_LIST_ENTRIES_CATEGORY_ID} 
                FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} 
                WHERE {GROCERY_LIST_ENTRIES_ID} = ?"
        ))
        .bind(entry_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((result.0, result.1))
    }

    async fn get_prior_position_for_category(&self, category_id: i64) -> Result<i64> {
        let result: i64 = sqlx::query_scalar(&format!(
            "SELECT {CATEGORIES_POSITION}
                FROM {TABLE_NAME_CATEGORIES} 
                WHERE {CATEGORIES_ID} = ?"
        ))
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
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

        self.update_category_and_position_for_entry(
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

    /// reorder_categories will assign the specific category.new_position to the category
    /// with category.id, adjusting all other categories up or
    /// down as needed to make room for the updated category
    pub async fn reorder_categories(&self, reorder_request: ReorderCategory) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.update_position_for_category(
            reorder_request.id,
            reorder_request.new_position,
            &mut tx,
        )
        .await
        .inspect_err(|e| tracing::error!("failed to update_category_and_position: {}", e))?;

        tx.commit().await?;

        Ok(())
    }

    async fn update_category_and_position_for_entry(
        &self,
        entry_id: i64,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        let (prior_position, prior_category_id) =
            self.get_prior_position_and_category(entry_id).await?;

        self.remove_entry_from_ordering(entry_id, tx).await?;

        self.decrement_entry_positions_gt(prior_position, prior_category_id, tx)
            .await?;

        self.increment_entry_positions_ge(new_position, new_category_id, tx)
            .await?;

        self.insert_entry_in_ordering(entry_id, new_position, new_category_id, tx)
            .await?;

        Ok(())
    }

    async fn update_position_for_category(
        &self,
        category_id: i64,
        new_position: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        let prior_position = self.get_prior_position_for_category(category_id).await?;

        self.remove_category_from_ordering(category_id, tx).await?;

        self.decrement_category_positions_gt(prior_position, tx)
            .await?;

        self.increment_category_positions_ge(new_position, tx)
            .await?;

        self.insert_category_in_ordering(category_id, new_position, tx)
            .await
            .inspect_err(|e| tracing::error!("failed to insert category in ordering: {}", e))?;

        Ok(())
    }

    async fn remove_entry_from_ordering(
        &self,
        entry_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES}
            SET {GROCERY_LIST_ENTRIES_POSITION} = ? 
            WHERE {GROCERY_LIST_ENTRIES_ID} = ?"
        ))
        .bind(ORDERABLE_LIST_REORDER_TEMPORARY_POSITION)
        .bind(entry_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn remove_category_from_ordering(
        &self,
        category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES}
            SET {CATEGORIES_POSITION} = ? 
            WHERE {CATEGORIES_ID} = ?"
        ))
        .bind(ORDERABLE_LIST_REORDER_TEMPORARY_POSITION)
        .bind(category_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn insert_entry_in_ordering(
        &self,
        entry_id: i64,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            SET {GROCERY_LIST_ENTRIES_POSITION} = ?, {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ? WHERE
            {GROCERY_LIST_ENTRIES_ID} = ?"
        ))
        .bind(new_position)
        .bind(new_category_id)
        .bind(entry_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn insert_category_in_ordering(
        &self,
        category_id: i64,
        new_position: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES} 
            SET {CATEGORIES_POSITION} = ? WHERE
            {CATEGORIES_ID} = ?"
        ))
        .bind(new_position)
        .bind(category_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// decrement_positions_gt decrements the position field if the position is greater than the provided position and the category_id matches
    async fn decrement_entry_positions_gt(
        &self,
        prior_position: i64,
        prior_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES}
            SET {GROCERY_LIST_ENTRIES_POSITION} = {GROCERY_LIST_ENTRIES_POSITION} + {}
            WHERE {GROCERY_LIST_ENTRIES_POSITION} > ? AND {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ?",
            MAX_NUM_POSITIONED_GROCERY_ITEMS - 1
        ))
        .bind(prior_position)
        .bind(prior_category_id)
        .execute(&mut **tx)
        .await?;

        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES}
            SET {GROCERY_LIST_ENTRIES_POSITION} = {GROCERY_LIST_ENTRIES_POSITION} - {MAX_NUM_POSITIONED_GROCERY_ITEMS}
            WHERE {GROCERY_LIST_ENTRIES_POSITION} > ? AND {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ?",
        ))
        .bind(prior_position)
        .bind(prior_category_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// decrement_positions_gt decrements the position field if the position is greater than the provided position and the category_id matches
    async fn decrement_category_positions_gt(
        &self,
        prior_position: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES}
            SET {CATEGORIES_POSITION} = {CATEGORIES_POSITION} + {}
            WHERE {CATEGORIES_POSITION} > ?",
            MAX_NUM_POSITIONED_GROCERY_ITEMS - 1
        ))
        .bind(prior_position)
        .execute(&mut **tx)
        .await?;

        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES}
            SET {CATEGORIES_POSITION} = {CATEGORIES_POSITION} - {MAX_NUM_POSITIONED_GROCERY_ITEMS}
            WHERE {CATEGORIES_POSITION} > ?",
        ))
        .bind(prior_position)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// increment_positions_ge increments the position field if the position is greather than OR equal to the provided position and the category_id matches
    async fn increment_entry_positions_ge(
        &self,
        new_position: i64,
        new_category_id: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES}
            SET {GROCERY_LIST_ENTRIES_POSITION} = {GROCERY_LIST_ENTRIES_POSITION} + {MAX_NUM_POSITIONED_GROCERY_ITEMS}
            WHERE {GROCERY_LIST_ENTRIES_POSITION} >= ? AND {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ?",
        ))
        .bind(new_position)
        .bind(new_category_id)
        .execute(&mut **tx)
        .await?;

        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES}
            SET {GROCERY_LIST_ENTRIES_POSITION} = {GROCERY_LIST_ENTRIES_POSITION} - {}
            WHERE {GROCERY_LIST_ENTRIES_POSITION} >= ? AND {GROCERY_LIST_ENTRIES_CATEGORY_ID} = ?",
            MAX_NUM_POSITIONED_GROCERY_ITEMS - 1
        ))
        .bind(new_position)
        .bind(new_category_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// increment_positions_ge increments the position field if the position is greather than OR equal to the provided position and the category_id matches
    async fn increment_category_positions_ge(
        &self,
        new_position: i64,
        tx: &mut sqlx::Transaction<'static, Sqlite>,
    ) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES}
            SET {CATEGORIES_POSITION} = {CATEGORIES_POSITION} + {MAX_NUM_POSITIONED_GROCERY_ITEMS}
            WHERE {CATEGORIES_POSITION} >= ?",
        ))
        .bind(new_position)
        .execute(&mut **tx)
        .await?;

        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_CATEGORIES}
            SET {CATEGORIES_POSITION} = {CATEGORIES_POSITION} - {}
            WHERE {CATEGORIES_POSITION} >= ?",
            MAX_NUM_POSITIONED_GROCERY_ITEMS - 1
        ))
        .bind(new_position)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_category(&self, id: i64) -> Result<Category> {
        Ok(sqlx::query_as(&format!(
            "SELECT {} FROM {TABLE_NAME_CATEGORIES} WHERE {CATEGORIES_ID} = ? LIMIT 1",
            all_fields(&CATEGORIES_FIELDS),
        ))
        .bind(id)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>> {
        let categories = sqlx::query_as::<_, Category>(&format!(
            "SELECT {} FROM {TABLE_NAME_CATEGORIES} ORDER BY {CATEGORIES_POSITION}",
            all_fields(&CATEGORIES_FIELDS)
        ))
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    /// get_next_position_for_category gets the next position available for a
    /// category (i.e. what position to append it to the end of the list)
    pub async fn get_next_position_for_category(&self) -> Result<i64> {
        let next_position = "next_position";
        Ok(sqlx::query(&format!(
            "SELECT {CATEGORIES_POSITION} + 1 as {next_position} 
            FROM {TABLE_NAME_CATEGORIES} 
            ORDER BY {CATEGORIES_POSITION} DESC 
            LIMIT 1",
        ))
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.get(next_position))
        .unwrap_or(ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION))
    }

    pub async fn create_category(&self, category: CreateCategory) -> Result<Category> {
        let category = sqlx::query_as(&format!(
            "INSERT INTO {TABLE_NAME_CATEGORIES} 
            ({CATEGORIES_NAME}, {CATEGORIES_POSITION}) 
            VALUES (?, ?) 
            RETURNING {}",
            all_fields(&CATEGORIES_FIELDS)
        ))
        .bind(&category.name)
        .bind(self.get_next_position_for_category().await?)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    pub async fn update_category(
        &self,
        id: i64,
        category: UpdateCategory,
    ) -> Result<Option<Category>> {
        let mut query_builder =
            sqlx::QueryBuilder::new(&format!("UPDATE {TABLE_NAME_CATEGORIES} SET "));
        let mut separated = query_builder.separated(", ");

        if let Some(name) = &category.name {
            separated
                .push(CATEGORIES_NAME)
                .push(" = ")
                .push_bind_unseparated(name);
        }

        separated
            .push(CATEGORIES_UPDATED_AT)
            .push(" = CURRENT_TIMESTAMP");

        query_builder
            .push(&format!(" WHERE {CATEGORIES_ID} = "))
            .push_bind(id);
        query_builder.push(" RETURNING ");
        query_builder.push(all_fields(&CATEGORIES_FIELDS));

        let row = query_builder.build().fetch_optional(&self.pool).await?;

        if let Some(row) = row {
            Ok(Some(Category::from_row(&row)?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_category(&self, id: i64) -> Result<bool> {
        let result = sqlx::query(&format!(
            "DELETE FROM {TABLE_NAME_CATEGORIES} WHERE {CATEGORIES_ID} = ?"
        ))
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
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

        let suggestions = sqlx::query_scalar::<_, String>(&format!(
            "SELECT DISTINCT {GROCERY_LIST_ENTRIES_DESCRIPTION} 
            FROM {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            WHERE {GROCERY_LIST_ENTRIES_DESCRIPTION} LIKE ? AND {GROCERY_LIST_ENTRIES_DESCRIPTION} != '' 
            ORDER BY {GROCERY_LIST_ENTRIES_DESCRIPTION} 
            LIMIT 10"
        ))
        .bind(format!("{}%", match_query))
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|s| format!("{} {}", quantity, s))
            .collect())
    }

    pub async fn archive_entries(&self) -> Result<()> {
        sqlx::query(&format!(
            "UPDATE {TABLE_NAME_GROCERY_LIST_ENTRIES} 
            SET {GROCERY_LIST_ENTRIES_ARCHIVED_AT} = CURRENT_TIME
            WHERE {GROCERY_LIST_ENTRIES_COMPLETED_AT} < datetime('now','-1 day')"
        ))
        .execute(&self.pool)
        .await?;

        tracing::debug!("archived entries");
        Ok(())
    }
}

fn all_fields(field_list: &[&str]) -> String {
    field_list.join(", ")
}
