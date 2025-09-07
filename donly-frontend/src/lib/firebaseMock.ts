// Mock Firebase for development/testing
export interface MockCategory {
  id: string
  name: string
  blockchainId: number
  createdAt: Date
  updatedAt: Date
}

// In-memory storage for development
let mockCategories: MockCategory[] = []

export const mockFirebase = {
  // Mock categories collection
  categories: {
    async add(data: Omit<MockCategory, 'id'>): Promise<{ id: string }> {
      const id = `mock-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
      const category: MockCategory = {
        id,
        ...data,
      }
      mockCategories.push(category)
      console.log('🔥 Mock Firebase: Added category', category)
      return { id }
    },

    async get(): Promise<MockCategory[]> {
      console.log('🔥 Mock Firebase: Retrieved categories', mockCategories)
      return [...mockCategories]
    },

    async getByBlockchainId(blockchainId: number): Promise<MockCategory | undefined> {
      const category = mockCategories.find(cat => cat.blockchainId === blockchainId)
      console.log('🔥 Mock Firebase: Retrieved category by blockchain ID', blockchainId, category)
      return category
    },

    async clear(): Promise<void> {
      mockCategories = []
      console.log('🔥 Mock Firebase: Cleared all categories')
    },

    async count(): Promise<number> {
      return mockCategories.length
    }
  }
}

// Check if we should use mock Firebase
export function shouldUseMockFirebase(): boolean {
  const apiKey = process.env.NEXT_PUBLIC_FIREBASE_API_KEY
  return !apiKey || apiKey === 'your_api_key_here' || apiKey === 'demo-key'
}

// Initialize mock data
export function initializeMockData() {
  if (shouldUseMockFirebase()) {
    console.log('🔥 Using Mock Firebase for development')
    
    // Add some sample categories
    mockFirebase.categories.add({
      name: 'Elektronika',
      blockchainId: 1,
      createdAt: new Date(),
      updatedAt: new Date(),
    })
    
    mockFirebase.categories.add({
      name: 'Zrównoważony rozwój',
      blockchainId: 2,
      createdAt: new Date(),
      updatedAt: new Date(),
    })
    
    mockFirebase.categories.add({
      name: 'Moda',
      blockchainId: 3,
      createdAt: new Date(),
      updatedAt: new Date(),
    })
    
    console.log('🔥 Mock Firebase initialized with sample data')
  }
}
