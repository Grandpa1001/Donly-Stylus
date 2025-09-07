import { Plus, Package, ShoppingBag, Archive } from 'lucide-react'

interface EmptyStateProps {
  type: 'active' | 'purchased' | 'inactive'
  onAddProduct?: () => void
}

export default function EmptyState({ type, onAddProduct }: EmptyStateProps) {
  const getIcon = () => {
    switch (type) {
      case 'active':
        return Package
      case 'purchased':
        return ShoppingBag
      case 'inactive':
        return Archive
      default:
        return Package
    }
  }

  const getTitle = () => {
    switch (type) {
      case 'active':
        return 'No Active Products'
      case 'purchased':
        return 'No Purchased Products'
      case 'inactive':
        return 'No Inactive Products'
      default:
        return 'No Products'
    }
  }

  const getDescription = () => {
    switch (type) {
      case 'active':
        return 'You don\'t have any active products for sale yet.'
      case 'purchased':
        return 'You haven\'t purchased any products yet.'
      case 'inactive':
        return 'You don\'t have any inactive products yet.'
      default:
        return 'No products.'
    }
  }

  const Icon = getIcon()

  return (
    <div className="text-center py-12">
      <Icon className="w-16 h-16 text-gray-400 mx-auto mb-4" />
      <h3 className="text-lg font-semibold text-gray-900 mb-2">{getTitle()}</h3>
      <p className="text-gray-600 mb-6">{getDescription()}</p>
      {type === 'active' && onAddProduct && (
        <button 
          onClick={onAddProduct}
          className="bg-green-600 text-white px-6 py-3 rounded-md hover:bg-green-700 transition-colors flex items-center space-x-2 mx-auto"
        >
          <Plus className="w-4 h-4" />
          <span>Add New Product</span>
        </button>
      )}
    </div>
  )
}
