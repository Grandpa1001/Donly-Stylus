'use client'

import Navigation from '@/components/Navigation'
import { ShoppingBag, Filter, Search } from 'lucide-react'
import { useContractRead, useContract } from '../../hooks/useContract'
import { useAccount } from 'wagmi'
import { useFirebaseAPIWithAutoFetch } from '../../hooks/useFirebaseAPI'
import { useState, useEffect } from 'react'

export default function MarketplacePage() {
  const [products, setProducts] = useState<any[]>([])
  const [filteredProducts, setFilteredProducts] = useState<any[]>([])
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedCampaign, setSelectedCampaign] = useState('all')
  const [sortBy, setSortBy] = useState('newest')
  const [isLoading, setIsLoading] = useState(true)

  const { productCount } = useContractRead()
  const { isConnected } = useAccount()
  const { purchaseProduct } = useContract()
  
  // Firebase hooks
  const { 
    getCategoryName,
    getCampaignName,
    getProductName,
    getProductDescription,
    getProductPriceInEth,
    getProductImageUrl,
    loading: firebaseLoading 
  } = useFirebaseAPIWithAutoFetch()

  // Function to fetch products from blockchain and Firebase
  const fetchProducts = async () => {
    if (!productCount) return
    
    setIsLoading(true)
    const count = Number(productCount)
    const productList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real product data from API
        const response = await fetch(`/api/product/${i}`)
        if (response.ok) {
          const productData = await response.json()
          
          // Only include active and unsold products for marketplace
          if (productData.isActive && !productData.isSold) {
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
              name: getProductName(i), // Get product name from Firebase
              description: getProductDescription(i), // Get product description from Firebase
              price: getProductPriceInEth(i).toFixed(6), // Get price from Firebase
              priceInWei: productData.price || '0',
              image: getProductImageUrl(i), // Get image URL from Firebase
              campaign: getCampaignName(Number(productData.campaignId)), // Get campaign name from Firebase
              categoryName: categoryName, // Get category name from Firebase
              rating: 4.5, // Default rating (can be enhanced later)
              reviews: Math.floor(Math.random() * 100) + 50, // Mock reviews (can be enhanced later)
              isActive: productData.isActive,
              isSold: productData.isSold,
              campaignId: productData.campaignId,
              categoryId: productData.categoryId
            })
          }
        } else {
          // Fallback if API fails - but don't add to marketplace since we can't verify if it's active/unsold
          console.log(`API failed for product ${i}, skipping from marketplace`)
        }
      } catch (error) {
        console.error(`Error fetching product ${i}:`, error)
      }
    }
    
    setProducts(productList)
    setIsLoading(false)
  }

  // Mock data removed - now using real data from Firebase and blockchain

  // Auto-fetch products when count changes or Firebase data changes
  useEffect(() => {
    fetchProducts()
  }, [productCount, firebaseLoading])

  // Filter and search products
  useEffect(() => {
    let filtered = products

    // Filter out inactive and sold products (only show active and available products)
    filtered = filtered.filter(product => product.isActive && !product.isSold)

    // Filter by search term
    if (searchTerm) {
      filtered = filtered.filter(product =>
        product.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        product.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
        product.campaign.toLowerCase().includes(searchTerm.toLowerCase())
      )
    }

    // Filter by campaign
    if (selectedCampaign !== 'all') {
      filtered = filtered.filter(product => product.campaign === selectedCampaign)
    }

    // Sort products
    switch (sortBy) {
      case 'price-low':
        filtered = filtered.sort((a, b) => parseFloat(a.price) - parseFloat(b.price))
        break
      case 'price-high':
        filtered = filtered.sort((a, b) => parseFloat(b.price) - parseFloat(a.price))
        break
      case 'newest':
        filtered = filtered.sort((a, b) => b.id - a.id)
        break
      case 'oldest':
        filtered = filtered.sort((a, b) => a.id - b.id)
        break
    }

    setFilteredProducts(filtered)
  }, [products, searchTerm, selectedCampaign, sortBy])

  const handlePurchaseProduct = async (productId: number, priceInWei: string) => {
    if (!isConnected) {
      alert('You must first connect your wallet to be able to purchase products.')
      return
    }

    try {
      const product = products.find(p => p.id === productId)
      if (!product) {
        alert('Product not found')
        return
      }

      const confirmed = confirm(
        `Purchase ${product.name}?\n\n` +
        `Price: ${product.price} ETH\n\n` +
        `This will send ${product.price} ETH to the campaign.\n` +
        `If this is the last product, funds will be automatically transferred to the destination wallet.`
      )
      
      if (!confirmed) return
      
      // Call the real purchase function from smart contract
      await purchaseProduct(BigInt(productId), BigInt(priceInWei))
      alert('Product purchased successfully!')
      
      // Refresh products
      fetchProducts()
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error'
      console.error('Error purchasing product:', error)
      alert(`Error purchasing product: ${errorMessage}`)
    }
  }

  const campaigns = [...new Set(products.map(p => p.campaign))]

  return (
    <div className="min-h-screen bg-gray-50">
      <Navigation />
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900">Marketplace</h1>
          <p className="text-gray-600">Sustainable fashion products supporting NGOs</p>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Total Products</h3>
            <p className="text-3xl font-bold text-blue-600">
              {productCount?.toString() || '0'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Available</h3>
            <p className="text-3xl font-bold text-green-600">
              {filteredProducts.length}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Sold</h3>
            <p className="text-3xl font-bold text-purple-600">
              {products.filter(p => p.isSold).length}
            </p>
          </div>
        </div>

        {/* Filters */}
        <div className="bg-white p-6 rounded-lg shadow mb-8">
          <div className="flex flex-col lg:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                <input
                  type="text"
                  placeholder="Search products..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
                />
              </div>
            </div>
            <div className="flex gap-4">
              <select
                value={selectedCampaign}
                onChange={(e) => setSelectedCampaign(e.target.value)}
                className="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              >
                <option value="all">All Campaigns</option>
                {campaigns.map(campaign => (
                  <option key={campaign} value={campaign}>{campaign}</option>
                ))}
              </select>
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value)}
                className="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              >
                <option value="newest">Newest</option>
                <option value="oldest">Oldest</option>
                <option value="price-low">Price: Low to High</option>
                <option value="price-high">Price: High to Low</option>
              </select>
            </div>
          </div>
        </div>

        {/* Products Grid */}
        {isLoading || firebaseLoading ? (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
            <p className="mt-4 text-gray-600">
              {firebaseLoading ? 'Loading Firebase data...' : 'Loading products...'}
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {filteredProducts.map((product) => (
              <div key={product.id} className="bg-white rounded-lg shadow hover:shadow-lg transition-shadow">
                  <div className="relative">
                    {product.image ? (
                      <img
                        src={product.image}
                        alt={product.name}
                        className="w-full h-40 object-cover rounded-t-lg"
                        onError={(e) => {
                          const target = e.target as HTMLImageElement;
                          target.src = 'https://images.unsplash.com/photo-1441986300917-64674bd600d8?w=400&h=400&fit=crop';
                        }}
                      />
                    ) : (
                      <div className="w-full h-40 bg-gray-200 rounded-t-lg flex items-center justify-center">
                        <span className="text-gray-400">No image</span>
                      </div>
                    )}
                    <div className="absolute top-2 left-2 bg-green-500 text-white px-2 py-1 rounded text-xs font-medium">
                      {product.campaign}
                    </div>
                  </div>
                
                <div className="p-3">
                  <h3 className="text-base font-semibold text-gray-900 mb-1 line-clamp-2">
                    {product.name}
                  </h3>
                  <p className="text-gray-600 text-xs mb-2 line-clamp-2">
                    {product.description}
                  </p>
                  
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-lg font-bold text-green-600">
                      {parseFloat(product.price) < 0.001 ? '<0.001' : parseFloat(product.price).toFixed(3)} ETH
                    </span>
                    <span className="text-xs text-gray-500">
                      {product.categoryName}
                    </span>
                  </div>
                  
                  <button
                    onClick={() => handlePurchaseProduct(product.id, product.priceInWei)}
                    className={`w-full py-2 px-4 rounded-md font-medium transition-colors ${
                      isConnected 
                        ? 'bg-green-600 text-white hover:bg-green-700' 
                        : 'bg-gray-400 text-gray-200 cursor-not-allowed'
                    }`}
                    disabled={!isConnected}
                  >
                    {isConnected ? 'Buy Now' : 'Connect Wallet'}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}

        {!isLoading && filteredProducts.length === 0 && (
          <div className="text-center py-12">
            <ShoppingBag className="w-16 h-16 text-gray-400 mx-auto mb-4" />
            <h3 className="text-lg font-semibold text-gray-900 mb-2">No products found</h3>
            <p className="text-gray-600">Try adjusting your search or filters</p>
          </div>
        )}
      </div>
    </div>
  )
}
