# Donly - Crowdfunding Platform on Arbitrum Stylus

Platforma crowdfundingowa zbudowana na Arbitrum Stylus z wykorzystaniem Rust.

## Struktura projektu

```
Donly-Stylus/
├── donly-contract/     # Smart contract w Rust (Arbitrum Stylus)
├── donly-frontend/     # Frontend w React/Next.js
└── README.md
```

## Funkcjonalności

- **Kampanie crowdfundingowe** - tworzenie i zarządzanie kampaniami
- **Produkty** - dodawanie produktów do kampanii
- **Kategorie** - klasyfikacja produktów
- **Zakupy** - system płatności i zakupów
- **Zarządzanie** - dezaktywacja i zamykanie kampanii

## Technologie

### Backend (Smart Contract)
- **Rust** - język programowania
- **Arbitrum Stylus** - platforma wykonawcza
- **Stylus SDK** - SDK dla smart kontraktów

### Frontend
- **Next.js 14** - framework React
- **TypeScript** - typowanie statyczne
- **Tailwind CSS** - stylowanie
- **Wagmi** - integracja z Ethereum
- **RainbowKit** - UI dla portfeli
- **React Query** - zarządzanie stanem

## Rozpoczęcie pracy

### Smart Contract
```bash
cd donly-contract
cargo stylus build
cargo stylus test
```

### Frontend
```bash
cd donly-frontend
npm run dev
```

## Status rozwoju

- [x] Utworzenie projektu Stylus
- [x] Konfiguracja frontendu React
- [x] Implementacja struktur danych
- [x] Implementacja logiki biznesowej
- [x] Integracja frontend-backend
- [x] Testy i optymalizacja
- [x] Deployment na Arbitrum Sepolia

## 🚀 **Deployment**

Smart kontrakt został zdeployowany na **Arbitrum Sepolia**:

- **Adres kontraktu**: `0xc2ad3070ff0a301f5df343d889da2a08eacd9792`
- **Sieć**: Arbitrum Sepolia (Chain ID: 421614)
- **Explorer**: [Arbiscan Sepolia](https://sepolia.arbiscan.io/address/0xc2ad3070ff0a301f5df343d889da2a08eacd9792)
- **RPC URL**: `https://sepolia-rollup.arbitrum.io/rpc`

### Funkcjonalności kontraktu:
- ✅ Dynamiczne kategorie, kampanie i produkty
- ✅ System płatności i zakupów
- ✅ Automatyczna wypłata środków po zakończeniu kampanii
- ✅ Ręczna wypłata środków przez admina kampanii
- ✅ Walidacja uprawnień i bezpieczeństwo

## 📄 **Licencja**

Ten projekt jest w pełni open source z licencją GPL-3.0-or-later.

Zobacz plik [LICENSE](LICENSE) dla szczegółów.

