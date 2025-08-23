import {
    useSortable,
} from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { getLabel } from '../hooks/useGroceryList'
import type { GroceryListEntry } from '../types/grocery'
import GroceryItem from './GroceryItem'

export default function SortableGroceryItem({
    entry,
    onUpdate,
    onDelete,
    autoFocus,
    onFetchSuggestions,
}: {
    entry: GroceryListEntry
    onUpdate: (id: number, updates: Partial<GroceryListEntry>) => void
    onDelete: (id: number) => void
    autoFocus?: boolean
    onFetchSuggestions: (query: string) => Promise<string[]>
}) {
    const {
        active, 
        over, 
        isDragging,
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
    } = useSortable({
        id: getLabel(entry),
        data: {
            type: 'entry',
            entry: entry
        }
    })

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
        opacity: isDragging ? 0.5 : undefined,
        // scale: active && active.id === getLabel(entry) ? 1.02 : 1,
    }

    return (
        <div ref={setNodeRef} style={style}>
            <GroceryItem
                item={entry}
                onUpdate={onUpdate}
                onDelete={onDelete}
                autoFocus={autoFocus}
                dragHandleProps={{
                    ...listeners,
                    ...attributes,
                }}
                onFetchSuggestions={onFetchSuggestions}
            />
        </div>
    )
}
