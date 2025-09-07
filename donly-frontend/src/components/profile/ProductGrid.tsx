import ProductCard from './ProductCard'
import EmptyState from './EmptyState'
import { Plus } from 'lucide-react'

interface Product {
  id: number
  name: string
  description: string
  price: string
  image: string
  campaign: string
  categoryName: string
  isActive: boolean
  isSold: boolean
}

interface ProductGridProps {
  products: Product[]
  type: 'active' | 'purchased' | 'inactive'
  onAddProduct?: () => void
}

export default function ProductGrid({ products, type, onAddProduct }: ProductGridProps) {
  if (products.length === 0) {
    return <EmptyState type={type} onAddProduct={onAddProduct} />
  }

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-semibold text-gray-900">
          {type === 'active' && 'Active Products for Sale'}
          {type === 'purchased' && 'Purchased Products'}
          {type === 'inactive' && 'Inactive Products'}
        </h2>
        {type === 'active' && products.length === 0 && (
          <button 
            onClick={onAddProduct}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 transition-colors flex items-center space-x-2"
          >
            <Plus className="w-4 h-4" />
            <span>Add New Product</span>
          </button>
        )}
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <ProductCard key={product.id} product={product} type={type} />
        ))}
      </div>
    </div>
  )
}
