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
- [ ] Integracja frontend-backend
- [ ] Testy i optymalizacja


