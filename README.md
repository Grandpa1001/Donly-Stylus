# Donly - Sustainable Fashion Crowdfunding Platform

A decentralized crowdfunding platform built on Arbitrum Stylus using Rust, focused on sustainable fashion and responsible liquidation of end-of-season collections.

## 🌟 Project Overview

Donly is a blockchain-based platform where fashion brands can liquidate their surplus inventory responsibly, turning end-of-season collections into support for NGOs driving sustainable change. The platform ensures transparent fund distribution through blockchain technology.

## 📁 Project Structure

```
Donly-Stylus/
├── donly-contract/     # Smart contract in Rust (Arbitrum Stylus)
├── donly-frontend/     # Frontend in React/Next.js with Firebase integration
├── deployment-config.json
├── deployer.key
└── README.md
```

## ✨ Core Features

### 🛍️ **Marketplace**
- Browse active products from various campaigns
- Filter products by campaign and search functionality
- Real-time price display with proper ETH formatting
- Secure wallet-based purchasing system
- Product images with fallback support

### 👤 **User Profile System**
- **Active Sales**: Manage products currently for sale
- **Purchased**: View transaction history of bought products
- **Inactive**: Track sold or expired products
- Add new products with campaign selection
- Wallet address display and management

### 🏷️ **Dynamic Category System**
- Flexible product categorization
- Firebase-backed category names and descriptions
- Real-time category management

### 🎯 **Campaign Management**
- Create and manage crowdfunding campaigns
- Set maximum product limits per campaign
- Track sold vs. available products
- Automatic campaign completion detection
- Firebase integration for campaign metadata

### 💰 **Payment & Fund Management**
- Secure ETH-based transactions
- Automatic fund collection per campaign
- Admin-controlled fund withdrawal
- Transparent payment tracking
- Gas-optimized transactions

### 🔐 **Security & Permissions**
- Wallet-based authentication
- Owner-only administrative functions
- Product ownership verification
- Secure fund withdrawal mechanisms

## 🛠️ Technology Stack

### **Smart Contract (Backend)**
- **Rust** - Systems programming language for high performance
- **Arbitrum Stylus** - WebAssembly-based execution environment
- **Stylus SDK** - Development kit for Arbitrum Stylus contracts
- **Cargo** - Rust package manager and build system

### **Frontend Application**
- **Next.js 14** - React framework with App Router
- **TypeScript** - Static type checking for JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Wagmi** - React hooks for Ethereum interaction
- **RainbowKit** - Wallet connection UI components
- **Viem** - TypeScript interface for Ethereum

### **Database & Storage**
- **Firebase Firestore** - NoSQL database for metadata storage
- **Firebase Admin SDK** - Server-side Firebase operations
- **Firebase Client SDK** - Client-side Firebase integration

### **Blockchain Integration**
- **Arbitrum Sepolia** - Layer 2 testnet for development
- **MetaMask/WalletConnect** - Wallet connectivity
- **Ethers.js/Viem** - Blockchain interaction libraries

### **Development Tools**
- **ESLint** - Code linting and formatting
- **Prettier** - Code formatting
- **TypeScript** - Type safety and IntelliSense

## 🚀 Getting Started

### **Prerequisites**
- Node.js 18+ and npm
- Rust and Cargo
- Arbitrum Stylus CLI
- MetaMask or compatible Web3 wallet

### **Smart Contract Development**
```bash
# Navigate to contract directory
cd donly-contract

# Install Stylus CLI (if not installed)
cargo install --force cargo-stylus

# Build the contract
cargo stylus build

# Run tests
cargo stylus test

# Check contract validity
cargo stylus check

# Deploy to Arbitrum Sepolia (requires private key)
cargo stylus deploy --endpoint https://sepolia-rollup.arbitrum.io/rpc --private-key YOUR_PRIVATE_KEY
```

### **Frontend Development**
```bash
# Navigate to frontend directory
cd donly-frontend

# Install dependencies
npm install

# Set up environment variables
cp .env.example .env.local
# Edit .env.local with your configuration

# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

### **Environment Configuration**
Create `.env.local` in the frontend directory:
```env
NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS=0xYourContractAddress
NEXT_PUBLIC_ARBITRUM_SEPOLIA_RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
NEXT_PUBLIC_FIREBASE_API_KEY=your_firebase_api_key
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=your_project.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=your_project_id
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=your_project.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=your_sender_id
NEXT_PUBLIC_FIREBASE_APP_ID=your_app_id
```

## 📈 Development Status

### **Completed Features**
- [x] Arbitrum Stylus smart contract development
- [x] React/Next.js frontend setup with TypeScript
- [x] Smart contract data structures and business logic
- [x] Frontend-backend integration with Wagmi/Viem
- [x] Firebase integration for metadata storage
- [x] User authentication with wallet connection
- [x] Product marketplace with filtering and search
- [x] User profile system with product management
- [x] Secure payment and fund withdrawal system
- [x] Deployment to Arbitrum Sepolia testnet
- [x] Comprehensive testing and optimization
- [x] English localization of the entire frontend

### **Current Architecture**
- **Hybrid Storage**: Blockchain for critical data, Firebase for metadata
- **Gas Optimization**: Minimal on-chain storage, efficient batch operations
- **User Experience**: Responsive design, real-time updates, wallet integration
- **Security**: Owner-based permissions, secure fund management

## 🌐 **Live Deployment**

The smart contract is deployed on **Arbitrum Sepolia Testnet**:

- **Contract Address**: `0xb4e32dfc1c792424f57506a5113d40aae5fbc437`
- **Network**: Arbitrum Sepolia (Chain ID: 421614)
- **Explorer**: [Arbiscan Sepolia](https://sepolia.arbiscan.io/address/0xb4e32dfc1c792424f57506a5113d40aae5fbc437)
- **RPC URL**: `https://sepolia-rollup.arbitrum.io/rpc`

### **Contract Capabilities**
- ✅ Dynamic categories, campaigns, and products
- ✅ Secure payment and purchasing system
- ✅ Automatic fund withdrawal upon campaign completion
- ✅ Manual fund withdrawal by campaign admin
- ✅ Permission validation and security measures
- ✅ Gas-optimized operations
- ✅ Product ownership tracking

## 🔐 **Permission System**

### **Current Permissions**
- **Category Creation**: Contract owner only (deployer)
- **Campaign Creation**: Contract owner only (deployer)
- **Fund Withdrawal**: Contract owner only (deployer)
- **Product Addition**: Any user
- **Product Purchase**: Any user

### **🚧 Future Development Roadmap**

#### **Phase 1: Enhanced Permission System**
- Add `onlyOwner` modifiers to category and campaign creation functions
- Implement permission controls for fund withdrawal
- Contract ownership transfer capability

#### **Phase 2: Token-Based Permissions**
- Create platform token (e.g., DONLY token)
- Product addition permissions for token holders only
- Staking system for additional permissions
- DAO governance for platform management

#### **Phase 3: Advanced Role System**
- **Category Admins**: Manage specific categories
- **Moderators**: Verify campaigns and products
- **Campaign Creators**: Extended permissions for managing their campaigns
- **Partners**: Special permissions for external partners

#### **Phase 4: Security Mechanisms**
- Multi-sig wallet for critical operations
- Time-lock for contract changes
- Emergency pause functionality
- Upgradeable contract pattern

### **📋 Development Phases**

1. **Phase 1** (Current): Basic contract owner permissions
2. **Phase 2**: `onlyOwner` modifier implementation
3. **Phase 3**: Token-based permission system
4. **Phase 4**: DAO governance and advanced roles
5. **Phase 5**: Multi-sig and security mechanisms

## 🏗️ **Smart Contract Architecture**

### **Core Data Structures**
```rust
// Categories for product classification
sol_storage! {
    mapping(U256 => bool) category_is_active;
    U256 category_count;
}

// Campaigns for crowdfunding
sol_storage! {
    mapping(U256 => U256) campaign_category_id;
    mapping(U256 => Address) campaign_admin;
    mapping(U256 => Address) campaign_destination_wallet;
    mapping(U256 => U256) campaign_max_sold_products;
    mapping(U256 => U256) campaign_sold_products_count;
    mapping(U256 => bool) campaign_is_active;
    U256 campaign_count;
}

// Products within campaigns
sol_storage! {
    mapping(U256 => U256) product_campaign_id;
    mapping(U256 => U256) product_price;
    mapping(U256 => bool) product_is_active;
    mapping(U256 => bool) product_is_sold;
    mapping(U256 => Address) product_owner;
    U256 product_count;
}
```

### **Key Functions**
- `create_category()` - Create new product categories
- `create_campaign(category_id, max_products, destination_wallet)` - Create crowdfunding campaigns
- `add_product(campaign_id, category_id, price)` - Add products to campaigns
- `purchase_product(product_id)` - Purchase products with ETH
- `withdraw_campaign_funds(campaign_id)` - Withdraw collected funds

## 📄 **License**

This project is fully open source under the GPL-3.0-or-later license.

See the [LICENSE](LICENSE) file for details.

## 🤝 **Contributing**

Contributions are welcome! Please feel free to submit a Pull Request.

## 📞 **Support**

For questions and support, please open an issue on GitHub.

