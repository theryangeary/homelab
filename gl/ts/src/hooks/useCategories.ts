import { useCallback, useEffect, useState } from 'react'
import type { Category } from '../types/category'

const API_BASE = '/api'

export type CategoryRepository = {
  categories: Category[],
  loading: boolean,
  createCategory: (name: string) => Promise<any>,
  updateCategory: (id: number, updates: Partial<Category>) => Promise<any>,
  deleteCategory: (id: number) => Promise<any>,
  reorderCategories: (reorderedCategories: Category[]) => Promise<any>,
  fetchSuggestions: (query: string) => Promise<string[]>,
}


export function useCategories(): CategoryRepository {
  const [categories, setCategories] = useState<Category[]>([])
  const [loading, setLoading] = useState(true)

  const fetchCategories = useCallback(async () => {
    try {
      const response = await fetch(`${API_BASE}/categories`)
      if (response.ok) {
        const data = await response.json()
        setCategories(data)
      } else {
        console.error('API request failed:', response.status, response.statusText)
        const text = await response.text()
        console.error('Response body:', text)
      }
    } catch (error) {
      console.error('Failed to fetch categories:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  const createCategory = useCallback(async (name: string) => {
    try {
      const response = await fetch(`${API_BASE}/categories`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name })
      })
      if (response.ok) {
        const newCategory = await response.json()
        setCategories(prev => [...prev, newCategory].sort((a, b) => a.position - b.position))
        return newCategory
      }
    } catch (error) {
      console.error('Failed to create category:', error)
    }
  }, [])

  const updateCategory = useCallback(async (id: number, updates: Partial<Category>) => {
    try {
      const response = await fetch(`${API_BASE}/categories/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates)
      })
      if (response.ok) {
        const updatedCategory = await response.json()
        setCategories(prev => prev.map(category => category.id === id ? updatedCategory : category))
      }
    } catch (error) {
      console.error('Failed to update category:', error)
    }
  }, [])

  const deleteCategory = useCallback(async (id: number) => {
    try {
      const response = await fetch(`${API_BASE}/categories/${id}`, {
        method: 'DELETE'
      })
      if (response.ok) {
        setCategories(prev => prev.filter(category => category.id !== id))
      }
    } catch (error) {
      console.error('Failed to delete category:', error)
    }
  }, [])

  const reorderCategories = useCallback(async (reorderedCategories: Category[]) => {
    const categoriesWithNewPositions = reorderedCategories.map((category, index) => ({
      ...category,
      position: index
    }))

    setCategories(categoriesWithNewPositions)

    try {
      await fetch(`${API_BASE}/categories/reorder`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(categoriesWithNewPositions.map(category => ({ id: category.id, position: category.position })))
      })
    } catch (error) {
      console.error('Failed to reorder categories:', error)
      fetchCategories()
    }
  }, [fetchCategories])

  const fetchSuggestions = useCallback(async (query: string): Promise<string[]> => {
    if (query.length === 0) {
      return []
    }

    try {
      const response = await fetch(`${API_BASE}/categories/suggestions?query=${encodeURIComponent(query)}`)
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
    fetchCategories()
  }, [fetchCategories])

  return {
    categories,
    loading,
    createCategory,
    updateCategory,
    deleteCategory,
    reorderCategories,
    fetchSuggestions
  }
}
