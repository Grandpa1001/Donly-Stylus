'use client'

import Link from 'next/link'
import { ConnectButton } from '@rainbow-me/rainbowkit'

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      <div className="max-w-6xl mx-auto px-4 py-16">
        {/* Header */}
        <div className="text-center mb-16">
          <div className="flex justify-center mb-6">
            <ConnectButton />
          </div>
          <h1 className="text-5xl font-bold text-gray-900 mb-6">
            Donly
          </h1>
          <p className="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
            Platforma crowdfundingowa zbudowana na Arbitrum Stylus. 
            Twórz kampanie, dodawaj produkty i zbieraj środki w sposób zdecentralizowany.
          </p>
        </div>

        {/* Features Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
          <div className="bg-white p-8 rounded-xl shadow-lg">
            <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
              <svg className="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </div>
            <h3 className="text-xl font-semibold text-gray-900 mb-3">Kategorie</h3>
            <p className="text-gray-600 mb-4">
              Organizuj swoje kampanie w kategoriach. Twórz kategorie tematyczne dla lepszej organizacji.
            </p>
          </div>

          <div className="bg-white p-8 rounded-xl shadow-lg">
            <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
              <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
              </svg>
            </div>
            <h3 className="text-xl font-semibold text-gray-900 mb-3">Kampanie</h3>
            <p className="text-gray-600 mb-4">
              Twórz kampanie crowdfundingowe z określonymi celami i limitami sprzedaży produktów.
            </p>
          </div>

          <div className="bg-white p-8 rounded-xl shadow-lg">
            <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
              <svg className="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
            </div>
            <h3 className="text-xl font-semibold text-gray-900 mb-3">Produkty</h3>
            <p className="text-gray-600 mb-4">
              Dodawaj produkty do kampanii. Ustaw ceny i zarządzaj sprzedażą w czasie rzeczywistym.
            </p>
          </div>
        </div>

        {/* CTA Section */}
        <div className="bg-white rounded-2xl shadow-xl p-12 text-center">
          <h2 className="text-3xl font-bold text-gray-900 mb-6">
            Gotowy do testowania?
          </h2>
          <p className="text-lg text-gray-600 mb-8 max-w-2xl mx-auto">
            Przejdź do panelu testowego, aby przetestować wszystkie funkcje smart contractu 
            na sieci Arbitrum Sepolia.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link 
              href="/test"
              className="bg-blue-600 text-white px-8 py-4 rounded-lg font-semibold hover:bg-blue-700 transition-colors"
            >
              Panel Testowy
            </Link>
            <a 
              href="https://sepolia.arbiscan.io/address/0x2602c51a914d9bd5c10a96033661b09d03f805f0"
              target="_blank"
              rel="noopener noreferrer"
              className="bg-gray-100 text-gray-700 px-8 py-4 rounded-lg font-semibold hover:bg-gray-200 transition-colors"
            >
              Zobacz na Arbiscan
            </a>
          </div>
        </div>

        {/* Contract Info */}
        <div className="mt-16 text-center">
          <div className="bg-gray-50 rounded-lg p-6 max-w-2xl mx-auto">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Informacje o kontrakcie</h3>
            <div className="space-y-2 text-sm text-gray-600">
              <p>
                <span className="font-medium">Adres:</span> 
                <span className="font-mono ml-2">0x2602c51a914d9bd5c10a96033661b09d03f805f0</span>
              </p>
              <p>
                <span className="font-medium">Sieć:</span> 
                <span className="ml-2">Arbitrum Sepolia</span>
              </p>
              <p>
                <span className="font-medium">Typ:</span> 
                <span className="ml-2">Arbitrum Stylus (Rust)</span>
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}