'use client'

import { useAccount } from 'wagmi'
import { ConnectButton } from '@rainbow-me/rainbowkit'
import Navigation from '@/components/Navigation'
import { Wallet } from 'lucide-react'
import { useState, useEffect } from 'react'
import { useContract, useContractRead } from '../../hooks/useContract'
import { useFirebaseAPIWithAutoFetch } from '../../hooks/useFirebaseAPI'

// Import our new components
import ProfileHeader from '@/components/profile/ProfileHeader'
import ProductTabs from '@/components/profile/ProductTabs'
import ProductGrid from '@/components/profile/ProductGrid'
import { useUserProducts } from '@/hooks/useUserProducts'

export default function ProfilePage() {
  const { isConnected, address } = useAccount()
  const [activeTab, setActiveTab] = useState<'active' | 'purchased' | 'inactive'>('active')
  const { isLoading, getFilteredProducts } = useUserProducts()

  // Contract hooks
  const { addProduct } = useContract()
  const { campaignCount, productCount } = useContractRead()
  
  // Firebase hooks
  const {
    campaigns: firebaseCampaigns,
    addProduct: addProductToFirebase,
    getCampaignByBlockchainId,
    getCampaignName
  } = useFirebaseAPIWithAutoFetch()

  // Product form state
  const [productName, setProductName] = useState('')
  const [productDescription, setProductDescription] = useState('')
  const [productPrice, setProductPrice] = useState('')
  const [productImageUrl, setProductImageUrl] = useState('')
  const [selectedCampaignId, setSelectedCampaignId] = useState('')
  const [showAddProductForm, setShowAddProductForm] = useState(false)
  const [activeCampaigns, setActiveCampaigns] = useState<any[]>([])

  const handleAddProduct = () => {
    setShowAddProductForm(true)
  }

  // Function to fetch active campaigns
  const fetchActiveCampaigns = async () => {
    if (!campaignCount) return
    const count = Number(campaignCount)
    const campaignList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real campaign data from API
        const response = await fetch(`/api/campaign/${i}`)
        if (response.ok) {
          const campaignData = await response.json()
          
          // Only include active campaigns
          if (campaignData.isActive) {
            campaignList.push({ 
              id: i, 
              name: getCampaignName(i), // Get campaign name from Firebase
              blockchainId: i,
              categoryId: campaignData.categoryId,
              admin: campaignData.admin,
              destinationWallet: campaignData.destinationWallet,
              maxSoldProducts: campaignData.maxSoldProducts,
              soldProductsCount: campaignData.soldProductsCount
            })
          }
        }
      } catch (error) {
        console.error(`Error fetching campaign ${i}:`, error)
      }
    }
    
    setActiveCampaigns(campaignList)
  }

  // Fetch active campaigns when campaign count changes
  useEffect(() => {
    fetchActiveCampaigns()
  }, [campaignCount])

  const handleSubmitProduct = async () => {
    if (!productName || !productDescription || !productPrice || !productImageUrl || !selectedCampaignId) {
      alert('Please fill in all fields')
      return
    }
    
    try {
      // Convert ETH to Wei for blockchain
      const priceInWei = BigInt(Math.floor(parseFloat(productPrice) * 1e18))
      
      // Get campaign data to get category ID
      const campaignData = await fetch(`/api/campaign/${selectedCampaignId}`)
      if (!campaignData.ok) {
        alert('Error fetching campaign data')
        return
      }
      const campaign = await campaignData.json()
      
      // Add product to blockchain
      await addProduct(
        BigInt(selectedCampaignId),
        BigInt(campaign.categoryId),
        priceInWei
      )
      
      // Get the new product ID (current count + 1)
      const newProductId = Number(productCount) + 1
      
      // Add product to Firebase
      await addProductToFirebase(
        productName.trim(),
        productDescription.trim(),
        parseFloat(productPrice),
        productImageUrl.trim(),
        newProductId,
        Number(selectedCampaignId),
        Number(campaign.categoryId)
      )
      
      // Reset form
      setProductName('')
      setProductDescription('')
      setProductPrice('')
      setProductImageUrl('')
      setSelectedCampaignId('')
      setShowAddProductForm(false)
      
      alert('Product added successfully!')
    } catch (error) {
      console.error('Error adding product:', error)
      alert('Error adding product')
    }
  }

  if (!isConnected) {
    return (
      <div className="min-h-screen bg-gray-50">
        <Navigation />
        
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
          <div className="text-center">
            <div className="w-24 h-24 bg-gradient-to-br from-gray-400 to-gray-500 rounded-full flex items-center justify-center mx-auto mb-8">
              <Wallet className="w-12 h-12 text-white" />
            </div>
            
            <h1 className="text-6xl font-bold text-gray-900 mb-6">
              Connect Wallet
            </h1>
            
            <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
              To access your profile, you must first connect your wallet.
            </p>
            
            <div className="bg-white rounded-lg shadow-sm p-8 max-w-md mx-auto">
              <ConnectButton />
            </div>
          </div>
        </div>
      </div>
    )
  }

  const filteredProducts = getFilteredProducts(activeTab)

  return (
    <div className="min-h-screen bg-gray-50">
      <Navigation />
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Profile Header */}
        <ProfileHeader address={address!} />

        {/* Add Product Form */}
        {showAddProductForm && (
          <div className="bg-white rounded-lg shadow-sm p-6 mb-8">
            <div className="flex justify-between items-center mb-6">
                    <h2 className="text-xl font-semibold text-gray-900">Add New Product</h2>
              <button
                onClick={() => setShowAddProductForm(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Product Name
                </label>
                <input
                  type="text"
                  value={productName}
                  onChange={(e) => setProductName(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter product name"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Price (ETH)
                </label>
                <input
                  type="number"
                  step="0.001"
                  value={productPrice}
                  onChange={(e) => setProductPrice(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="0.001"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Campaign
                </label>
                <select
                  value={selectedCampaignId}
                  onChange={(e) => setSelectedCampaignId(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">Select Campaign</option>
                  {activeCampaigns.map((campaign) => (
                    <option key={campaign.blockchainId} value={campaign.blockchainId}>
                      {campaign.name} (ID: {campaign.blockchainId})
                    </option>
                  ))}
                </select>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Image URL
                </label>
                <input
                  type="url"
                  value={productImageUrl}
                  onChange={(e) => setProductImageUrl(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="https://example.com/image.jpg"
                />
              </div>
              
              <div className="md:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Product Description
                </label>
                <textarea
                  value={productDescription}
                  onChange={(e) => setProductDescription(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Describe your product..."
                />
              </div>
            </div>
            
            <div className="flex gap-4 mt-6">
              <button
                onClick={handleSubmitProduct}
                className="bg-green-600 text-white px-6 py-2 rounded-md hover:bg-green-700 transition-colors"
              >
                Add Product
              </button>
              <button
                onClick={() => setShowAddProductForm(false)}
                className="bg-gray-500 text-white px-6 py-2 rounded-md hover:bg-gray-600 transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        )}

        {/* Product Tabs */}
        <div className="bg-white rounded-lg shadow-sm">
          {/* Tab Navigation */}
          <ProductTabs activeTab={activeTab} setActiveTab={setActiveTab} />

          {/* Tab Content */}
          <div className="p-6">
            {isLoading ? (
              <div className="text-center py-12">
                <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
                <p className="mt-4 text-gray-600">Loading products...</p>
              </div>
            ) : (
              <ProductGrid 
                products={filteredProducts} 
                type={activeTab} 
                onAddProduct={handleAddProduct}
              />
            )}
          </div>
        </div>
      </div>
    </div>
  )
}