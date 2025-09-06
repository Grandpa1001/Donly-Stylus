'use client'

import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useContract, useContractRead, useCategoryData, useCampaignData, useProductData } from '../../hooks/useContract'
import { DONLY_ADDRESS } from '../../lib/contract'
import { useState, useEffect } from 'react'
import Link from 'next/link'

export default function TestPage() {
  const [categoryName, setCategoryName] = useState('')
  const [campaignTitle, setCampaignTitle] = useState('')
  const [campaignDescription, setCampaignDescription] = useState('')
  const [destinationWallet, setDestinationWallet] = useState('')
  const [maxProducts, setMaxProducts] = useState('')
  const [selectedCategoryId, setSelectedCategoryId] = useState('')
  const [selectedCampaignId, setSelectedCampaignId] = useState('')
  const [productName, setProductName] = useState('')
  const [productDescription, setProductDescription] = useState('')
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
    purchaseProduct
  } = useContract()

  const { categoryCount, campaignCount, productCount } = useContractRead()

  // Function to fetch categories list
  const fetchCategories = async () => {
    if (!categoryCount) return
    const count = Number(categoryCount)
    const categoryList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // We'll show ID and basic info since names are stored as hashes
        categoryList.push({ 
          id: i, 
          name: `Category ${i}`,
          status: 'Active' // We could fetch this from contract if needed
        })
      } catch (error) {
        console.error(`Error fetching category ${i}:`, error)
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
        campaignList.push({ 
          id: i, 
          title: `Campaign ${i}`,
          status: 'Active',
          categoryId: 'N/A' // We could fetch this from contract
        })
      } catch (error) {
        console.error(`Error fetching campaign ${i}:`, error)
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
        productList.push({ 
          id: i, 
          name: `Product ${i}`,
          status: 'Active',
          price: 'N/A',
          campaignId: 'N/A'
        })
      } catch (error) {
        console.error(`Error fetching product ${i}:`, error)
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
    if (!categoryName) return
    try {
      await createCategory(categoryName)
      setCategoryName('')
      alert('Category created successfully!')
    } catch (error) {
      console.error('Error creating category:', error)
      alert('Error creating category')
    }
  }

  const handleCreateCampaign = async () => {
    if (!campaignTitle || !campaignDescription || !destinationWallet || !maxProducts || !selectedCategoryId) return
    try {
      await createCampaign(
        BigInt(selectedCategoryId),
        campaignTitle,
        campaignDescription,
        destinationWallet as `0x${string}`,
        BigInt(maxProducts)
      )
      setCampaignTitle('')
      setCampaignDescription('')
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
    if (!productName || !productDescription || !productPrice || !selectedCampaignId || !selectedCategoryId) return
    try {
      await addProduct(
        BigInt(selectedCampaignId),
        BigInt(selectedCategoryId),
        productName,
        productDescription,
        BigInt(productPrice)
      )
      setProductName('')
      setProductDescription('')
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
      await purchaseProduct(BigInt(selectedProductId))
      alert('Product purchased successfully!')
    } catch (error) {
      console.error('Error purchasing product:', error)
      alert('Error purchasing product')
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
                    <div className="text-xs text-gray-500 mt-1">Click to use this ID in forms</div>
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
                    <div className="text-xs text-gray-500 mt-1">Category ID: {campaign.categoryId}</div>
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
                    <div className="text-xs text-gray-500 mt-1">Price: {product.price} | Campaign: {product.campaignId}</div>
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
              <input
                type="text"
                placeholder="Category name"
                value={categoryName}
                onChange={(e) => setCategoryName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
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
                placeholder="Campaign title"
                value={campaignTitle}
                onChange={(e) => setCampaignTitle(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
              />
              <textarea
                placeholder="Campaign description"
                value={campaignDescription}
                onChange={(e) => setCampaignDescription(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
                rows={3}
              />
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
                type="text"
                placeholder="Product name"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <textarea
                placeholder="Product description"
                value={productDescription}
                onChange={(e) => setProductDescription(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
                rows={3}
              />
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
