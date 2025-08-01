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
import type { GroceryItem as GroceryItemType } from '../types/grocery'

function SortableGroceryItem({
  item,
  onUpdate,
  onDelete,
  onCreateBelow,
  autoFocus,
}: {
  item: GroceryItemType
  onUpdate: (id: number, updates: Partial<GroceryItemType>) => void
  onDelete: (id: number) => void
  onCreateBelow: (text: string, position: number) => Promise<GroceryItemType | undefined>
  autoFocus?: boolean
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ id: item.id })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  }

  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      <GroceryItem
        item={item}
        onUpdate={onUpdate}
        onDelete={onDelete}
        onCreateBelow={onCreateBelow}
        autoFocus={autoFocus}
      />
    </div>
  )
}

export default function GroceryList() {
  const { items, loading, createItem, updateItem, deleteItem, reorderItems } = useGroceryList()
  const [lastCreatedId, setLastCreatedId] = useState<number | null>(null)

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  useEffect(() => {
    if (items.length === 0 && !loading) {
      createItem('', 0).then((newItem) => {
        if (newItem) {
          setLastCreatedId(newItem.id)
        }
      })
    }
  }, [items.length, loading, createItem])

  const handleCreateBelow = async (text: string, position: number) => {
    //const itemsToUpdate = items
      //.filter(item => item.position >= position)
      //.map(item => ({ ...item, position: item.position + 1 }))

    const newItem = await createItem(text, position)
    if (newItem) {
      setLastCreatedId(newItem.id)
    }
    return newItem
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event

    if (over && active.id !== over.id) {
      const oldIndex = items.findIndex((item) => item.id === active.id)
      const newIndex = items.findIndex((item) => item.id === over.id)

      const reorderedItems = arrayMove(items, oldIndex, newIndex)
      reorderItems(reorderedItems)
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
      <SortableContext items={items.map(item => item.id)} strategy={verticalListSortingStrategy}>
        <div className="space-y-2">
          {items.map((item) => (
            <SortableGroceryItem
              key={item.id}
              item={item}
              onUpdate={updateItem}
              onDelete={deleteItem}
              onCreateBelow={handleCreateBelow}
              autoFocus={item.id === lastCreatedId}
            />
          ))}
        </div>
      </SortableContext>
    </DndContext>
  )
}
