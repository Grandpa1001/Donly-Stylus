# Donly - Crowdfunding Platform on Arbitrum Stylus

Migracja platformy crowdfundingowej Donly z Solana na Arbitrum Stylus.

## 🏗️ Struktura projektu

```
Donly-Stylus/
├── donly-contract/          # Smart contract (Rust)
├── donly-frontend/          # Frontend (Next.js/React)
└── README.md
```

## 🚀 Smart Contract

### Funkcjonalności:
- ✅ **Kategorie** (3 sloty) - tworzenie, dezaktywacja
- ✅ **Kampanie** (3 sloty) - pełna funkcjonalność z Solana
- ✅ **Status kampanii** - Active, Completed, Cancelled
- ✅ **Walidacja** - kategorie muszą istnieć i być aktywne

### Testy:
```bash
cd donly-contract
cargo test
```

## 🌐 Frontend

### Technologie:
- Next.js 14
- React 18
- TypeScript
- Tailwind CSS
- Web3 (ethers.js, wagmi, viem)

### Uruchomienie:
```bash
cd donly-frontend
npm install
npm run dev
```

## 📋 TODO

- [ ] Dodanie produktów
- [ ] Metody zarządzania kampaniami (complete, cancel)
- [ ] Płatności i zbieranie środków
- [ ] Integracja frontend z smart contractem

## 🔧 Wymagania

- Rust 1.70+
- Node.js 18+
- cargo-stylus
