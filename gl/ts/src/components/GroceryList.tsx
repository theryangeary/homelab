import {
  closestCenter,
  DndContext,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from '@dnd-kit/core'
import {
  arrayMove,
  sortableKeyboardCoordinates
} from '@dnd-kit/sortable'
import { CategoryRepository } from '../hooks/useCategories'
import { GroceryListRepository } from '../hooks/useGroceryList'
import Category from './Category'

interface GroceryListProps {
  groceryListRepository: GroceryListRepository
  categoryRepository: CategoryRepository
}

export default function GroceryList({
  groceryListRepository,
  categoryRepository,
}: GroceryListProps) {
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event

    if (over === null) {
      return;
    }
    
    const entry = active.data.current?.entry
    const category = over.data.current?.category

    if (entry.category_id != category.id) {
      groceryListRepository.updateEntry(entry.id, {category_id: category.id})
    }

    // TODO ordering things within category is broken; probably need to implement Droppable for GroceryItem
    if (over && active.id !== over.id) {
      const oldIndex = groceryListRepository.entries.findIndex((entry) => entry.id === active.id)
      const newIndex = groceryListRepository.entries.findIndex((entry) => entry.id === over.id)

      const reorderedEntries = arrayMove(groceryListRepository.entries, oldIndex, newIndex)
      groceryListRepository.reorderEntries(reorderedEntries)
    }
  }

  if (groceryListRepository.loading) {
    return <div className="text-center">Loading...</div>
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      {categoryRepository.categories.map((category) =>
        <Category key={category.id} category={category} groceryListRepository={groceryListRepository} />
      )}
    </DndContext>
  )
}
