![Image](./header.png)

# Donly Smart Contract

A crowdfunding platform smart contract built on **Arbitrum Stylus** using Rust and the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs), focused on sustainable fashion and responsible inventory liquidation. 

## 🚀 **Project Status**

### ✅ **Deployed on Arbitrum Sepolia Testnet**
- **Contract Address:** `0xb4e32dfc1c792424f57506a5113d40aae5fbc437`
- **Network:** Arbitrum Sepolia (Chain ID: 421614)
- **Status:** Active and ready for use
- **Size:** Optimized for gas efficiency
- **Cache:** Cached in ArbOS for cheaper calls

### 📋 **Implemented Features**

#### 🏗️ **Architecture**
- ✅ **Dynamic structures** - No hardcoded limits
- ✅ **Storage optimization** - Using `sol_storage!` macro
- ✅ **Error handling** - Comprehensive error system
- ✅ **Gas efficiency** - Optimized code for minimal gas usage

#### 🎯 **Business Logic**
- ✅ **Categories** - Creation, management, deactivation
- ✅ **Campaigns** - Full crowdfunding campaign lifecycle
- ✅ **Products** - Adding, purchasing, management
- ✅ **Payments** - ETH integration with secure transactions
- ✅ **Permissions** - Owner and administrator system
- ✅ **Fund Management** - Automatic and manual fund withdrawal

#### 🔧 **Contract Functions**
```rust
// Categories
pub fn create_category(&mut self) -> U256
pub fn get_category_is_active(&self, id: U256) -> bool
pub fn deactivate_category(&mut self, id: U256)

// Campaigns  
pub fn create_campaign(&mut self, category_id: U256, max_sold_products: U256, destination_wallet: Address) -> U256
pub fn get_campaign_data(&self, id: U256) -> (U256, Address, Address, U256, U256, bool)
pub fn deactivate_campaign(&mut self, id: U256)

// Products
pub fn add_product(&mut self, campaign_id: U256, category_id: U256, price: U256) -> U256
pub fn purchase_product(&mut self, id: U256) -> Result<(), Vec<u8>>
pub fn get_product_owner(&self, id: U256) -> Address
pub fn deactivate_product(&mut self, id: U256)

// Fund Management
pub fn withdraw_campaign_funds(&mut self, campaign_id: U256) -> Result<(), Vec<u8>>
```

## 🌐 **Contract Access**

### **Arbiscan Explorer**
```
https://sepolia.arbiscan.io/address/0xb4e32dfc1c792424f57506a5113d40aae5fbc437
```

### **ABI Interface**
Complete ABI available in `abi.sol` file - ready to use with:
- Web3.js
- Ethers.js  
- Wagmi (React)
- Viem (TypeScript)

## 🛠️ **Technologies**

- **Language:** Rust
- **Platform:** Arbitrum Stylus
- **SDK:** stylus-sdk-rs v0.9.0
- **Compilation:** WebAssembly (WASM)
- **Storage:** sol_storage! macro
- **Network:** Arbitrum Sepolia Testnet

## 📊 **Deployment Statistics**

- **Contract Address:** `0xb4e32dfc1c792424f57506a5113d40aae5fbc437`
- **Network:** Arbitrum Sepolia Testnet
- **Deployment Cost:** Optimized for minimal gas usage
- **Status:** Active and fully functional

## 🚀 **Quick Start**

### **Prerequisites**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stylus CLI
cargo install --force cargo-stylus cargo-stylus-check
```

### **Compilation**
```bash
# Check compilation
cargo stylus check

# Export ABI
cargo stylus export-abi

# Run tests
cargo test
```

### **Deployment** (already completed)
```bash
# Deploy to Arbitrum Sepolia
cargo stylus deploy \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key-path deployer.key \
  --no-verify

# Cache contract for cheaper calls
cargo stylus cache bid \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key-path deployer.key \
  0xb4e32dfc1c792424f57506a5113d40aae5fbc437 0
```

## 🔗 **Contract Interaction**

### **Example Usage with Web3.js**
```javascript
const contractAddress = "0xb4e32dfc1c792424f57506a5113d40aae5fbc437";
const abi = [/* ABI from abi.sol */];

// Connect to contract
const contract = new web3.eth.Contract(abi, contractAddress);

// Create category (owner only)
await contract.methods.createCategory().send({from: ownerAddress});

// Create campaign (owner only)
await contract.methods.createCampaign(
  categoryId,
  maxSoldProducts,
  destinationWallet
).send({from: ownerAddress});

// Add product (any user)
await contract.methods.addProduct(
  campaignId,
  categoryId,
  priceInWei
).send({from: userAddress});

// Purchase product
await contract.methods.purchaseProduct(productId).send({
  from: userAddress,
  value: productPriceInWei
});

// Withdraw campaign funds (owner only)
await contract.methods.withdrawCampaignFunds(campaignId).send({from: ownerAddress});
```

### **Data Storage Architecture**
The contract uses a hybrid approach:
- **On-chain**: Critical data (prices, ownership, transaction state)
- **Off-chain (Firebase)**: Metadata (names, descriptions, images)

This design optimizes gas costs while maintaining security and functionality.

## 📁 **Project Structure**

```
donly-contract/
├── src/
│   ├── lib.rs          # Main smart contract code
│   └── main.rs         # Entry point
├── abi.sol             # Solidity ABI interface
├── abi.json            # JSON ABI
├── deployer.key        # Private key for deployment
└── Cargo.toml          # Rust dependencies
```

## 🎯 **Next Steps**

- [x] **Contract Development** - Core functionality implemented
- [x] **Deployment** - Successfully deployed to Arbitrum Sepolia
- [x] **Frontend Integration** - React/Next.js application
- [x] **Firebase Integration** - Metadata storage system
- [x] **Testing** - Comprehensive testing completed
- [ ] **Mainnet Deployment** - When project is production-ready
- [ ] **Advanced Features** - Enhanced permission system

## 📄 **License**

This project is fully open source under the GPL-3.0-or-later license.

---

**🎉 The Donly smart contract is fully functional and ready for use on Arbitrum Sepolia!**