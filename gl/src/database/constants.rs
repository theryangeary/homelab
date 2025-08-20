/// DEFAULT_CATEGORY_ID is the category of the default category, which must exist and must be the first category order by id
pub const DEFAULT_CATEGORY_ID: i64 = 1;
pub const DEFAULT_CATEGORY_NAME: &str = "Uncategorized";

pub const ORDERABLE_LIST_REORDER_TEMPORARY_POSITION: i64 = 0;
pub const ORDERABLE_LIST_MINIMUM_PERMANENT_POSITION: i64 =
    ORDERABLE_LIST_REORDER_TEMPORARY_POSITION + 1;

/// MAX_NUM_POSITIONED_GROCERY_ITEMS is an assumption about the number of
/// positioned grocery items in the grocery list
/// 
/// This should be sufficiently large to never have a naturally positioned entry
/// achieve this position, but also small enough that a natural position + this
/// value will not overflow.
pub const MAX_NUM_POSITIONED_GROCERY_ITEMS: i64 = 100000;


pub const TABLE_NAME_GROCERY_LIST_ENTRIES: &str = "grocery_list_entries";
pub const TABLE_NAME_CATEGORIES: &str = "categories";

pub const GROCERY_LIST_ENTRIES_ID: &str = "id";
pub const GROCERY_LIST_ENTRIES_DESCRIPTION: &str = "description";
pub const GROCERY_LIST_ENTRIES_COMPLETED_AT: &str = "completed_at";
pub const GROCERY_LIST_ENTRIES_ARCHIVED_AT: &str = "archived_at";
pub const GROCERY_LIST_ENTRIES_POSITION: &str = "position";
pub const GROCERY_LIST_ENTRIES_QUANTITY: &str = "quantity";
pub const GROCERY_LIST_ENTRIES_NOTES: &str = "notes";
pub const GROCERY_LIST_ENTRIES_CATEGORY_ID: &str = "category_id";
pub const GROCERY_LIST_ENTRIES_UPDATED_AT: &str = "updated_at";

pub const GROCERY_LIST_ENTRIES_FIELDS: [&'static str; 9] = [
    GROCERY_LIST_ENTRIES_ID,
    GROCERY_LIST_ENTRIES_DESCRIPTION,
    GROCERY_LIST_ENTRIES_COMPLETED_AT,
    GROCERY_LIST_ENTRIES_ARCHIVED_AT,
    GROCERY_LIST_ENTRIES_POSITION,
    GROCERY_LIST_ENTRIES_QUANTITY,
    GROCERY_LIST_ENTRIES_NOTES,
    GROCERY_LIST_ENTRIES_CATEGORY_ID,
    GROCERY_LIST_ENTRIES_UPDATED_AT,
];

pub const CATEGORIES_ID: &str = "id";
pub const CATEGORIES_NAME: &str = "name";
pub const CATEGORIES_IS_DEFAULT_CATEGORY: &str = "is_default_category";
pub const CATEGORIES_POSITION: &str = "position";
pub const CATEGORIES_UPDATED_AT: &str = "updated_at";

pub const CATEGORIES_FIELDS: [&'static str; 5] = [
    CATEGORIES_ID,
    CATEGORIES_NAME,
    CATEGORIES_IS_DEFAULT_CATEGORY,
    CATEGORIES_POSITION,
    CATEGORIES_UPDATED_AT,
];
