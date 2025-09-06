# Donly Frontend

Frontend aplikacji Donly - platformy crowdfundingowej zbudowanej na Arbitrum Stylus.

## 🚀 **Status**

✅ **Zintegrowany z smart contractem** na Arbitrum Sepolia  
✅ **Gotowy do testowania** funkcji kontraktu  
✅ **Responsywny UI** z Tailwind CSS  
✅ **Web3 integration** z RainbowKit i Wagmi  

## 🛠️ **Technologie**

- **Framework:** Next.js 14
- **Styling:** Tailwind CSS
- **Web3:** Wagmi + RainbowKit
- **Blockchain:** Arbitrum Sepolia
- **Contract:** Donly Smart Contract (Stylus)

## 📋 **Funkcjonalności**

### ✅ **Zaimplementowane**
- **Połączenie z portfelem** - RainbowKit integration
- **Odczyt danych** - kategorie, kampanie, produkty
- **Tworzenie kategorii** - interfejs do createCategory
- **Tworzenie kampanii** - interfejs do createCampaign  
- **Dodawanie produktów** - interfejs do addProduct
- **Kupowanie produktów** - interfejs do purchaseProduct
- **Wyświetlanie danych** - szczegóły kategorii, kampanii, produktów

### 🎯 **Dostępne funkcje**
- `createCategory(name)` - Tworzenie kategorii
- `createCampaign(...)` - Tworzenie kampanii
- `addProduct(...)` - Dodawanie produktu
- `purchaseProduct(id)` - Kupowanie produktu
- `categoryCount()` - Liczba kategorii
- `campaignCount()` - Liczba kampanii
- `productCount()` - Liczba produktów

## 🚀 **Quick Start**

### **1. Instalacja zależności**
```bash
cd donly-frontend
npm install
```

### **2. Konfiguracja środowiska**
Stwórz plik `.env.local`:
```env
# Arbitrum Sepolia RPC URL
NEXT_PUBLIC_ARBITRUM_SEPOLIA_RPC_URL=https://sepolia-rollup.arbitrum.io/rpc

# Donly Contract Address (Arbitrum Sepolia)
NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS=0xc2ad3070ff0a301f5df343d889da2a08eacd9792

# WalletConnect Project ID (get from https://cloud.walletconnect.com/)
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=your_project_id_here
```

### **3. Uruchomienie**
```bash
npm run dev
```

Aplikacja będzie dostępna pod adresem: `http://localhost:3000`

## 🔗 **Integracja z Smart Contractem**

### **Adres kontraktu**
```
0xc2ad3070ff0a301f5df343d889da2a08eacd9792
```

### **Sieć**
- **Nazwa:** Arbitrum Sepolia
- **Chain ID:** 421614
- **RPC:** https://sepolia-rollup.arbitrum.io/rpc

### **Przykład użycia**
```typescript
import { useContract } from '../hooks/useContract'

function MyComponent() {
  const { createCategory, createCampaign } = useContract()
  
  const handleCreateCategory = async () => {
    await createCategory("Electronics")
  }
  
  return (
    <button onClick={handleCreateCategory}>
      Create Category
    </button>
  )
}
```

## 📱 **Interfejs użytkownika**

### **Główne sekcje:**
1. **Statystyki** - liczba kategorii, kampanii, produktów
2. **Tworzenie kategorii** - formularz do dodawania kategorii
3. **Tworzenie kampanii** - formularz do tworzenia kampanii
4. **Dodawanie produktów** - formularz do dodawania produktów
5. **Kupowanie produktów** - interfejs do zakupów
6. **Wyświetlanie danych** - szczegóły elementów

### **Funkcje testowe:**
- ✅ **Connect Wallet** - połączenie z portfelem
- ✅ **Read Data** - odczyt danych z blockchain
- ✅ **Write Transactions** - wysyłanie transakcji
- ✅ **Real-time Updates** - automatyczne odświeżanie

## 🧪 **Testowanie**

### **1. Połącz portfel**
- Kliknij "Connect Wallet"
- Wybierz portfel (MetaMask, WalletConnect, etc.)
- Przełącz na sieć Arbitrum Sepolia

### **2. Testuj funkcje**
- **Stwórz kategorię** - wpisz nazwę i kliknij "Create Category"
- **Stwórz kampanię** - wypełnij formularz kampanii
- **Dodaj produkt** - wypełnij formularz produktu
- **Kup produkt** - wybierz ID produktu i kliknij "Purchase"

### **3. Sprawdź dane**
- Wpisz ID w sekcji "Data Display"
- Zobacz szczegóły kategorii, kampanii, produktów
- Sprawdź status i właściwości

## 🔧 **Struktura projektu**

```
src/
├── app/
│   ├── page.tsx          # Główna strona z interfejsem
│   ├── layout.tsx        # Layout z providerami
│   └── globals.css       # Style globalne
├── hooks/
│   └── useContract.ts    # Hooki do interakcji z kontraktem
└── lib/
    ├── contract.ts       # ABI i adres kontraktu
    └── wagmi.ts          # Konfiguracja Wagmi
```

## 🎯 **Następne kroki**

- [ ] **Dodaj więcej funkcji** - dezaktywacja, zarządzanie
- [ ] **Ulepsz UI** - lepszy design, animacje
- [ ] **Dodaj walidację** - sprawdzanie danych wejściowych
- [ ] **Error handling** - obsługa błędów
- [ ] **Loading states** - stany ładowania
- [ ] **Responsive design** - optymalizacja mobilna

## 📄 **Licencja**

Ten projekt jest w pełni open source z licencją Apache-2.0 lub MIT do wyboru.

---

**🎉 Frontend Donly jest gotowy do testowania smart contractu na Arbitrum Sepolia!**