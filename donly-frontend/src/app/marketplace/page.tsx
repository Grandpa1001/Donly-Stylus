'use client'

import Navigation from '@/components/Navigation'
import { ShoppingBag, Filter, Search, Heart, Star, Eye } from 'lucide-react'
import { useContractRead } from '../../hooks/useContract'
import { useState, useEffect } from 'react'
import { ConnectButton } from '@rainbow-me/rainbowkit'

export default function MarketplacePage() {
  const [products, setProducts] = useState<any[]>([])
  const [filteredProducts, setFilteredProducts] = useState<any[]>([])
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedCampaign, setSelectedCampaign] = useState('all')
  const [sortBy, setSortBy] = useState('newest')
  const [isLoading, setIsLoading] = useState(true)

  const { productCount } = useContractRead()

  // Mock data for products (will be replaced with real data later)
  const mockProductData = [
    {
      id: 1,
      name: "Eco-Friendly Cotton T-Shirt",
      description: "Sustainable cotton t-shirt made from organic materials",
      price: "0.05",
      priceInWei: "50000000000000000",
      campaignId: 1,
      categoryId: 1,
      image: "https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=400&h=400&fit=crop",
      isActive: true,
      isSold: false,
      campaign: "Environmental Care",
      category: "Clothing"
    },
    {
      id: 2,
      name: "Recycled Denim Jacket",
      description: "Vintage-style jacket made from recycled denim",
      price: "0.08",
      priceInWei: "80000000000000000",
      campaignId: 1,
      categoryId: 1,
      image: "https://images.unsplash.com/photo-1551028719-00167b16eac5?w=400&h=400&fit=crop",
      isActive: true,
      isSold: false,
      campaign: "Environmental Care",
      category: "Clothing"
    },
    {
      id: 3,
      name: "Handmade Wool Scarf",
      description: "Artisan-crafted wool scarf supporting local communities",
      price: "0.03",
      priceInWei: "30000000000000000",
      campaignId: 2,
      categoryId: 1,
      image: "https://images.unsplash.com/photo-1601924994987-69e26d50dc26?w=400&h=400&fit=crop",
      isActive: true,
      isSold: false,
      campaign: "Sustainable Development",
      category: "Accessories"
    },
    {
      id: 4,
      name: "Organic Cotton Bag",
      description: "Eco-friendly shopping bag made from organic cotton",
      price: "0.02",
      priceInWei: "20000000000000000",
      campaignId: 2,
      categoryId: 2,
      image: "https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=400&h=400&fit=crop",
      isActive: true,
      isSold: true,
      campaign: "Sustainable Development",
      category: "Accessories"
    },
    {
      id: 5,
      name: "Bamboo Sunglasses",
      description: "Sustainable sunglasses made from bamboo frames",
      price: "0.12",
      priceInWei: "120000000000000000",
      campaignId: 3,
      categoryId: 2,
      image: "https://images.unsplash.com/photo-1511499767150-a48a237f0083?w=400&h=400&fit=crop",
      isActive: true,
      isSold: false,
      campaign: "Environmental Care",
      category: "Accessories"
    },
    {
      id: 6,
      name: "Fair Trade Coffee Mug",
      description: "Ceramic mug supporting fair trade coffee farmers",
      price: "0.04",
      priceInWei: "40000000000000000",
      campaignId: 3,
      categoryId: 3,
      image: "https://images.unsplash.com/photo-1514228742587-6b1558fcf93a?w=400&h=400&fit=crop",
      isActive: true,
      isSold: false,
      campaign: "Food Security",
      category: "Home & Living"
    }
  ]

  // Fetch products from blockchain
  const fetchProducts = async () => {
    if (!productCount) {
      setIsLoading(false)
      return
    }
    
    const count = Number(productCount)
    const productList = []
    
    for (let i = 1; i <= count; i++) {
      try {
        // Fetch real product data from API
        const response = await fetch(`/api/product/${i}`)
        if (response.ok) {
          const productData = await response.json()
          
          // Get mock data for this product
          const mockData = mockProductData.find(mock => mock.id === i) || {
            name: `Product ${i}`,
            description: `Description for product ${i}`,
            image: "https://images.unsplash.com/photo-1441986300917-64674bd600d8?w=400&h=400&fit=crop",
            campaign: "Campaign",
            category: "Category",
            categoryId: 1
          }
          
          productList.push({
            id: i,
            name: mockData.name,
            description: mockData.description,
            price: productData.priceInEth?.toFixed(6) || "0.00",
            priceInWei: productData.price || "0",
            campaignId: productData.campaignId,
            categoryId: mockData.categoryId,
            image: mockData.image,
            isActive: productData.isActive,
            isSold: productData.isSold,
            campaign: mockData.campaign,
            category: mockData.category
          })
        }
      } catch (error) {
        console.error(`Error fetching product ${i}:`, error)
      }
    }
    
    setProducts(productList)
    setFilteredProducts(productList)
    setIsLoading(false)
  }

  // Auto-fetch products when count changes
  useEffect(() => {
    fetchProducts()
  }, [productCount])

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
    try {
      const confirmed = confirm(
        `Purchase ${products.find(p => p.id === productId)?.name}?\n\n` +
        `Price: ${products.find(p => p.id === productId)?.price} ETH\n\n` +
        `This will send ${products.find(p => p.id === productId)?.price} ETH to the campaign.`
      )
      
      if (!confirmed) return
      
      // Here you would call the purchase function
      // await purchaseProduct(BigInt(productId), BigInt(priceInWei))
      alert('Product purchase initiated! (Integration with smart contract pending)')
      
      // Refresh products
      fetchProducts()
    } catch (error) {
      console.error('Error purchasing product:', error)
      alert('Error purchasing product')
    }
  }

  const campaigns = [...new Set(products.map(p => p.campaign))]

  return (
    <div className="min-h-screen bg-gray-50">
      <Navigation />
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-4xl font-bold text-gray-900">Marketplace</h1>
            <p className="text-gray-600">Sustainable fashion products supporting NGOs</p>
          </div>
          <ConnectButton />
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
        {isLoading ? (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
            <p className="mt-4 text-gray-600">Loading products...</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {filteredProducts.map((product) => (
              <div key={product.id} className="bg-white rounded-lg shadow hover:shadow-lg transition-shadow">
                  <div className="relative">
                    <img
                      src={product.image}
                      alt={product.name}
                      className="w-full h-48 object-cover rounded-t-lg"
                    />
                    <div className="absolute top-2 left-2 bg-green-500 text-white px-2 py-1 rounded text-sm font-medium">
                      {product.campaign}
                    </div>
                    <div className="absolute top-2 right-2 bg-blue-500 text-white px-2 py-1 rounded text-sm font-medium">
                      Available
                    </div>
                  </div>
                
                <div className="p-4">
                  <h3 className="text-lg font-semibold text-gray-900 mb-2 line-clamp-2">
                    {product.name}
                  </h3>
                  <p className="text-gray-600 text-sm mb-3 line-clamp-2">
                    {product.description}
                  </p>
                  
                  <div className="flex items-center justify-between mb-3">
                    <span className="text-2xl font-bold text-green-600">
                      {product.price} ETH
                    </span>
                    <span className="text-sm text-gray-500">
                      {product.category}
                    </span>
                  </div>
                  
                  <div className="flex gap-2">
                    <button
                      onClick={() => handlePurchaseProduct(product.id, product.priceInWei)}
                      className="flex-1 py-2 px-4 rounded-md font-medium transition-colors bg-green-600 text-white hover:bg-green-700"
                    >
                      Buy Now
                    </button>
                    <button className="p-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors">
                      <Heart className="w-5 h-5 text-gray-400" />
                    </button>
                    <button className="p-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors">
                      <Eye className="w-5 h-5 text-gray-400" />
                    </button>
                  </div>
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
