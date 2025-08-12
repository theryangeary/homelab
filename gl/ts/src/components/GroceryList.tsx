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
  const [lastCreatedId, setLastCreatedId] = useState<number | null>(null)

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event

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
        <Category category={category} groceryListRepository={groceryListRepository}
        />
      )}
    </DndContext>
  )
}
