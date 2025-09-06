'use client'

import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useContract, useContractRead, useCategoryData, useCampaignData, useProductData } from '../../hooks/useContract'
import { DONLY_ADDRESS } from '../../lib/contract'
import { useState, useEffect } from 'react'
import Link from 'next/link'

export default function TestPage() {
  const [destinationWallet, setDestinationWallet] = useState('')
  const [maxProducts, setMaxProducts] = useState('')
  const [selectedCategoryId, setSelectedCategoryId] = useState('')
  const [selectedCampaignId, setSelectedCampaignId] = useState('')
  const [productPrice, setProductPrice] = useState('')
  const [selectedProductId, setSelectedProductId] = useState('')
  
  // Lists for displaying data
  const [categories, setCategories] = useState<any[]>([])
  const [campaigns, setCampaigns] = useState<any[]>([])
  const [products, setProducts] = useState<any[]>([])

  const {
    createCategory,
    createCampaign,
    addProduct,
    purchaseProduct,
    withdrawCampaignFunds
  } = useContract()

  const { categoryCount, campaignCount, productCount } = useContractRead()

  // Function to fetch categories list
  const fetchCategories = async () => {
    if (!categoryCount) return
    const count = Number(categoryCount)
    const categoryList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real category data from API
        const response = await fetch(`/api/category/${i}`)
        if (response.ok) {
          const categoryData = await response.json()
          categoryList.push({ 
            id: i, 
            name: `Category ${i}`, // Names are stored as hashes, so we show generic names
            status: categoryData.isActive ? 'Active' : 'Inactive',
            creator: categoryData.creator,
            nameHash: categoryData.nameHash
          })
        } else {
          // Fallback if API fails
          categoryList.push({ 
            id: i, 
            name: `Category ${i}`,
            status: 'Unknown',
            creator: 'N/A',
            nameHash: 'N/A'
          })
        }
      } catch (error) {
        console.error(`Error fetching category ${i}:`, error)
        categoryList.push({ 
          id: i, 
          name: `Category ${i}`,
          status: 'Error',
          creator: 'N/A',
          nameHash: 'N/A'
        })
      }
    }
    setCategories(categoryList)
  }

  // Function to fetch campaigns list
  const fetchCampaigns = async () => {
    if (!campaignCount) return
    const count = Number(campaignCount)
    const campaignList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real campaign data from API
        const response = await fetch(`/api/campaign/${i}`)
        if (response.ok) {
          const campaignData = await response.json()
          campaignList.push({ 
            id: i, 
            title: `Campaign ${i}`, // Titles are stored as hashes, so we show generic titles
            status: campaignData.isActive ? 'Active' : 'Inactive',
            categoryId: campaignData.categoryId,
            admin: campaignData.admin,
            destinationWallet: campaignData.destinationWallet,
            soldProductsCount: campaignData.soldProductsCount,
            maxSoldProducts: campaignData.maxSoldProducts,
            titleHash: campaignData.titleHash,
            descriptionHash: campaignData.descriptionHash
          })
        } else {
          // Fallback if API fails
          campaignList.push({ 
            id: i, 
            title: `Campaign ${i}`,
            status: 'Unknown',
            categoryId: 'N/A',
            admin: 'N/A',
            destinationWallet: 'N/A',
            soldProductsCount: 'N/A',
            maxSoldProducts: 'N/A',
            titleHash: 'N/A',
            descriptionHash: 'N/A'
          })
        }
      } catch (error) {
        console.error(`Error fetching campaign ${i}:`, error)
        campaignList.push({ 
          id: i, 
          title: `Campaign ${i}`,
          status: 'Error',
          categoryId: 'N/A',
          admin: 'N/A',
          destinationWallet: 'N/A',
          soldProductsCount: 'N/A',
          maxSoldProducts: 'N/A',
          titleHash: 'N/A',
          descriptionHash: 'N/A'
        })
      }
    }
    setCampaigns(campaignList)
  }

  // Function to fetch products list
  const fetchProducts = async () => {
    if (!productCount) return
    const count = Number(productCount)
    const productList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real product data from API
        const response = await fetch(`/api/product/${i}`)
        if (response.ok) {
          const productData = await response.json()
          productList.push({ 
            id: i, 
            name: `Product ${i}`, // Names are stored as hashes, so we show generic names
            status: productData.isActive ? (productData.isSold ? 'Sold' : 'Active') : 'Inactive',
            price: productData.priceInEth?.toFixed(6) || '0.00',
            priceInWei: productData.price || '0',
            campaignId: productData.campaignId,
            isActive: productData.isActive,
            isSold: productData.isSold
          })
        } else {
          // Fallback if API fails
          productList.push({ 
            id: i, 
            name: `Product ${i}`,
            status: 'Unknown',
            price: 'N/A',
            priceInWei: 'N/A',
            campaignId: 'N/A',
            isActive: false,
            isSold: false
          })
        }
      } catch (error) {
        console.error(`Error fetching product ${i}:`, error)
        productList.push({ 
          id: i, 
          name: `Product ${i}`,
          status: 'Error',
          price: 'N/A',
          priceInWei: 'N/A',
          campaignId: 'N/A',
          isActive: false,
          isSold: false
        })
      }
    }
    setProducts(productList)
  }

  // Auto-fetch lists when counts change
  useEffect(() => {
    fetchCategories()
    fetchCampaigns()
    fetchProducts()
  }, [categoryCount, campaignCount, productCount])

  const handleCreateCategory = async () => {
    try {
      await createCategory()
      alert('Category created successfully!')
    } catch (error) {
      console.error('Error creating category:', error)
      alert('Error creating category')
    }
  }

  const handleCreateCampaign = async () => {
    if (!destinationWallet || !maxProducts || !selectedCategoryId) return
    try {
      await createCampaign(
        BigInt(selectedCategoryId),
        destinationWallet as `0x${string}`,
        BigInt(maxProducts)
      )
      setDestinationWallet('')
      setMaxProducts('')
      setSelectedCategoryId('')
      alert('Campaign created successfully!')
    } catch (error) {
      console.error('Error creating campaign:', error)
      alert('Error creating campaign')
    }
  }

  const handleAddProduct = async () => {
    if (!productPrice || !selectedCampaignId || !selectedCategoryId) return
    try {
      await addProduct(
        BigInt(selectedCampaignId),
        BigInt(selectedCategoryId),
        BigInt(productPrice)
      )
      setProductPrice('')
      setSelectedCampaignId('')
      setSelectedCategoryId('')
      alert('Product added successfully!')
    } catch (error) {
      console.error('Error adding product:', error)
      alert('Error adding product')
    }
  }

  const handlePurchaseProduct = async () => {
    if (!selectedProductId) return
    try {
      // Find the product in our local list first
      const localProduct = products.find(p => p.id.toString() === selectedProductId)
      
      if (!localProduct) {
        alert('Product not found in local list')
        return
      }
      
      if (!localProduct.isActive) {
        alert('Product is not active')
        return
      }
      
      if (localProduct.isSold) {
        alert('Product is already sold')
        return
      }
      
      const priceInWei = BigInt(localProduct.priceInWei)
      const priceInEth = parseFloat(localProduct.price)
      
      // Show confirmation dialog
      const confirmed = confirm(
        `Purchase Product #${selectedProductId}?\n\n` +
        `Price: ${priceInEth.toFixed(6)} ETH (${priceInWei.toString()} wei)\n\n` +
        `This will send ${priceInEth.toFixed(6)} ETH to the campaign.\n` +
        `If this is the last product, funds will be automatically transferred to the destination wallet.`
      )
      
      if (!confirmed) return
      
      await purchaseProduct(BigInt(selectedProductId), priceInWei)
      alert('Product purchased successfully!')
      
      // Refresh the lists
      fetchProducts()
      fetchCampaigns()
      
      // Clear the selected product
      setSelectedProductId('')
    } catch (error) {
      console.error('Error purchasing product:', error)
      const errorMessage = error instanceof Error ? error.message : 'Unknown error'
      alert(`Error purchasing product: ${errorMessage}`)
    }
  }

  const handleWithdrawCampaignFunds = async () => {
    if (!selectedCampaignId) return
    try {
      await withdrawCampaignFunds(BigInt(selectedCampaignId))
      alert('Campaign funds withdrawn successfully!')
      setSelectedCampaignId('')
      // Refresh the campaigns list
      fetchCampaigns()
    } catch (error) {
      console.error('Error withdrawing campaign funds:', error)
      const errorMessage = error instanceof Error ? error.message : 'Unknown error'
      alert(`Error withdrawing campaign funds: ${errorMessage}`)
    }
  }


  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-6xl mx-auto px-4">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Panel Testowy</h1>
            <p className="text-gray-600">Testuj funkcje smart contractu na Arbitrum Sepolia</p>
          </div>
          <div className="flex items-center gap-4">
            <Link 
              href="/"
              className="bg-gray-100 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-200 transition-colors"
            >
              ← Strona główna
            </Link>
            <ConnectButton />
          </div>
        </div>

        {/* Contract Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Categories</h3>
            <p className="text-3xl font-bold text-blue-600">
              {categoryCount?.toString() || '0'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Campaigns</h3>
            <p className="text-3xl font-bold text-green-600">
              {campaignCount?.toString() || '0'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">Products</h3>
            <p className="text-3xl font-bold text-purple-600">
              {productCount?.toString() || '0'}
            </p>
          </div>
        </div>

        {/* Lists Section */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
          {/* Categories List */}
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold text-gray-900">Categories List</h3>
              <button
                onClick={fetchCategories}
                className="bg-blue-100 text-blue-700 px-3 py-1 rounded text-sm hover:bg-blue-200 transition-colors"
              >
                Refresh
              </button>
            </div>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {categories.length > 0 ? (
                categories.map((category) => (
                  <div 
                    key={category.id} 
                    className="p-3 bg-gray-50 rounded border cursor-pointer hover:bg-blue-50 hover:border-blue-300 transition-colors"
                    onClick={() => setSelectedCategoryId(category.id.toString())}
                  >
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-sm font-medium text-blue-600">ID: {category.id}</span>
                      <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded">{category.status}</span>
                    </div>
                    <div className="text-sm text-gray-600">{category.name}</div>
                    <div className="text-xs text-gray-500 mt-1">
                      Creator: {category.creator?.slice(0, 6)}...{category.creator?.slice(-4)}
                    </div>
                    <div className="text-xs text-gray-500">Click to use this ID in forms</div>
                  </div>
                ))
              ) : (
                <p className="text-sm text-gray-500">No categories found</p>
              )}
            </div>
          </div>

          {/* Campaigns List */}
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold text-gray-900">Campaigns List</h3>
              <button
                onClick={fetchCampaigns}
                className="bg-green-100 text-green-700 px-3 py-1 rounded text-sm hover:bg-green-200 transition-colors"
              >
                Refresh
              </button>
            </div>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {campaigns.length > 0 ? (
                campaigns.map((campaign) => (
                  <div 
                    key={campaign.id} 
                    className="p-3 bg-gray-50 rounded border cursor-pointer hover:bg-green-50 hover:border-green-300 transition-colors"
                    onClick={() => setSelectedCampaignId(campaign.id.toString())}
                  >
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-sm font-medium text-green-600">ID: {campaign.id}</span>
                      <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded">{campaign.status}</span>
                    </div>
                    <div className="text-sm text-gray-600">{campaign.title}</div>
                    <div className="text-xs text-gray-500 mt-1">
                      Category ID: {campaign.categoryId} | Max Products: {campaign.maxSoldProducts}
                    </div>
                    <div className="text-xs text-gray-500">
                      Sold: {campaign.soldProductsCount}/{campaign.maxSoldProducts}
                    </div>
                    <div className="text-xs text-gray-500">
                      Wallet: {campaign.destinationWallet?.slice(0, 6)}...{campaign.destinationWallet?.slice(-4)}
                    </div>
                  </div>
                ))
              ) : (
                <p className="text-sm text-gray-500">No campaigns found</p>
              )}
            </div>
          </div>

          {/* Products List */}
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold text-gray-900">Products List</h3>
              <button
                onClick={fetchProducts}
                className="bg-purple-100 text-purple-700 px-3 py-1 rounded text-sm hover:bg-purple-200 transition-colors"
              >
                Refresh
              </button>
            </div>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {products.length > 0 ? (
                products.map((product) => (
                  <div 
                    key={product.id} 
                    className="p-3 bg-gray-50 rounded border cursor-pointer hover:bg-purple-50 hover:border-purple-300 transition-colors"
                    onClick={() => setSelectedProductId(product.id.toString())}
                  >
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-sm font-medium text-purple-600">ID: {product.id}</span>
                      <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded">{product.status}</span>
                    </div>
                    <div className="text-sm text-gray-600">{product.name}</div>
                    <div className="text-xs text-gray-500 mt-1">
                      Price: {product.price} ETH | Campaign: {product.campaignId}
                    </div>
                    <div className="text-xs text-gray-500">
                      Price in Wei: {product.priceInWei}
                    </div>
                  </div>
                ))
              ) : (
                <p className="text-sm text-gray-500">No products found</p>
              )}
            </div>
          </div>
        </div>

        {/* Forms Section */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Create Category */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Create Category</h2>
          <div className="space-y-4">
            <button
              onClick={handleCreateCategory}
              className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors"
            >
              Create Category
            </button>
          </div>
        </div>

          {/* Create Campaign */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Create Campaign</h2>
            <div className="space-y-4">
              <input
                type="text"
                placeholder="Destination wallet (0x...)"
                value={destinationWallet}
                onChange={(e) => setDestinationWallet(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              />
              <input
                type="number"
                placeholder="Max products"
                value={maxProducts}
                onChange={(e) => setMaxProducts(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              />
              <input
                type="number"
                placeholder="Category ID"
                value={selectedCategoryId}
                onChange={(e) => setSelectedCategoryId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              />
              <button
                onClick={handleCreateCampaign}
                className="w-full bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 transition-colors"
              >
                Create Campaign
              </button>
            </div>
          </div>

          {/* Add Product */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Add Product</h2>
            <div className="space-y-4">
              <input
                type="number"
                placeholder="Price (wei)"
                value={productPrice}
                onChange={(e) => setProductPrice(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <input
                type="number"
                placeholder="Campaign ID"
                value={selectedCampaignId}
                onChange={(e) => setSelectedCampaignId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <input
                type="number"
                placeholder="Category ID"
                value={selectedCategoryId}
                onChange={(e) => setSelectedCategoryId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <button
                onClick={handleAddProduct}
                className="w-full bg-purple-600 text-white py-2 px-4 rounded-md hover:bg-purple-700 transition-colors"
              >
                Add Product
              </button>
            </div>
          </div>

          {/* Purchase Product */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Purchase Product</h2>
            <div className="space-y-4">
              <input
                type="number"
                placeholder="Product ID"
                value={selectedProductId}
                onChange={(e) => setSelectedProductId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500"
              />
              <button
                onClick={handlePurchaseProduct}
                className="w-full bg-orange-600 text-white py-2 px-4 rounded-md hover:bg-orange-700 transition-colors"
              >
                Purchase Product
              </button>
            </div>
          </div>
        </div>

        {/* Withdraw Campaign Funds */}
        <div className="mt-8">
          <div className="bg-white p-6 rounded-lg shadow max-w-md mx-auto">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Withdraw Campaign Funds</h2>
            <div className="space-y-4">
              <input
                type="number"
                placeholder="Campaign ID"
                value={selectedCampaignId}
                onChange={(e) => setSelectedCampaignId(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-red-500"
              />
              <button
                onClick={handleWithdrawCampaignFunds}
                className="w-full bg-red-600 text-white py-2 px-4 rounded-md hover:bg-red-700 transition-colors"
              >
                Withdraw Campaign Funds
              </button>
            </div>
            <p className="text-sm text-gray-500 mt-2">
              Manual withdrawal for campaign admin. Only the campaign admin can withdraw funds.
            </p>
          </div>
        </div>

        {/* Contract Address */}
        <div className="mt-8 text-center">
          <p className="text-sm text-gray-600">
            Contract Address: <span className="font-mono">{DONLY_ADDRESS}</span>
          </p>
          <p className="text-sm text-gray-600">
            Network: Arbitrum Sepolia
          </p>
        </div>
      </div>
    </div>
  )
}