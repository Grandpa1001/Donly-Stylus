import { NextRequest, NextResponse } from 'next/server'
import { createPublicClient, http } from 'viem'
import { arbitrumSepolia } from 'viem/chains'
import { DONLY_ABI } from '../../../../lib/contract'

const client = createPublicClient({
  chain: arbitrumSepolia,
  transport: http(process.env.NEXT_PUBLIC_ARBITRUM_SEPOLIA_RPC_URL)
})

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS || '0xc2ad3070ff0a301f5df343d889da2a08eacd9792'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const productId = BigInt(params.id)
    
    // Fetch all product data in one call
    const [campaignId, price, isActive, isSold] = await Promise.all([
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getProductCampaignId',
        args: [productId]
      }),
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getProductPrice',
        args: [productId]
      }),
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getProductIsActive',
        args: [productId]
      }),
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getProductIsSold',
        args: [productId]
      })
    ])

    return NextResponse.json({
      id: productId.toString(),
      campaignId: campaignId.toString(),
      price: price.toString(),
      isActive: isActive,
      isSold: isSold,
      priceInEth: Number(price) / 1e18 // Convert wei to ETH for display
    })
  } catch (error) {
    console.error('Error fetching product data:', error)
    return NextResponse.json(
      { error: 'Failed to fetch product data' },
      { status: 500 }
    )
  }
}
