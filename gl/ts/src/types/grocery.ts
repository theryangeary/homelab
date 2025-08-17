export interface GroceryListEntry {
  id: number;
  completed: boolean;
  updated_at: string;
  position: number;
  quantity: string;
  notes: string;
  description: string;
  category_id: number;
}

export interface ReorderRequest {
  id: number;
  new_position?: number;
  new_category_id?: number;
}
