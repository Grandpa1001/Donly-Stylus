import { useState, useEffect } from 'react'
import { 
  collection, 
  addDoc, 
  getDocs, 
  doc, 
  updateDoc, 
  deleteDoc,
  query,
  orderBy 
} from 'firebase/firestore'
import { db } from '../lib/firebase'
import { mockFirebase, shouldUseMockFirebase, initializeMockData } from '../lib/firebaseMock'

// Types
export interface Category {
  id: string
  name: string
  blockchainId: number
  createdAt: Date
  updatedAt: Date
}

export interface Campaign {
  id: string
  name: string
  description: string
  blockchainId: number
  createdAt: Date
  updatedAt: Date
}

export interface Product {
  id: string
  name: string
  description: string
  blockchainId: number
  campaignId: number
  categoryId: number
  createdAt: Date
  updatedAt: Date
}

// Categories Hook
export function useCategories() {
  const [categories, setCategories] = useState<Category[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch all categories
  const fetchCategories = async () => {
    setLoading(true)
    setError(null)
    try {
      if (shouldUseMockFirebase()) {
        // Use mock Firebase
        const categoriesData = await mockFirebase.categories.get()
        setCategories(categoriesData)
      } else {
        // Use real Firebase
        const categoriesRef = collection(db, 'categories')
        const q = query(categoriesRef, orderBy('createdAt', 'desc'))
        const querySnapshot = await getDocs(q)
        
        const categoriesData: Category[] = []
        querySnapshot.forEach((doc) => {
          const data = doc.data()
          categoriesData.push({
            id: doc.id,
            name: data.name,
            blockchainId: data.blockchainId,
            createdAt: data.createdAt?.toDate() || new Date(),
            updatedAt: data.updatedAt?.toDate() || new Date(),
          })
        })
        
        setCategories(categoriesData)
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch categories')
      console.error('Error fetching categories:', err)
    } finally {
      setLoading(false)
    }
  }

  // Add new category
  const addCategory = async (name: string, blockchainId: number) => {
    try {
      const categoryData = {
        name,
        blockchainId,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
      
      let docId: string
      
      if (shouldUseMockFirebase()) {
        // Use mock Firebase
        const result = await mockFirebase.categories.add(categoryData)
        docId = result.id
      } else {
        // Use real Firebase
        const docRef = await addDoc(collection(db, 'categories'), categoryData)
        docId = docRef.id
      }
      
      // Update local state
      const newCategory: Category = {
        id: docId,
        name,
        blockchainId,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
      
      setCategories(prev => [newCategory, ...prev])
      
      return docId
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add category')
      console.error('Error adding category:', err)
      throw err
    }
  }

  // Update category
  const updateCategory = async (id: string, name: string) => {
    try {
      const categoryRef = doc(db, 'categories', id)
      await updateDoc(categoryRef, {
        name,
        updatedAt: new Date(),
      })
      
      // Update local state
      setCategories(prev => 
        prev.map(cat => 
          cat.id === id 
            ? { ...cat, name, updatedAt: new Date() }
            : cat
        )
      )
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update category')
      console.error('Error updating category:', err)
      throw err
    }
  }

  // Delete category
  const deleteCategory = async (id: string) => {
    try {
      await deleteDoc(doc(db, 'categories', id))
      
      // Update local state
      setCategories(prev => prev.filter(cat => cat.id !== id))
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete category')
      console.error('Error deleting category:', err)
      throw err
    }
  }

  // Get category by blockchain ID
  const getCategoryByBlockchainId = (blockchainId: number): Category | undefined => {
    return categories.find(cat => cat.blockchainId === blockchainId)
  }

  // Get category name by blockchain ID
  const getCategoryName = (blockchainId: number): string => {
    const category = getCategoryByBlockchainId(blockchainId)
    return category ? category.name : `Category ${blockchainId}`
  }

  return {
    categories,
    loading,
    error,
    fetchCategories,
    addCategory,
    updateCategory,
    deleteCategory,
    getCategoryByBlockchainId,
    getCategoryName,
  }
}

// Auto-fetch categories on mount
export function useCategoriesWithAutoFetch() {
  const categoriesHook = useCategories()
  
  useEffect(() => {
    // Initialize mock data if using mock Firebase
    if (shouldUseMockFirebase()) {
      initializeMockData()
    }
    
    categoriesHook.fetchCategories()
  }, [])
  
  return categoriesHook
}
