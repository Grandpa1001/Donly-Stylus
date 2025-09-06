import { useWriteContract, useReadContract } from 'wagmi'
import { DONLY_ADDRESS, DONLY_ABI } from '../lib/contract'

export function useContract() {
  const { writeContract } = useWriteContract()

  const createCategory = async (name: string) => {
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'createCategory',
      args: [name]
    })
  }

  const createCampaign = async (
    categoryId: bigint,
    title: string,
    description: string,
    destinationWallet: `0x${string}`,
    maxSoldProducts: bigint
  ) => {
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'createCampaign',
      args: [categoryId, title, description, destinationWallet, maxSoldProducts]
    })
  }

  const addProduct = async (
    campaignId: bigint,
    categoryId: bigint,
    name: string,
    description: string,
    price: bigint
  ) => {
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'addProduct',
      args: [campaignId, categoryId, name, description, price]
    })
  }

  const purchaseProduct = async (productId: bigint) => {
    return writeContract({
      address: DONLY_ADDRESS as `0x${string}`,
      abi: DONLY_ABI,
      functionName: 'purchaseProduct',
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

// Hook do odczytu danych konkretnych elementów
export function useCategoryData(categoryId: bigint | undefined) {
  const { data: nameHash } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCategoryNameHash',
    args: categoryId ? [categoryId] : undefined
  })

  const { data: creator } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCategoryCreator',
    args: categoryId ? [categoryId] : undefined
  })

  const { data: isActive } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCategoryIsActive',
    args: categoryId ? [categoryId] : undefined
  })

  return {
    nameHash,
    creator,
    isActive
  }
}

export function useCampaignData(campaignId: bigint | undefined) {
  const { data: categoryId } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCampaignCategoryId',
    args: campaignId ? [campaignId] : undefined
  })

  const { data: admin } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCampaignAdmin',
    args: campaignId ? [campaignId] : undefined
  })

  const { data: isActive } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCampaignIsActive',
    args: campaignId ? [campaignId] : undefined
  })

  const { data: soldProductsCount } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCampaignSoldProductsCount',
    args: campaignId ? [campaignId] : undefined
  })

  const { data: maxSoldProducts } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getCampaignMaxSoldProducts',
    args: campaignId ? [campaignId] : undefined
  })


  return {
    categoryId,
    admin,
    isActive,
    soldProductsCount,
    maxSoldProducts
  }
}

export function useProductData(productId: bigint | undefined) {
  const { data: campaignId } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getProductCampaignId',
    args: productId ? [productId] : undefined
  })

  const { data: price } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getProductPrice',
    args: productId ? [productId] : undefined
  })

  const { data: isActive } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getProductIsActive',
    args: productId ? [productId] : undefined
  })

  const { data: isSold } = useReadContract({
    address: DONLY_ADDRESS as `0x${string}`,
    abi: DONLY_ABI,
    functionName: 'getProductIsSold',
    args: productId ? [productId] : undefined
  })


  return {
    campaignId,
    price,
    isActive,
    isSold
  }
}