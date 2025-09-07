import { useWriteContract, useReadContract } from 'wagmi'
import { DONLY_ADDRESS, DONLY_ABI } from '../lib/contract'

export function useContract() {
  const { writeContract } = useWriteContract()

  const createCategory = async () => {
    console.log(`🔄 Creating Category`)
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'createCategory',
      args: []
    })
  }

  const createCampaign = async (
    categoryId: bigint,
    destinationWallet: `0x${string}`,
    maxSoldProducts: bigint
  ) => {
    console.log(`🔄 Creating Campaign`)
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'createCampaign',
      args: [categoryId, destinationWallet, maxSoldProducts]
    })
  }

  const addProduct = async (
    campaignId: bigint,
    categoryId: bigint,
    price: bigint
  ) => {
    console.log(`🔄 Adding Product`)
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'addProduct',
      args: [campaignId, categoryId, price]
    })
  }

  const purchaseProduct = async (productId: bigint, price: bigint) => {
    console.log(`🔄 Purchasing Product #${productId}`)
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'purchaseProduct',
      args: [productId],
      value: price
    })
  }

  const withdrawCampaignFunds = async (campaignId: bigint) => {
    console.log(`🔄 Withdrawing Campaign Funds #${campaignId}`)
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'withdrawCampaignFunds',
      args: [campaignId]
    })
  }


  return {
    createCategory,
    createCampaign,
    addProduct,
    purchaseProduct,
    withdrawCampaignFunds
  }
}

export function useContractRead() {
  const { data: categoryCount } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'categoryCount'
  })

  const { data: campaignCount } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'campaignCount'
  })

  const { data: productCount } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'productCount'
  })

  return {
    categoryCount,
    campaignCount,
    productCount
  }
}

// Removed individual data hooks - we use API endpoints instead