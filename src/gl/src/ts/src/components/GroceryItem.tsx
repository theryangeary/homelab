import { useState, useRef, useEffect } from 'react'
import type { GroceryItem as GroceryItemType } from '../types/grocery'

interface GroceryItemProps {
  item: GroceryItemType
  onUpdate: (id: number, updates: Partial<GroceryItemType>) => void
  onDelete: (id: number) => void
  onCreateBelow: (text: string, position: number) => Promise<GroceryItemType | undefined>
  onFocus?: () => void
  autoFocus?: boolean
}

export default function GroceryItem({
  item,
  onUpdate,
  onDelete,
  onCreateBelow,
  onFocus,
  autoFocus = false
}: GroceryItemProps) {
  const [text, setText] = useState(item.text)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (autoFocus && inputRef.current) {
      inputRef.current.focus()
    }
  }, [autoFocus])

  const handleTextChange = (newText: string) => {
    setText(newText)
    onUpdate(item.id, { text: newText })
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
      <input
        type="checkbox"
        checked={item.completed}
        onChange={(e) => handleCheckboxChange(e.target.checked)}
        className="w-4 h-4"
      />
      <input
        ref={inputRef}
        type="text"
        value={text}
        onChange={(e) => handleTextChange(e.target.value)}
        onKeyDown={handleKeyDown}
        onFocus={onFocus}
        placeholder="Enter grocery item..."
        className="flex-1 px-2 py-1 border-none outline-none"
      />
      {text === '' && (
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