import {
  closestCenter,
  DataRef,
  DndContext,
  DragOverlay,
  DragStartEvent,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent
} from '@dnd-kit/core'
import {
  SortableContext,
  sortableKeyboardCoordinates
} from '@dnd-kit/sortable'
import { useState } from 'react'
import { CategoryRepository } from '../hooks/useCategories'
import { GroceryListRepository } from '../hooks/useGroceryList'
import Category from './Category'
import GroceryItem from './GroceryItem'
import SortableCategory from './SortableCategory'

interface GroceryListProps {
  groceryListRepository: GroceryListRepository
  categoryRepository: CategoryRepository
}

export default function GroceryList({
  groceryListRepository,
  categoryRepository,
}: GroceryListProps) {
  const [active, setActive] = useState<DataRef | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  const sortedCategories = categoryRepository.categories.sort((a, b) => a.position - b.position);
  const sortedCategoryIds = sortedCategories.map((category) => `category-${category.id}`);

  function handleDragStart(event: DragStartEvent) {
    setActive(event.active.data);
  }

  const handleDragEnd = (_event: DragEndEvent) => {
    setActive(null);
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
      <SortableContext items={sortedCategoryIds}>
        {categoryRepository.categories.map((category) =>
          <SortableCategory key={category.id} category={category} groceryListRepository={groceryListRepository} />
        )}
      </SortableContext>

      <DragOverlay>
        {active && active.current?.type === 'category' ? (
          <Category category={categoryRepository.categories.filter((cat) => cat.id === active.current?.category.id)[0]} groceryListRepository={groceryListRepository} />
        ) : null}
        {active && active.current?.type === 'entry' ? (
          <GroceryItem
            item={groceryListRepository.entries.filter((entry) => entry.id === active.current?.entry.id)[0]}
            onDelete={groceryListRepository.deleteEntry}
            onFetchSuggestions={groceryListRepository.fetchSuggestions}
            onUpdate={groceryListRepository.updateEntry} 
            dragHandleProps={undefined}            />
        ) : null}
      </DragOverlay>
    </DndContext>
  )
}
