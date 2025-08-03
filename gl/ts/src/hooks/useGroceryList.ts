import { useState, useEffect, useCallback } from 'react'
import type { GroceryListEntry } from '../types/grocery'

const API_BASE = '/api'

export function useGroceryList() {
  const [items, setItems] = useState<GroceryListEntry[]>([])
  const [loading, setLoading] = useState(true)

  const fetchItems = useCallback(async () => {
    try {
      const response = await fetch(`${API_BASE}/entries`)
      if (response.ok) {
        const data = await response.json()
        setItems(data)
      } else {
        console.error('API request failed:', response.status, response.statusText)
        const text = await response.text()
        console.error('Response body:', text)
      }
    } catch (error) {
      console.error('Failed to fetch items:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  const createItem = useCallback(async (description: string, position: number, quantity?: string, notes?: string) => {
    try {
      const response = await fetch(`${API_BASE}/entries`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description, position, quantity, notes })
      })
      if (response.ok) {
        const newItem = await response.json()
        setItems(prev => [...prev, newItem].sort((a, b) => a.position - b.position))
        return newItem
      }
    } catch (error) {
      console.error('Failed to create item:', error)
    }
  }, [])

  const updateItem = useCallback(async (id: number, updates: Partial<GroceryListEntry>) => {
    try {
      const response = await fetch(`${API_BASE}/entries/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates)
      })
      if (response.ok) {
        const updatedItem = await response.json()
        setItems(prev => prev.map(item => item.id === id ? updatedItem : item))
      }
    } catch (error) {
      console.error('Failed to update item:', error)
    }
  }, [])

  const deleteItem = useCallback(async (id: number) => {
    try {
      const response = await fetch(`${API_BASE}/entries/${id}`, {
        method: 'DELETE'
      })
      if (response.ok) {
        setItems(prev => prev.filter(item => item.id !== id))
      }
    } catch (error) {
      console.error('Failed to delete item:', error)
    }
  }, [])

  const reorderItems = useCallback(async (reorderedItems: GroceryListEntry[]) => {
    const itemsWithNewPositions = reorderedItems.map((item, index) => ({
      ...item,
      position: index
    }))

    setItems(itemsWithNewPositions)

    try {
      await fetch(`${API_BASE}/entries/reorder`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(itemsWithNewPositions.map(item => ({ id: item.id, position: item.position })))
      })
    } catch (error) {
      console.error('Failed to reorder items:', error)
      fetchItems()
    }
  }, [fetchItems])

  useEffect(() => {
    fetchItems()
  }, [fetchItems])

  return {
    items,
    loading,
    createItem,
    updateItem,
    deleteItem,
    reorderItems
  }
}
