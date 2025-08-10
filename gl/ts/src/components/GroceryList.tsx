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
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable'
import { useEffect, useState } from 'react'
import { GroceryListRepository } from '../hooks/useGroceryList'
import SortableGroceryItem from './SortableGroceryItem'

interface GroceryListProps {
  groceryListRepository: GroceryListRepository
}

export default function GroceryList({
  groceryListRepository,
}: GroceryListProps) {
  const [lastCreatedId, setLastCreatedId] = useState<number | null>(null)

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  useEffect(() => {
    if (groceryListRepository.entries.length === 0 && !groceryListRepository.loading) {
      groceryListRepository.createEntry('', 0).then((newEntry) => {
        if (newEntry) {
          setLastCreatedId(newEntry.id)
        }
      })
    }
  }, [groceryListRepository.entries.length, groceryListRepository.loading, groceryListRepository.createEntry])

  const handleCreateBelow = async (description: string, position: number) => {
    //const entriesToUpdate = entries
    //.filter(entry => entry.position >= position)
    //.map(entry => ({ ...entry, position: entry.position + 1 }))

    const newEntry = await groceryListRepository.createEntry(description, position)
    if (newEntry) {
      setLastCreatedId(newEntry.id)
    }
    return newEntry
  }

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
      <SortableContext items={groceryListRepository.entries.map(entry => entry.id)} strategy={verticalListSortingStrategy}>
        <div className="space-y-2">
          {groceryListRepository.entries.map((entry) => (
            <SortableGroceryItem
              key={entry.id}
              entry={entry}
              onUpdate={groceryListRepository.updateEntry}
              onDelete={groceryListRepository.deleteEntry}
              onCreateBelow={handleCreateBelow}
              autoFocus={entry.id === lastCreatedId}
              onFetchSuggestions={groceryListRepository.fetchSuggestions}
            />
          ))}
        </div>
      </SortableContext>
    </DndContext>
  )
}
