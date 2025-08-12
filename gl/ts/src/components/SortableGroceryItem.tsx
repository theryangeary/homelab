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
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
    } = useSortable({
        id: `entry-{entry.id}`,
        data: {
            type: 'entry',
            entry: entry
        }
    })

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
                autoFocus={autoFocus}
                dragHandleProps={listeners}
                onFetchSuggestions={onFetchSuggestions}
            />
        </div>
    )
}
