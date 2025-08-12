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
  SortableContext,
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
  const [activeCategoryId, setActiveCategoryId] = useState(null);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  const sortedCategories = categoryRepository.categories.sort((a, b) => a.position - b.position);

  function handleDragStart(event) {
    if (event.active.data.current?.type === 'event') {
      setActiveCategoryId(event.active.data.current?.entry.category_id);
    } else if (event.active.data.current?.type === 'category') {
      setActiveCategoryId(event.active.data.current?.category.id);
    }
  }

  const handleDragEnd = (event: DragEndEvent) => {
    setActiveCategoryId(null);
  }

  if (groceryListRepository.loading) {
    return <div className="text-center">Loading...</div>
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
    >
      <SortableContext items={categoryRepository.categories}>
        {categoryRepository.categories.map((category) =>
          <Category key={category.id} category={category} groceryListRepository={groceryListRepository} />
        )}
      </SortableContext>

      <DragOverlay>
        {activeCategoryId ? (
          <Category key={activeCategoryId} category={categoryRepository.categories.filter((cat) => cat.id === activeCategoryId)[0]} groceryListRepository={groceryListRepository} />
        ) : null}
      </DragOverlay>
    </DndContext>
  )
}
