import {
  useSortable,
} from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import type { GroceryListEntry } from '../types/grocery'
import GroceryItem from './GroceryItem'

export default function SortableGroceryItem({
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
        fetchSuggestions={onFetchSuggestions}
      />
    </div>
  )
}
