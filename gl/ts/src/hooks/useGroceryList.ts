import { useState, useEffect, useCallback } from 'react'
import type { GroceryListEntry } from '../types/grocery'

const API_BASE = '/api'

export function useGroceryList() {
  const [entries, setEntries] = useState<GroceryListEntry[]>([])
  const [loading, setLoading] = useState(true)

  const fetchEntries = useCallback(async () => {
    try {
      const response = await fetch(`${API_BASE}/entries`)
      if (response.ok) {
        const data = await response.json()
        setEntries(data)
      } else {
        console.error('API request failed:', response.status, response.statusText)
        const text = await response.text()
        console.error('Response body:', text)
      }
    } catch (error) {
      console.error('Failed to fetch entries:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  const createEntry = useCallback(async (description: string, position: number, quantity?: string, notes?: string) => {
    try {
      const response = await fetch(`${API_BASE}/entries`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description, position, quantity, notes })
      })
      if (response.ok) {
        const newEntry = await response.json()
        setEntries(prev => [...prev, newEntry].sort((a, b) => a.position - b.position))
        return newEntry
      }
    } catch (error) {
      console.error('Failed to create entry:', error)
    }
  }, [])

  const updateEntry = useCallback(async (id: number, updates: Partial<GroceryListEntry>) => {
    try {
      const response = await fetch(`${API_BASE}/entries/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates)
      })
      if (response.ok) {
        const updatedEntry = await response.json()
        setEntries(prev => prev.map(entry => entry.id === id ? updatedEntry : entry))
      }
    } catch (error) {
      console.error('Failed to update entry:', error)
    }
  }, [])

  const deleteEntry = useCallback(async (id: number) => {
    try {
      const response = await fetch(`${API_BASE}/entries/${id}`, {
        method: 'DELETE'
      })
      if (response.ok) {
        setEntries(prev => prev.filter(entry => entry.id !== id))
      }
    } catch (error) {
      console.error('Failed to delete entry:', error)
    }
  }, [])

  const reorderEntries = useCallback(async (reorderedEntries: GroceryListEntry[]) => {
    const entriesWithNewPositions = reorderedEntries.map((entry, index) => ({
      ...entry,
      position: index
    }))

    setEntries(entriesWithNewPositions)

    try {
      await fetch(`${API_BASE}/entries/reorder`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(entriesWithNewPositions.map(entry => ({ id: entry.id, position: entry.position })))
      })
    } catch (error) {
      console.error('Failed to reorder entries:', error)
      fetchEntries()
    }
  }, [fetchEntries])

  const fetchSuggestions = useCallback(async (query: string): Promise<string[]> => {
    if (query.length === 0) {
      return []
    }

    try {
      const response = await fetch(`${API_BASE}/entries/suggestions?query=${encodeURIComponent(query)}`)
      if (response.ok) {
        const data = await response.json()
        return data
      } else {
        console.error('Failed to fetch suggestions:', response.status, response.statusText)
        return []
      }
    } catch (error) {
      console.error('Failed to fetch suggestions:', error)
      return []
    }
  }, [])

  useEffect(() => {
    fetchEntries()
  }, [fetchEntries])

  return {
    entries,
    loading,
    createEntry,
    updateEntry,
    deleteEntry,
    reorderEntries,
    fetchSuggestions
  }
}