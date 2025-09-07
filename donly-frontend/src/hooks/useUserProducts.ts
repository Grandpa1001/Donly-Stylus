import { useState, useEffect } from 'react'
import { useAccount } from 'wagmi'
import { useContractRead } from './useContract'
import { useFirebaseAPIWithAutoFetch } from './useFirebaseAPI'

export interface UserProduct {
  id: number
  name: string
  description: string
  price: string
  priceInWei: string
  image: string
  campaign: string
  categoryName: string
  isActive: boolean
  isSold: boolean
  campaignId: string
  categoryId: number
  isOwned: boolean
  isPurchased: boolean
}

export function useUserProducts() {
  const { address } = useAccount()
  const { productCount } = useContractRead()
  const { 
    getCategoryName,
    getCampaignName,
    getProductName,
    getProductDescription,
    getProductPriceInEth,
    getProductImageUrl,
    loading: firebaseLoading 
  } = useFirebaseAPIWithAutoFetch()
  
  const [products, setProducts] = useState<UserProduct[]>([])
  const [isLoading, setIsLoading] = useState(true)

  // Function to fetch user's products
  const fetchUserProducts = async () => {
    if (!productCount || !address) {
      setIsLoading(false)
      return
    }
    
    setIsLoading(true)
    const count = Number(productCount)
    const productList: UserProduct[] = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real product data from API
        const response = await fetch(`/api/product/${i}`)
        if (response.ok) {
          const productData = await response.json()
          
          // Get categoryId from campaign since product API doesn't return it
          const campaignResponse = await fetch(`/api/campaign/${productData.campaignId}`)
          let categoryId = 1 // fallback
          if (campaignResponse.ok) {
            const campaignData = await campaignResponse.json()
            categoryId = Number(campaignData.categoryId)
          }
          
          const categoryName = getCategoryName(categoryId)
          
          productList.push({ 
            id: i, 
            name: getProductName(i),
            description: getProductDescription(i),
            price: getProductPriceInEth(i).toFixed(6),
            priceInWei: productData.price || '0',
            image: getProductImageUrl(i),
            campaign: getCampaignName(Number(productData.campaignId)),
            categoryName: categoryName,
            isActive: productData.isActive,
            isSold: productData.isSold,
            campaignId: productData.campaignId,
            categoryId: categoryId,
            // Check real ownership from blockchain
            isOwned: productData.owner?.toLowerCase() === address?.toLowerCase(),
            isPurchased: productData.isSold && productData.owner?.toLowerCase() !== address?.toLowerCase()
          })
        }
      } catch (error) {
        console.error(`Error fetching product ${i}:`, error)
      }
    }
    
    setProducts(productList)
    setIsLoading(false)
  }

  // Auto-fetch products when count changes or Firebase data changes
  useEffect(() => {
    fetchUserProducts()
  }, [productCount, firebaseLoading, address])

  // Filter products based on type
  const getFilteredProducts = (type: 'active' | 'purchased' | 'inactive') => {
    switch (type) {
      case 'active':
        // Products owned by user that are active and not sold
        return products.filter(p => p.isOwned && p.isActive && !p.isSold)
      case 'purchased':
        // Products purchased by user
        return products.filter(p => p.isPurchased)
      case 'inactive':
        // Products owned by user that are inactive or sold to someone else
        return products.filter(p => p.isOwned && (!p.isActive || p.isSold))
      default:
        return []
    }
  }

  return {
    products,
    isLoading: isLoading || firebaseLoading,
    getFilteredProducts,
    refetch: fetchUserProducts
  }
}
