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
import { useGroceryList } from '../hooks/useGroceryList'
import SortableGroceryItem from './SortableGroceryItem'


export default function GroceryList() {
  const {
    entries,
    loading,
    createEntry,
    updateEntry,
    deleteEntry,
    reorderEntries,
    fetchSuggestions
  } = useGroceryList()
  const [lastCreatedId, setLastCreatedId] = useState<number | null>(null)

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  useEffect(() => {
    if (entries.length === 0 && !loading) {
      createEntry('', 0).then((newEntry) => {
        if (newEntry) {
          setLastCreatedId(newEntry.id)
        }
      })
    }
  }, [entries.length, loading, createEntry])

  const handleCreateBelow = async (description: string, position: number) => {
    //const entriesToUpdate = entries
      //.filter(entry => entry.position >= position)
      //.map(entry => ({ ...entry, position: entry.position + 1 }))

    const newEntry = await createEntry(description, position)
    if (newEntry) {
      setLastCreatedId(newEntry.id)
    }
    return newEntry
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event

    if (over && active.id !== over.id) {
      const oldIndex = entries.findIndex((entry) => entry.id === active.id)
      const newIndex = entries.findIndex((entry) => entry.id === over.id)

      const reorderedEntries = arrayMove(entries, oldIndex, newIndex)
      reorderEntries(reorderedEntries)
    }
  }

  if (loading) {
    return <div className="text-center">Loading...</div>
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      <SortableContext items={entries.map(entry => entry.id)} strategy={verticalListSortingStrategy}>
        <div className="space-y-2">
          {entries.map((entry) => (
            <SortableGroceryItem
              key={entry.id}
              entry={entry}
              onUpdate={updateEntry}
              onDelete={deleteEntry}
              onCreateBelow={handleCreateBelow}
              autoFocus={entry.id === lastCreatedId}
              onFetchSuggestions={fetchSuggestions}
            />
          ))}
        </div>
      </SortableContext>
    </DndContext>
  )
}
