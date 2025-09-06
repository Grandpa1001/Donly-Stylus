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
    const campaignId = BigInt(params.id)
    
    // Fetch all campaign data in one call
    const [categoryId, admin, isActive, soldProductsCount, maxSoldProducts, destinationWallet] = await client.readContract({
      address: CONTRACT_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'getCampaignData',
      args: [campaignId]
    })

    return NextResponse.json({
      id: campaignId.toString(),
      categoryId: categoryId.toString(),
      admin: admin,
      isActive: isActive,
      soldProductsCount: soldProductsCount.toString(),
      maxSoldProducts: maxSoldProducts.toString(),
      destinationWallet: destinationWallet,
      progress: maxSoldProducts > 0n ? 
        Math.round((Number(soldProductsCount) / Number(maxSoldProducts)) * 100) : 0
    })
  } catch (error) {
    console.error('Error fetching campaign data:', error)
    return NextResponse.json(
      { error: 'Failed to fetch campaign data' },
      { status: 500 }
    )
  }
}
