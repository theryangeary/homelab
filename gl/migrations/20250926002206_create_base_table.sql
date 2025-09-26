-- Add migration script here
CREATE TABLE IF NOT EXISTS grocery_list_entries(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    completed_at TIMESTAMP,
    archived_at TIMESTAMP,
    position INTEGER NOT NULL,
    quantity TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
    category_id INTEGER NOT NULL DEFAULT 1,
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
