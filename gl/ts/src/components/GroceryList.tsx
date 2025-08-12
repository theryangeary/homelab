import {
  closestCenter,
  DndContext,
  DragOverlay,
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
import { useState } from 'react'
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
  const [activeId, setActiveId] = useState(null);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  const sortedCategories = categoryRepository.categories.sort((a, b) => a.position - b.position);

  function handleDragStart(event) {
    setActiveId(event.active.id);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    setActiveId(null);

    const { active, over } = event

    if (over === null) {
      return;
    }

    const drop_category = over.data.current?.category

    if (active.data.current?.type === 'entry') {
      const entry = active.data.current?.entry

      if (entry.category_id != drop_category.id) {
        groceryListRepository.updateEntry(entry.id, { category_id: drop_category.id })
      }
    } else if (active.data.current?.type === 'category') {
      const category = active.data.current?.category

      const oldIndex = categoryRepository.categories.findIndex((entry) => entry.id === category.id)
      const newIndex = categoryRepository.categories.findIndex((entry) => entry.id === over.id)

      const reorderedEntries = arrayMove(categoryRepository.categories, oldIndex, newIndex)
      categoryRepository.reorderCategories(reorderedEntries)
      console.log("should reorder categories");
    }

    // TODO ordering things within category is broken; probably need to implement Droppable for GroceryItem
    if (over && active.id !== over.id) {
      const oldIndex = groceryListRepository.entries.findIndex((entry) => entry.id === active.id)
      const newIndex = groceryListRepository.entries.findIndex((entry) => entry.id === over.id)

      const reorderedEntries = arrayMove(groceryListRepository.entries, oldIndex, newIndex)
      groceryListRepository.reorderEntries(reorderedEntries)
      console.log("should reorder items");
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

      <DragOverlay>
        {activeId ? (
          <Category key={activeId} category={categoryRepository.categories.filter((cat) => cat.id === activeId)[0]} groceryListRepository={groceryListRepository} />
        ) : null}
      </DragOverlay>
    </DndContext>
  )
}
