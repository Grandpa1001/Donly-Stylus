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
    const categoryId = BigInt(params.id)
    
    // Fetch category data from contract
    const [creator, isActive] = await Promise.all([
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getCategoryCreator',
        args: [categoryId]
      }),
      client.readContract({
        address: CONTRACT_ADDRESS as `0x${string}`,
        abi: DONLY_ABI,
        functionName: 'getCategoryIsActive',
        args: [categoryId]
      })
    ])

    return NextResponse.json({
      id: categoryId.toString(),
      creator: creator,
      isActive: isActive
    })
  } catch (error) {
    console.error('Error fetching category data:', error)
    return NextResponse.json(
      { error: 'Failed to fetch category data' },
      { status: 500 }
    )
  }
}
