import { useWriteContract, useReadContract } from 'wagmi'
import { DONLY_ADDRESS, DONLY_ABI } from '../lib/contract'

export function useContract() {
  const { writeContract } = useWriteContract()

  const createCategory = async (name: string) => {
    return writeContract({
      address: DONLY_ADDRESS,
      abi: DONLY_ABI,
      functionName: 'create_category',
      args: [name]
    })
  }

  const createCampaign = async (
    categoryId: bigint,
    title: string,
    description: string,
    imageUrl: string,
    destinationWallet: `0x${string}`,
    maxProducts: bigint
  ) => {
    return writeContract({
      address: DONLY_ADDRESS,
      abi: DONLY_ABI,
      functionName: 'create_campaign',
      args: [categoryId, title, description, imageUrl, destinationWallet, maxProducts]
    })
  }

  const addProduct = async (
    name: string,
    description: string,
    imageUrl: string,
    price: bigint,
    campaignId: bigint,
    categoryId: bigint
  ) => {
    return writeContract({
      address: DONLY_ADDRESS,
      abi: DONLY_ABI,
      functionName: 'add_product',
      args: [name, description, imageUrl, price, campaignId, categoryId]
    })
  }

  const purchaseProduct = async (productId: bigint) => {
    return writeContract({
      address: DONLY_ADDRESS,
      abi: DONLY_ABI,
      functionName: 'purchase_product',
      args: [productId]
    })
  }

  return {
    createCategory,
    createCampaign,
    addProduct,
    purchaseProduct
  }
}

export function useContractRead() {
  const { data: categoryCount } = useReadContract({
    address: DONLY_ADDRESS,
    abi: DONLY_ABI,
    functionName: 'get_category_count'
  })

  const { data: campaignCount } = useReadContract({
    address: DONLY_ADDRESS,
    abi: DONLY_ABI,
    functionName: 'get_campaign_count'
  })

  const { data: productCount } = useReadContract({
    address: DONLY_ADDRESS,
    abi: DONLY_ABI,
    functionName: 'get_product_count'
  })

  return {
    categoryCount,
    campaignCount,
    productCount
  }
}
