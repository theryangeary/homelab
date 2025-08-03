export interface GroceryItem {
  id: number;
  description: string;
}

export interface GroceryListEntry {
  id: number;
  completed: boolean;
  updated_at: string;
  position: number;
  quantity: string;
  notes: string;
  description: string;
}