-- Add migration script here

UPDATE grocery_list_entries 
SET archived_at = datetime(completed_at,'+1 day') 
WHERE completed_at < datetime('now','-1 day') 
AND archived_at IS NOT NULL;
