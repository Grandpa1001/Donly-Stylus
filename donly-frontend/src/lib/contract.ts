export const DONLY_ADDRESS = process.env.NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS || '0x0000000000000000000000000000000000000000'

export const DONLY_ABI = [
  {
    "type": "function",
    "name": "create_category",
    "inputs": [{"type": "string", "name": "name"}],
    "outputs": [{"type": "uint256"}]
  },
  {
    "type": "function", 
    "name": "create_campaign",
    "inputs": [
      {"type": "uint256", "name": "category_id"},
      {"type": "string", "name": "title"},
      {"type": "string", "name": "description"},
      {"type": "string", "name": "image_url"},
      {"type": "address", "name": "destination_wallet"},
      {"type": "uint256", "name": "max_sold_products"}
    ],
    "outputs": [{"type": "uint256"}]
  },
  {
    "type": "function",
    "name": "add_product",
    "inputs": [
      {"type": "string", "name": "name"},
      {"type": "string", "name": "description"}, 
      {"type": "string", "name": "image_url"},
      {"type": "uint256", "name": "price"},
      {"type": "uint256", "name": "campaign_id"},
      {"type": "uint256", "name": "category_id"}
    ],
    "outputs": [{"type": "uint256"}]
  },
  {
    "type": "function",
    "name": "purchase_product",
    "inputs": [{"type": "uint256", "name": "product_id"}],
    "outputs": [{"type": "bool"}]
  },
  {
    "type": "function",
    "name": "get_category_count",
    "inputs": [],
    "outputs": [{"type": "uint256"}]
  },
  {
    "type": "function",
    "name": "get_campaign_count", 
    "inputs": [],
    "outputs": [{"type": "uint256"}]
  },
  {
    "type": "function",
    "name": "get_product_count",
    "inputs": [],
    "outputs": [{"type": "uint256"}]
  }
] as const