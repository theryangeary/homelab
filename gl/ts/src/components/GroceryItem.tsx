import { useState, useRef, useEffect } from 'react'
import type { GroceryListEntry } from '../types/grocery'

interface GroceryItemProps {
  item: GroceryListEntry
  onUpdate: (id: number, updates: Partial<GroceryListEntry>) => void
  onDelete: (id: number) => void
  onCreateBelow: (text: string, position: number) => Promise<GroceryListEntry | undefined>
  onFocus?: () => void
  autoFocus?: boolean
  dragHandleProps?: any
}

export default function GroceryItem({
  item,
  onUpdate,
  onDelete,
  onCreateBelow,
  onFocus,
  autoFocus = false,
  dragHandleProps
}: GroceryItemProps) {
  const [description, setDescription] = useState(item.description)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (autoFocus && inputRef.current) {
      inputRef.current.focus()
    }
  }, [autoFocus])

  const handleDescriptionChange = (newDescription: string) => {
    setDescription(newDescription)
    onUpdate(item.id, { description: newDescription })
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      onCreateBelow('', item.position + 1)
    }
  }

  const handleCheckboxChange = (completed: boolean) => {
    onUpdate(item.id, { completed })
  }

  const handleDelete = () => {
    onDelete(item.id)
  }

  return (
    <div className="flex items-center gap-2 p-2 border border-gray-200 rounded">
      <div
        {...dragHandleProps}
        className="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600 px-1"
        title="Drag to reorder"
      >
        ⋮⋮
      </div>
      <input
        type="checkbox"
        checked={item.completed}
        onChange={(e) => handleCheckboxChange(e.target.checked)}
        className="w-4 h-4"
      />
      <input
        ref={inputRef}
        type="text"
        value={description}
        onChange={(e) => handleDescriptionChange(e.target.value)}
        onKeyDown={handleKeyDown}
        onFocus={onFocus}
        placeholder="Enter grocery item..."
        className="flex-1 px-2 py-1 border-none outline-none"
      />
      {description === '' && (
        <button
          onClick={handleDelete}
          className="text-gray-400 hover:text-red-500 text-lg leading-none"
          aria-label="Delete item"
        >
          ×
        </button>
      )}
    </div>
  )
}
