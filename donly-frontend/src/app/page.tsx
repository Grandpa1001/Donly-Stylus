'use client'

import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useAccount } from 'wagmi'
import { useContract, useContractRead } from '../hooks/useContract'
import { useState } from 'react'

export default function Home() {
  const { isConnected, address } = useAccount()
  const { createCategory, createCampaign, addProduct, purchaseProduct } = useContract()
  const { categoryCount, campaignCount, productCount } = useContractRead()
  const [status, setStatus] = useState('')
  const [loading, setLoading] = useState(false)

  const testCreateCategory = async () => {
    setLoading(true)
    setStatus('Tworzenie kategorii...')
    
    try {
      await createCategory('Electronics')
      setStatus('✅ Kategoria utworzona! Sprawdź transakcję w portfelu.')
    } catch (error) {
      setStatus(`❌ Błąd: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const testCreateCampaign = async () => {
    setLoading(true)
    setStatus('Tworzenie kampanii...')
    
    try {
      await createCampaign(
        1n, // category_id
        'Test Campaign',
        'Test Description', 
        'https://example.com/image.jpg',
        address!,
        10n // max_products
      )
      setStatus('✅ Kampania utworzona! Sprawdź transakcję w portfelu.')
    } catch (error) {
      setStatus(`❌ Błąd: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const testAddProduct = async () => {
    setLoading(true)
    setStatus('Dodawanie produktu...')
    
    try {
      await addProduct(
        'Test Product',
        'Test Description',
        'https://example.com/product.jpg',
        1000000000000000000n, // 1 ETH w wei
        1n, // campaign_id
        1n  // category_id
      )
      setStatus('✅ Produkt dodany! Sprawdź transakcję w portfelu.')
    } catch (error) {
      setStatus(`❌ Błąd: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const testPurchaseProduct = async () => {
    setLoading(true)
    setStatus('Kupowanie produktu...')
    
    try {
      await purchaseProduct(1n) // product_id
      setStatus('✅ Produkt kupiony! Sprawdź transakcję w portfelu.')
    } catch (error) {
      setStatus(`❌ Błąd: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-4xl mx-auto px-6">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">
            🎯 Donly Smart Contract Tester
          </h1>
          <p className="text-lg text-gray-600 mb-6">
            Platforma crowdfundingowa na Arbitrum Stylus
          </p>
          <ConnectButton />
        </div>

        {/* Contract Stats */}
        {isConnected && (
          <div className="mb-8 grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-white p-4 rounded-lg shadow-sm border text-center">
              <h3 className="font-semibold text-gray-800">Kategorie</h3>
              <p className="text-2xl font-bold text-blue-600">
                {categoryCount?.toString() || '0'}
              </p>
            </div>
            <div className="bg-white p-4 rounded-lg shadow-sm border text-center">
              <h3 className="font-semibold text-gray-800">Kampanie</h3>
              <p className="text-2xl font-bold text-green-600">
                {campaignCount?.toString() || '0'}
              </p>
            </div>
            <div className="bg-white p-4 rounded-lg shadow-sm border text-center">
              <h3 className="font-semibold text-gray-800">Produkty</h3>
              <p className="text-2xl font-bold text-purple-600">
                {productCount?.toString() || '0'}
              </p>
            </div>
          </div>
        )}

        {/* Status */}
        {status && (
          <div className="mb-8 p-4 bg-white rounded-lg shadow-sm border">
            <h3 className="font-semibold text-gray-800 mb-2">Status:</h3>
            <p className="text-gray-700">{status}</p>
          </div>
        )}

        {/* Test Buttons */}
        {isConnected && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Categories */}
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <h2 className="text-xl font-semibold text-gray-800 mb-4">
                📂 Kategorie
              </h2>
              <p className="text-sm text-gray-600 mb-4">
                Utwórz kategorię "Electronics" do testowania
              </p>
              <button
                onClick={testCreateCategory}
                disabled={loading}
                className="w-full bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {loading ? 'Ładowanie...' : 'Dodaj kategorię "Electronics"'}
              </button>
            </div>

            {/* Campaigns */}
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <h2 className="text-xl font-semibold text-gray-800 mb-4">
                🚀 Kampanie
              </h2>
              <p className="text-sm text-gray-600 mb-4">
                Utwórz kampanię testową z maksymalnie 10 produktami
              </p>
              <button
                onClick={testCreateCampaign}
                disabled={loading}
                className="w-full bg-green-500 hover:bg-green-600 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {loading ? 'Ładowanie...' : 'Utwórz kampanię testową'}
              </button>
            </div>

            {/* Products */}
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <h2 className="text-xl font-semibold text-gray-800 mb-4">
                📦 Produkty
              </h2>
              <p className="text-sm text-gray-600 mb-4">
                Dodaj produkt testowy za 1 ETH
              </p>
              <button
                onClick={testAddProduct}
                disabled={loading}
                className="w-full bg-purple-500 hover:bg-purple-600 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {loading ? 'Ładowanie...' : 'Dodaj produkt testowy'}
              </button>
            </div>

            {/* Purchase */}
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <h2 className="text-xl font-semibold text-gray-800 mb-4">
                💰 Zakupy
              </h2>
              <p className="text-sm text-gray-600 mb-4">
                Kup produkt o ID 1 (wymaga ETH w portfelu)
              </p>
              <button
                onClick={testPurchaseProduct}
                disabled={loading}
                className="w-full bg-orange-500 hover:bg-orange-600 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {loading ? 'Ładowanie...' : 'Kup produkt testowy'}
              </button>
            </div>
          </div>
        )}

        {/* Info */}
        <div className="mt-12 bg-blue-50 border border-blue-200 rounded-lg p-6">
          <h3 className="text-lg font-semibold text-blue-800 mb-2">
            ℹ️ Informacje
          </h3>
          <ul className="text-blue-700 space-y-1 text-sm">
            <li>• Smart contract: Donly na Arbitrum Stylus</li>
            <li>• Frontend: Next.js + wagmi + RainbowKit</li>
            <li>• Status: Testowanie podstawowych funkcjonalności</li>
            <li>• Następny krok: Wdrożenie na testnet</li>
          </ul>
        </div>

        {/* Warning */}
        {!isConnected && (
          <div className="mt-8 bg-yellow-50 border border-yellow-200 rounded-lg p-6">
            <h3 className="text-lg font-semibold text-yellow-800 mb-2">
              ⚠️ Wymagane połączenie
            </h3>
            <p className="text-yellow-700 text-sm">
              Połącz portfel, aby móc testować funkcjonalności smart contractu.
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
