import { User } from 'lucide-react'

interface ProfileHeaderProps {
  address: string
}

export default function ProfileHeader({ address }: ProfileHeaderProps) {
  return (
    <div className="text-center mb-8">
      <div className="w-20 h-20 bg-gradient-to-br from-green-600 to-blue-600 rounded-full flex items-center justify-center mx-auto mb-4">
        <User className="w-10 h-10 text-white" />
      </div>
      
      <h1 className="text-4xl font-bold text-gray-900 mb-2">
        User Profile
      </h1>
      
      <div className="bg-white rounded-lg shadow-sm p-6 max-w-lg mx-auto">
        <h3 className="text-lg font-semibold text-gray-900 mb-2">Your Wallet Address:</h3>
        <p className="text-sm text-gray-600 font-mono break-all">
          {address}
        </p>
      </div>
    </div>
  )
}
