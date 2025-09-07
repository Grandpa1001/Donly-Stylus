export const DONLY_ADDRESS = process.env.NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS || '0xb4e32dfc1c792424f57506a5113d40aae5fbc437'

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
    "inputs": [],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "nonpayable"
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
      {"type": "address", "name": "destination_wallet"},
      {"type": "uint256", "name": "max_sold_products"}
    ],
    "outputs": [{"type": "uint256"}],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCampaignData",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [
      {"type": "uint256", "name": "categoryId"},
      {"type": "address", "name": "admin"},
      {"type": "bool", "name": "isActive"},
      {"type": "uint256", "name": "soldProductsCount"},
      {"type": "uint256", "name": "maxSoldProducts"},
      {"type": "address", "name": "destinationWallet"}
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deactivateCampaign",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawCampaignFunds",
    "inputs": [{"type": "uint256", "name": "campaign_id"}],
    "outputs": [{"type": "bool"}],
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
    "name": "getProductOwner",
    "inputs": [{"type": "uint256", "name": "id"}],
    "outputs": [{"type": "address"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "purchaseProduct",
    "inputs": [{"type": "uint256", "name": "product_id"}],
    "outputs": [{"type": "bool"}],
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