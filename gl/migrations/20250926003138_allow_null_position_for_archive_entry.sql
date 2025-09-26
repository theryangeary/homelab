-- Add migration script here

-- this has to be done in a transaction to safely migrate to new table, however
-- sqlx auto-wraps each migration in a transaction for us

-- create a new table 
CREATE TABLE IF NOT EXISTS grocery_list_entries_tmp (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    completed_at TIMESTAMP,
    archived_at TIMESTAMP,
    position INTEGER,
    quantity TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
    category_id INTEGER NOT NULL DEFAULT 1,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(category_id) REFERENCES categories(id)
    UNIQUE(category_id, position)
    CHECK ((archived_at IS NULL) <> (position IS NULL))
);
-- copy data from old table to the new one
INSERT INTO grocery_list_entries_tmp 
    SELECT id, description, completed_at, archived_at,
        CASE WHEN archived_at IS NULL THEN gle.position ELSE NULL END as position, 
        quantity, notes, category_id, updated_at 
    FROM grocery_list_entries gle;

-- drop the old table
DROP TABLE grocery_list_entries;

-- rename new table to the old one
ALTER TABLE grocery_list_entries_tmp RENAME TO grocery_list_entries;
