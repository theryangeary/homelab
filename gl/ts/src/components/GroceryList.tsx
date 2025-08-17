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
import { Category as CategoryModel } from '../types/category'
import { GroceryListEntry } from '../types/grocery'
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

  const handleDragEnd = (event: DragEndEvent) => {
    setActive(null);

    var newPosition = undefined;
    var newCategoryId = undefined;

    if (event.active.data.current?.type === 'entry') {
      const draggedEntry: GroceryListEntry = event.active.data.current?.entry;
      
      // if dropped on another entry, take that entry's position AND category
      if (event.over?.data.current?.type === 'entry') {
        const dropEntry: GroceryListEntry = event.over?.data.current?.entry
        newPosition = dropEntry.position;
        if (draggedEntry.category_id !== dropEntry.category_id) {
          newCategoryId = dropEntry.category_id
        }
      // if dropped on a category, take that category's id
      } else if (event.over?.data.current?.type === 'category') {
        const dropCategory: CategoryModel = event.over?.data.current?.category
        newCategoryId = dropCategory.id
      }

      groceryListRepository.reorderEntries(draggedEntry.id, newPosition, newCategoryId);
    } else {
      console.log("type issue?")
    }

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
            dragHandleProps={undefined} />
        ) : null}
      </DragOverlay>
    </DndContext>
  )
}
