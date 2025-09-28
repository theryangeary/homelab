-- Add migration script here

CREATE TABLE IF NOT EXISTS categories_tmp (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    is_default_category BOOLEAN NOT NULL DEFAULT FALSE,
    position INTEGER NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO categories_tmp 
    SELECT 
        id, 
        CASE 
            WHEN (SELECT COUNT(*) FROM categories WHERE name=c.name) == 1
            THEN name
            ELSE name || id
        END as name, 
        is_default_category,
        position,
        updated_at
    FROM categories c;

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
    FOREIGN KEY(category_id) REFERENCES categories_tmp(id)
    UNIQUE(category_id, position)
    CHECK ((archived_at IS NULL) <> (position IS NULL))
);
-- copy data from old table to the new one
INSERT INTO grocery_list_entries_tmp SELECT * FROM grocery_list_entries gle;

-- drop the old table
DROP TABLE grocery_list_entries;

-- rename new table to the old one
ALTER TABLE grocery_list_entries_tmp RENAME TO grocery_list_entries;

-- drop old table
DROP TABLE categories;

-- rename new table to the old one. at this point foreign keys should point to new table
ALTER TABLE categories_tmp RENAME TO categories;


