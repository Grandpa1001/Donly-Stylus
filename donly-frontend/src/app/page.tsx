'use client'

import Link from 'next/link'
import Navigation from '@/components/Navigation'
import { CheckCircle, ArrowRight, Heart, ShieldCheck, Link2 } from 'lucide-react'

export default function HomePage() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Navigation />
      
      {/* Hero Section */}
      <section className="relative bg-gradient-to-br from-green-50 via-blue-50 to-purple-50 py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            <div>
              <h1 className="text-6xl font-bold text-gray-900 leading-tight mb-6">
                DOnly one step to
                <span className="text-transparent bg-clip-text bg-gradient-to-r from-green-600 to-blue-600"> Help</span>
              </h1>
              <p className="text-xl text-gray-600 mb-6 leading-relaxed">
                Where fashion brands liquidate responsibly. We help sell end-of-season collections, 
                turning surplus into support for NGOs driving sustainable change.
              </p>
              <div className="bg-white/80 backdrop-blur-sm rounded-lg p-4 mb-8 border border-gray-200">
                <div className="flex items-center space-x-2 text-sm font-medium text-gray-700">
                  <CheckCircle className="w-5 h-5 text-green-600" />
                  <span>Transparent.</span>
                  <CheckCircle className="w-5 h-5 text-blue-600" />
                  <span>Responsible.</span>
                  <CheckCircle className="w-5 h-5 text-purple-600" />
                  <span>Powered by Blockchain.</span>
                </div>
              </div>
              <div className="flex flex-col sm:flex-row gap-4">
                <Link href="/marketplace" className="bg-gradient-to-r from-green-600 to-blue-600 hover:from-green-700 hover:to-blue-700 text-white px-8 py-3 rounded-md font-medium flex items-center justify-center">
                  Browse Marketplace
                  <ArrowRight className="w-4 h-4 ml-2" />
                </Link>
                <Link href="/test" className="border border-gray-300 hover:bg-gray-50 px-8 py-3 rounded-md font-medium flex items-center justify-center">
                  Test Platform
                  <Heart className="w-4 h-4 ml-2" />
                </Link>
              </div>
            </div>
            <div className="relative">
              <div className="relative rounded-2xl overflow-hidden shadow-2xl">
                <div className="w-full h-96 bg-gradient-to-br from-green-100 to-blue-100 flex items-center justify-center">
                  <div className="text-center">
                    <div className="w-16 h-16 bg-gradient-to-r from-green-600 to-blue-600 rounded-full flex items-center justify-center mx-auto mb-4">
                      <Link2 className="w-8 h-8 text-white" />
                    </div>
                    <p className="text-gray-600 font-medium">Blockchain-verified transparency</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Value Proposition Section */}
      <section className="py-16 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold text-gray-900 mb-4">Our Core Values</h2>
            <p className="text-xl text-gray-600">What drives us to create positive impact</p>
          </div>
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            {[
              {
                title: 'Responsibility',
                description: 'We operate sustainably, prioritizing our stakeholders (employees, customers, local communities) and the planet.',
                icon: Heart,
                color: 'bg-green-100 text-green-600'
              },
              {
                title: 'Openness',
                description: 'Inclusive, open to new technologies (blockchain), and committed to equality and fighting discrimination.',
                icon: ShieldCheck,
                color: 'bg-blue-100 text-blue-600'
              },
              {
                title: 'Trustworthiness',
                description: 'Transparent fund distribution via blockchain. We select partners equitably and are worthy of trust.',
                icon: Link2,
                color: 'bg-purple-100 text-purple-600'
              },
              {
                title: 'Unity (Zgrani)',
                description: 'We listen to your needs and strive to meet them—we are here for you!',
                icon: Heart,
                color: 'bg-red-100 text-red-600'
              }
            ].map((value, index) => {
              const Icon = value.icon
              return (
                <div key={index} className="text-center p-6 border border-gray-200 rounded-xl hover:shadow-lg transition-all duration-300 hover:-translate-y-1">
                  <div className={`w-16 h-16 ${value.color} rounded-full flex items-center justify-center mx-auto mb-6`}>
                    <Icon className="w-8 h-8" />
                  </div>
                  <h3 className="text-xl font-semibold mb-4 text-gray-900">{value.title}</h3>
                  <p className="text-gray-600 leading-relaxed">{value.description}</p>
                </div>
              )
            })}
          </div>
        </div>
      </section>

      {/* Mission Statement */}
      <section className="py-16 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold text-gray-900 mb-4">Building Networks of Positive Impact</h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Our mission extends beyond commerce to create meaningful change in communities worldwide
            </p>
          </div>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {[
              'We facilitate better world-building through conscious purchasing processes.',
              'We satisfy customers while investing in their loyalty.',
              'Blockchain simplifies transparent reporting on fund distribution.',
              'We verify NGOs and their use of funds.',
              'We embrace modernity and trends to better fulfill our mission.',
              'We promote socially and environmentally responsible brands.'
            ].map((point, index) => (
              <div key={index} className="flex items-start space-x-3 p-4">
                <div className="flex-shrink-0 w-6 h-6 bg-green-100 rounded-full flex items-center justify-center mt-1">
                  <CheckCircle className="w-4 h-4 text-green-600" />
                </div>
                <p className="text-gray-700 leading-relaxed">{point}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 bg-gradient-to-r from-green-600 to-blue-600">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-4xl font-bold text-white mb-4">Ready to Make an Impact?</h2>
          <p className="text-xl text-green-100 mb-8 max-w-2xl mx-auto">
            Join thousands of conscious consumers and fashion brands creating positive change through sustainable commerce.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link href="/marketplace" className="bg-white text-green-600 hover:bg-gray-100 px-8 py-3 rounded-md font-medium flex items-center justify-center">
              Browse Marketplace
              <ArrowRight className="w-4 h-4 ml-2" />
            </Link>
            <Link href="/test" className="border border-white text-white hover:bg-white hover:text-green-600 px-8 py-3 rounded-md font-medium flex items-center justify-center">
              Test Platform
              <Heart className="w-4 h-4 ml-2" />
            </Link>
          </div>
        </div>
      </section>
    </div>
  )
}