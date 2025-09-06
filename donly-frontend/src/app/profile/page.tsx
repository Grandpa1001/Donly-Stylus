import Navigation from '@/components/Navigation'
import { User } from 'lucide-react'

export default function ProfilePage() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Navigation />
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="text-center">
          <div className="w-24 h-24 bg-gradient-to-br from-green-600 to-blue-600 rounded-full flex items-center justify-center mx-auto mb-8">
            <User className="w-12 h-12 text-white" />
          </div>
          
          <h1 className="text-6xl font-bold text-gray-900 mb-6">
            Profile
          </h1>
          
          <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
            Tutaj będzie znajdować się profil użytkownika z zarządzaniem produktami,
            statystykami wpływu i ustawieniami konta.
          </p>
          
          <div className="bg-white rounded-lg shadow-sm p-8 max-w-md mx-auto">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Planowane funkcje:</h3>
            <ul className="text-left text-gray-600 space-y-2">
              <li>• Zarządzanie produktami</li>
              <li>• Statystyki wpływu</li>
              <li>• Historia transakcji</li>
              <li>• Ustawienia konta</li>
              <li>• Wspierane kampanie</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}
