export interface Category {
    id: number;
    updated_at: string;
    position: number;
    name: string;
    is_default_category: boolean;
}

export interface ReorderRequest {
  id: number;
  new_position?: number;
}
