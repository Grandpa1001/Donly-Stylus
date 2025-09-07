import { useState, useEffect } from 'react'
import { collection, addDoc, getDocs, query, orderBy } from 'firebase/firestore'
import { db } from '../lib/firebase'

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
  priceInEth: number
  imageUrl: string
  blockchainId: number
  campaignId: number
  categoryId: number
  createdAt: Date
  updatedAt: Date
}

// Hook for Firebase Client SDK operations
export function useFirebaseAPI() {
  const [categories, setCategories] = useState<Category[]>([])
  const [campaigns, setCampaigns] = useState<Campaign[]>([])
  const [products, setProducts] = useState<Product[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch all categories from Firebase
  const fetchCategories = async () => {
    setLoading(true)
    setError(null)
    try {
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
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to fetch categories')
    console.error('Error fetching categories:', err)
  } finally {
    setLoading(false)
  }
}

// Fetch all campaigns from Firebase
const fetchCampaigns = async () => {
  setLoading(true)
  setError(null)
  try {
    const campaignsRef = collection(db, 'campaigns')
    const q = query(campaignsRef, orderBy('createdAt', 'desc'))
    const querySnapshot = await getDocs(q)
    
    const campaignsData: Campaign[] = []
    querySnapshot.forEach((doc) => {
      const data = doc.data()
      campaignsData.push({
        id: doc.id,
        name: data.name,
        description: data.description,
        blockchainId: data.blockchainId,
        createdAt: data.createdAt?.toDate() || new Date(),
        updatedAt: data.updatedAt?.toDate() || new Date(),
      })
    })
    
    setCampaigns(campaignsData)
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to fetch campaigns')
    console.error('Error fetching campaigns:', err)
  } finally {
    setLoading(false)
  }
}

// Fetch all products from Firebase
const fetchProducts = async () => {
  setLoading(true)
  setError(null)
  try {
    const productsRef = collection(db, 'products')
    const q = query(productsRef, orderBy('createdAt', 'desc'))
    const querySnapshot = await getDocs(q)
    
    const productsData: Product[] = []
    querySnapshot.forEach((doc) => {
      const data = doc.data()
      productsData.push({
        id: doc.id,
        name: data.name,
        description: data.description,
        priceInEth: data.priceInEth,
        imageUrl: data.imageUrl,
        blockchainId: data.blockchainId,
        campaignId: data.campaignId,
        categoryId: data.categoryId,
        createdAt: data.createdAt?.toDate() || new Date(),
        updatedAt: data.updatedAt?.toDate() || new Date(),
      })
    })
    
    setProducts(productsData)
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to fetch products')
    console.error('Error fetching products:', err)
  } finally {
    setLoading(false)
  }
}

  // Add new category to Firebase
  const addCategory = async (name: string, blockchainId: number) => {
    try {
      const categoryData = {
        name,
        blockchainId,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
      
      const docRef = await addDoc(collection(db, 'categories'), categoryData)
      
      // Update local state
      const newCategory: Category = {
        id: docRef.id,
        name,
        blockchainId,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
      
      setCategories(prev => [newCategory, ...prev])
      
    return docRef.id
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to add category')
    console.error('Error adding category:', err)
    throw err
  }
}

// Add new campaign to Firebase
const addCampaign = async (name: string, description: string, blockchainId: number) => {
  try {
    const campaignData = {
      name,
      description,
      blockchainId,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    
    const docRef = await addDoc(collection(db, 'campaigns'), campaignData)
    
    // Update local state
    const newCampaign: Campaign = {
      id: docRef.id,
      name,
      description,
      blockchainId,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    
    setCampaigns(prev => [newCampaign, ...prev])
    
    return docRef.id
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to add campaign')
    console.error('Error adding campaign:', err)
    throw err
  }
}

// Add new product to Firebase
const addProduct = async (
  name: string, 
  description: string, 
  priceInEth: number, 
  imageUrl: string, 
  blockchainId: number, 
  campaignId: number, 
  categoryId: number
) => {
  try {
    const productData = {
      name,
      description,
      priceInEth,
      imageUrl,
      blockchainId,
      campaignId,
      categoryId,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    
    const docRef = await addDoc(collection(db, 'products'), productData)
    
    // Update local state
    const newProduct: Product = {
      id: docRef.id,
      name,
      description,
      priceInEth,
      imageUrl,
      blockchainId,
      campaignId,
      categoryId,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    
    setProducts(prev => [newProduct, ...prev])
    
    return docRef.id
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to add product')
    console.error('Error adding product:', err)
    throw err
  }
}


  // Get category name by blockchain ID
  const getCategoryName = (blockchainId: number): string => {
    const category = categories.find(cat => Number(cat.blockchainId) === blockchainId)
    return category ? category.name : `Category ${blockchainId}`
  }

  // Get category by blockchain ID
  const getCategoryByBlockchainId = (blockchainId: number): Category | undefined => {
    return categories.find(cat => Number(cat.blockchainId) === blockchainId)
  }

  // Get campaign name by blockchain ID
  const getCampaignName = (blockchainId: number): string => {
    const campaign = campaigns.find(camp => Number(camp.blockchainId) === blockchainId)
    return campaign ? campaign.name : `Campaign ${blockchainId}`
  }

  // Get campaign description by blockchain ID
  const getCampaignDescription = (blockchainId: number): string => {
    const campaign = campaigns.find(camp => Number(camp.blockchainId) === blockchainId)
    return campaign ? campaign.description : `Description for Campaign ${blockchainId}`
  }

  // Get campaign by blockchain ID
  const getCampaignByBlockchainId = (blockchainId: number): Campaign | undefined => {
    return campaigns.find(camp => Number(camp.blockchainId) === blockchainId)
  }

  // Get product name by blockchain ID
  const getProductName = (blockchainId: number): string => {
    const product = products.find(prod => Number(prod.blockchainId) === blockchainId)
    return product ? product.name : `Product ${blockchainId}`
  }

  // Get product description by blockchain ID
  const getProductDescription = (blockchainId: number): string => {
    const product = products.find(prod => Number(prod.blockchainId) === blockchainId)
    return product ? product.description : `Description for Product ${blockchainId}`
  }

  // Get product price in ETH by blockchain ID
  const getProductPriceInEth = (blockchainId: number): number => {
    const product = products.find(prod => Number(prod.blockchainId) === blockchainId)
    return product ? product.priceInEth : 0
  }

  // Get product image URL by blockchain ID
  const getProductImageUrl = (blockchainId: number): string => {
    const product = products.find(prod => Number(prod.blockchainId) === blockchainId)
    return product ? product.imageUrl : ''
  }

  // Get product by blockchain ID
  const getProductByBlockchainId = (blockchainId: number): Product | undefined => {
    return products.find(prod => Number(prod.blockchainId) === blockchainId)
  }

  return {
    categories,
    campaigns,
    products,
    loading,
    error,
    fetchCategories,
    fetchCampaigns,
    fetchProducts,
    addCategory,
    addCampaign,
    addProduct,
    getCategoryName,
    getCategoryByBlockchainId,
    getCampaignName,
    getCampaignDescription,
    getCampaignByBlockchainId,
    getProductName,
    getProductDescription,
    getProductPriceInEth,
    getProductImageUrl,
    getProductByBlockchainId,
  }
}

// Auto-fetch categories, campaigns and products on mount
export function useFirebaseAPIWithAutoFetch() {
  const firebaseAPIHook = useFirebaseAPI()
  
  useEffect(() => {
    firebaseAPIHook.fetchCategories()
    firebaseAPIHook.fetchCampaigns()
    firebaseAPIHook.fetchProducts()
  }, [])
  
  return firebaseAPIHook
}
