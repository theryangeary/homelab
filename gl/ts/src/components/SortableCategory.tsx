import {
  useSortable,
} from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { getLabel } from '../hooks/useCategories'
import { GroceryListRepository } from '../hooks/useGroceryList'
import type { Category as CategoryModel } from '../types/category'
import { GroceryListEntry } from '../types/grocery'
import Category from './Category'

export default function SortableCategory({
  category,
  items,
  groceryListRepository,
}: {
  category: CategoryModel,
  items: GroceryListEntry[],
  groceryListRepository: GroceryListRepository,
}) {
  const {
    // active,
    // over,
    isDragging,
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ 
    id: getLabel(category),
    data: {
      type: 'category',
      category: category,
    }
   })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : undefined,
    // scale: active && active.id === getLabel(category) ? 1.02 : 1,
  }

  return (
    <div ref={setNodeRef} style={style}>
      <Category
        category={category}
        items={items}
        groceryListRepository={groceryListRepository}
        dragHandleProps={{
          ...attributes,
          ...listeners
        }}
      />
    </div>
  )
}
