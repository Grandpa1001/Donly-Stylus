export const DONLY_ADDRESS = process.env.NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS || '0x2602c51a914d9bd5c10a96033661b09d03f805f0'

export const DONLY_ABI = [
  // Category functions
  {
    "type": "function",
    "name": "categoryCount",
    "inputs": [],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createCategory",
    "inputs": [{"type": "string", "name": "name"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCategoryNameHash",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCategoryCreator",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "address"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCategoryIsActive",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "bool"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deactivateCategory",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [],
    "stateMutability": "nonpayable"
  },

  // Campaign functions
  {
    "type": "function",
    "name": "campaignCount",
    "inputs": [],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createCampaign",
    "inputs": [
      {"type": "uint256", "name": "category_id"},
      {"type": "string", "name": "title"},
      {"type": "string", "name": "description"},
      {"type": "address", "name": "destination_wallet"},
      {"type": "uint256", "name": "max_sold_products"}
    ],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCampaignCategoryId",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCampaignAdmin",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "address"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCampaignIsActive",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "bool"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCampaignSoldProductsCount",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCampaignMaxSoldProducts",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deactivateCampaign",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [],
    "stateMutability": "nonpayable"
  },

  // Product functions
  {
    "type": "function",
    "name": "productCount",
    "inputs": [],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addProduct",
    "inputs": [
      {"type": "uint256", "name": "campaign_id"},
      {"type": "uint256", "name": "category_id"},
      {"type": "string", "name": "name"},
      {"type": "string", "name": "description"},
      {"type": "uint256", "name": "price"}
    ],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getProductCampaignId",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getProductPrice",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getProductIsActive",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "bool"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getProductIsSold",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "bool"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "purchaseProduct",
    "inputs": [{"type": "uint256", "name": "product_id"}],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deactivateProduct",
    "inputs": [{"type": "uint256", "name": "product_id"}],
    "outputs": [],
    "stateMutability": "nonpayable"
  }
] as const