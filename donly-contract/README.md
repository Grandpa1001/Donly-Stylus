![Image](./header.png)

# Donly Smart Contract

Smart contract dla platformy crowdfundingowej Donly zbudowany na **Arbitrum Stylus** z wykorzystaniem Rust i [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs). 

## 🚀 **Status Projektu**

### ✅ **Wdrożony na Arbitrum Sepolia Testnet**
- **Adres kontraktu:** `0xba18b6f947d4e2a4770b35ce1dd98a18903f09dc`
- **Sieć:** Arbitrum Sepolia
- **Status:** Aktywny i gotowy do użycia
- **Rozmiar:** 19.4 KiB (optymalny)
- **Cache:** Zbuforowany w ArbOS dla tańszych wywołań

### 📋 **Zrealizowane funkcjonalności**

#### 🏗️ **Architektura**
- ✅ **Dynamiczne struktury** - bez hardkodowanych limitów
- ✅ **Storage optimization** - użycie `sol_storage!` makro
- ✅ **Error handling** - kompletny system błędów
- ✅ **Gas efficiency** - zoptymalizowany kod

#### 🎯 **Funkcjonalności biznesowe**
- ✅ **Kategorie** - tworzenie, zarządzanie, dezaktywacja
- ✅ **Kampanie** - pełny cykl życia kampanii crowdfundingowych
- ✅ **Produkty** - dodawanie, kupowanie, zarządzanie
- ✅ **Płatności** - integracja z ETH
- ✅ **Uprawnienia** - system właścicieli i administratorów
- ✅ **Automatyzacja** - zamykanie kampanii po osiągnięciu celu

#### 🔧 **Funkcje kontraktu**
```solidity
// Kategorie
function createCategory(string calldata name) external returns (uint256);
function getCategoryNameHash(uint256 category_id) external view returns (uint256);
function deactivateCategory(uint256 category_id) external;

// Kampanie  
function createCampaign(...) external returns (uint256);
function getCampaignData(uint256 campaign_id) external view returns (...);
function deactivateCampaign(uint256 campaign_id) external;

// Produkty
function addProduct(...) external returns (uint256);
function purchaseProduct(uint256 product_id) external payable;
function deactivateProduct(uint256 product_id) external;
```

## 🌐 **Dostęp do kontraktu**

### **Arbiscan Explorer**
```
https://sepolia.arbiscan.io/address/0xba18b6f947d4e2a4770b35ce1dd98a18903f09dc
```

### **ABI Interface**
Pełne ABI dostępne w pliku `abi.sol` - gotowe do użycia z:
- Web3.js
- Ethers.js  
- Wagmi (React)
- Viem (TypeScript)

## 🛠️ **Technologie**

- **Język:** Rust
- **Platforma:** Arbitrum Stylus
- **SDK:** stylus-sdk-rs v0.9.0
- **Kompilacja:** WebAssembly (WASM)
- **Storage:** sol_storage! makro
- **Sieć:** Arbitrum Sepolia Testnet

## 📊 **Statystyki deploymentu**

- **Deployment tx:** `0xfc96d08af403304409413cf54377a23ad360573e62ada5dc56393d4d1e95852a`
- **Activation tx:** `0x6ed268a04b7766e37936b70d457d7e74790399a89ec95a4c8eb0b199ff76ffb3`
- **Cache tx:** `0x414ea7be7d3438504b2bd18b8c2791cc93710027809ffdef6152546dc58b90cf`
- **Koszt deploymentu:** ~0.0006 ETH
- **Metadata hash:** `b76bfbc1266157f01ccbdb8f21751db3f6e1717ea2d1dcc3104a9d1ea09b4002`

## 🚀 **Quick Start**

### **Wymagania**
```bash
# Zainstaluj Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dodaj target WASM
rustup target add wasm32-unknown-unknown

# Zainstaluj Stylus CLI
cargo install --force cargo-stylus cargo-stylus-check
```

### **Kompilacja**
```bash
# Sprawdź kompilację
cargo stylus check

# Eksportuj ABI
cargo stylus export-abi

# Uruchom testy
cargo test
```

### **Deployment** (już wykonane)
```bash
# Deploy na Arbitrum Sepolia
cargo stylus deploy \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key-path deployer.key \
  --no-verify

# Cache kontraktu
cargo stylus cache bid \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key-path deployer.key \
  0xba18b6f947d4e2a4770b35ce1dd98a18903f09dc 0
```

## 🔗 **Interakcja z kontraktem**

### **Przykład użycia z Web3.js**
```javascript
const contractAddress = "0xba18b6f947d4e2a4770b35ce1dd98a18903f09dc";
const abi = [/* ABI z abi.sol */];

// Połączenie z kontraktem
const contract = new web3.eth.Contract(abi, contractAddress);

// Tworzenie kategorii
await contract.methods.createCategory("Electronics").send({from: userAddress});

// Tworzenie kampanii
await contract.methods.createCampaign(
  categoryId,
  "Amazing Project",
  "Description...",
  "https://image.url",
  destinationWallet,
  100
).send({from: userAddress});

// Kupowanie produktu
await contract.methods.purchaseProduct(productId).send({
  from: userAddress,
  value: web3.utils.toWei("0.1", "ether")
});
```

## 📁 **Struktura projektu**

```
donly-contract/
├── src/
│   ├── lib.rs          # Główny kod smart contractu
│   └── main.rs         # Entry point
├── abi.sol             # Solidity ABI interface
├── abi.json            # JSON ABI
├── deployer.key        # Klucz prywatny do deploymentu
└── Cargo.toml          # Zależności Rust
```

## 🎯 **Następne kroki**

- [ ] **Weryfikacja na Arbiscan** - dla lepszej widoczności
- [ ] **Frontend development** - interfejs użytkownika
- [ ] **Testowanie funkcji** - kompleksowe testy
- [ ] **Deploy na mainnet** - gdy projekt będzie gotowy
- [ ] **Dokumentacja API** - szczegółowa dokumentacja

## 📄 **Licencja**

Ten projekt jest w pełni open source z licencją Apache-2.0 lub MIT do wyboru.

---

**🎉 Smart contract Donly jest w pełni funkcjonalny i gotowy do użycia na Arbitrum Sepolia!**