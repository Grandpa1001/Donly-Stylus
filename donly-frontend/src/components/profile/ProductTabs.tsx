import { Package, ShoppingBag, Archive } from 'lucide-react'

interface ProductTabsProps {
  activeTab: 'active' | 'purchased' | 'inactive'
  setActiveTab: (tab: 'active' | 'purchased' | 'inactive') => void
}

export default function ProductTabs({ activeTab, setActiveTab }: ProductTabsProps) {
  return (
    <div className="border-b border-gray-200">
      <nav className="flex space-x-8 px-6">
        <button
          onClick={() => setActiveTab('active')}
          className={`py-4 px-1 border-b-2 font-medium text-sm ${
            activeTab === 'active'
              ? 'border-green-500 text-green-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
          }`}
        >
          <div className="flex items-center space-x-2">
            <Package className="w-4 h-4" />
            <span>Active Sales</span>
          </div>
        </button>
        <button
          onClick={() => setActiveTab('purchased')}
          className={`py-4 px-1 border-b-2 font-medium text-sm ${
            activeTab === 'purchased'
              ? 'border-green-500 text-green-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
          }`}
        >
          <div className="flex items-center space-x-2">
            <ShoppingBag className="w-4 h-4" />
            <span>Purchased</span>
          </div>
        </button>
        <button
          onClick={() => setActiveTab('inactive')}
          className={`py-4 px-1 border-b-2 font-medium text-sm ${
            activeTab === 'inactive'
              ? 'border-green-500 text-green-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
          }`}
        >
          <div className="flex items-center space-x-2">
            <Archive className="w-4 h-4" />
            <span>Inactive</span>
          </div>
        </button>
      </nav>
    </div>
  )
}
