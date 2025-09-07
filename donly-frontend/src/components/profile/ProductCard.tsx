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

interface ProductCardProps {
  product: Product
  type: 'active' | 'purchased' | 'inactive'
}

export default function ProductCard({ product, type }: ProductCardProps) {
  const getBadgeColor = () => {
    switch (type) {
      case 'active':
        return 'bg-green-500'
      case 'purchased':
        return 'bg-blue-500'
      case 'inactive':
        return 'bg-gray-500'
      default:
        return 'bg-gray-500'
    }
  }

  const getBadgeText = () => {
    switch (type) {
      case 'active':
        return product.campaign
      case 'purchased':
        return 'Purchased'
      case 'inactive':
        return product.isSold ? 'Sold' : 'Inactive'
      default:
        return ''
    }
  }

  const getStatusText = () => {
    switch (type) {
      case 'active':
        return null
      case 'purchased':
        return 'Status: Sold'
      case 'inactive':
        return `Status: ${product.isSold ? 'Sold to someone else' : 'Not sold'}`
      default:
        return null
    }
  }

  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <div className="relative mb-3">
        {product.image ? (
          <img
            src={product.image}
            alt={product.name}
            className="w-full h-40 object-cover rounded-lg"
            onError={(e) => {
              const target = e.target as HTMLImageElement;
              target.src = 'https://images.unsplash.com/photo-1441986300917-64674bd600d8?w=400&h=400&fit=crop';
            }}
          />
        ) : (
          <div className="w-full h-40 bg-gray-200 rounded-lg flex items-center justify-center">
            <span className="text-gray-400">No image</span>
          </div>
        )}
        <div className={`absolute top-2 left-2 ${getBadgeColor()} text-white px-2 py-1 rounded text-xs font-medium`}>
          {getBadgeText()}
        </div>
      </div>
      
      <h3 className="font-semibold text-gray-900 mb-1">{product.name}</h3>
      <p className="text-sm text-gray-600 mb-2 line-clamp-2">{product.description}</p>
      
      <div className="flex justify-between items-center mb-3">
        <span className="text-lg font-bold text-green-600">
          {parseFloat(product.price) < 0.001 ? '<0.001' : parseFloat(product.price).toFixed(3)} ETH
        </span>
        <span className="text-xs text-gray-500">{product.categoryName}</span>
      </div>
      
      {type === 'active' ? (
        <div className="flex gap-2">
          <button className="flex-1 bg-green-600 text-white py-2 px-3 rounded text-sm hover:bg-green-700 transition-colors">
            Edit
          </button>
          <button className="flex-1 bg-gray-600 text-white py-2 px-3 rounded text-sm hover:bg-gray-700 transition-colors">
            Deactivate
          </button>
        </div>
      ) : (
        <div className="text-center">
          <span className="text-sm text-gray-500">{getStatusText()}</span>
        </div>
      )}
    </div>
  )
}
