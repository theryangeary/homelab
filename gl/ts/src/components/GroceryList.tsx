import { useState, useEffect } from 'react'
import {
  DndContext,
  closestCenter,
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
import {
  useSortable,
} from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import GroceryItem from './GroceryItem'
import { useGroceryList } from '../hooks/useGroceryList'
import type { GroceryListEntry } from '../types/grocery'

function SortableGroceryItem({
  entry,
  onUpdate,
  onDelete,
  onCreateBelow,
  autoFocus,
  onFetchSuggestions,
}: {
  entry: GroceryListEntry
  onUpdate: (id: number, updates: Partial<GroceryListEntry>) => void
  onDelete: (id: number) => void
  onCreateBelow: (text: string, position: number) => Promise<GroceryListEntry | undefined>
  autoFocus?: boolean
  onFetchSuggestions: (query: string) => Promise<string[]>
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ id: entry.id })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  }

  return (
    <div ref={setNodeRef} style={style} {...attributes}>
      <GroceryItem
        item={entry}
        onUpdate={onUpdate}
        onDelete={onDelete}
        onCreateBelow={onCreateBelow}
        autoFocus={autoFocus}
        dragHandleProps={listeners}
        onFetchSuggestions={onFetchSuggestions}
      />
    </div>
  )
}

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