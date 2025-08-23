import {
  useSortable,
} from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { GroceryListRepository } from '../hooks/useGroceryList'
import type { Category as CategoryModel } from '../types/category'
import Category from './Category'

export default function SortableCategory({
  category,
  groceryListRepository,
}: {
  category: CategoryModel
  groceryListRepository: GroceryListRepository,
}) {
  const {
    active,
    over,
    isDragging,
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ 
    id: `category-${category.id}`,
    data: {
      type: 'category',
      category: category,
    }
   })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : undefined,
  }

  return (
    <div ref={setNodeRef} style={style}>
      <Category
        category={category}
        groceryListRepository={groceryListRepository}
        dragHandleProps={{
          ...attributes,
          ...listeners
        }}
      />
    </div>
  )
}
