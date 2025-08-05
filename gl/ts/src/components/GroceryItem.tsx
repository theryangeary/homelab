import AsyncCreatableSelect from 'react-select/async-creatable';

import type { GroceryListEntry } from '../types/grocery';

interface GroceryItemProps {
  item: GroceryListEntry
  onUpdate: (id: number, updates: Partial<GroceryListEntry>) => void
  onDelete: (id: number) => void
  onCreateBelow: (text: string, position: number) => Promise<GroceryListEntry | undefined>
  fetchSuggestions: (query: string) => Promise<string[]>
  autoFocus?: boolean
  dragHandleProps?: any
}

export default function GroceryItem({
  item,
  onUpdate,
  onDelete,
  onCreateBelow,
  fetchSuggestions,
  autoFocus = false,
  dragHandleProps
}: GroceryItemProps) {
  const fullLabel = `${item.quantity} ${item.description} ${item.notes}`;

  const handleDescriptionChange = (newDescription: string) => {
    onUpdate(item.id, { description: newDescription })
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
      <div className="flex-1">
        <AsyncCreatableSelect
          value={{ label: fullLabel, value: fullLabel }}
          loadOptions={async (inputValue: string) => {
            const suggestions = await fetchSuggestions(inputValue)
            return suggestions.map(s => ({ label: s, value: s }))
          }}
          onChange={(option) => {
            console.log(option)
            if (option) {
              const newDescription = option.value
              handleDescriptionChange(newDescription)
              onCreateBelow('', item.position + 1)
            }
          }}
          onCreateOption={(inputValue: string) => {
            console.log(inputValue)
            handleDescriptionChange(inputValue)
            onCreateBelow('', item.position + 1)
          }}
          placeholder="Add item..."
          isClearable={false}
          autoFocus={autoFocus}
        />
      </div>
      <button
         onClick={handleDelete}
         className="text-gray-400 hover:text-red-500 text-lg leading-none"
         aria-label="Delete item"
      >
        ×
      </button>
    </div>
  )
}
