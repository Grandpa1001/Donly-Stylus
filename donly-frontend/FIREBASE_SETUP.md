# Firebase Setup Instructions

## 🔥 **Konfiguracja Firebase dla Donly**

### **1. Pobierz klucze API z Firebase Console**

1. Przejdź do [Firebase Console](https://console.firebase.google.com/)
2. Wybierz projekt `donly-18a81`
3. Kliknij na ikonę ⚙️ (Settings) → Project settings
4. Przewiń w dół do sekcji "Your apps"
5. Jeśli nie masz jeszcze aplikacji webowej, kliknij "Add app" → Web (</>) 
6. Skopiuj konfigurację z sekcji "SDK setup and configuration"

### **2. Zaktualizuj plik .env.local**

Zastąp placeholder'y w pliku `donly-frontend/.env.local`:

```env
# Firebase Configuration
FIREBASE_PROJECT_ID=donly-18a81
FIREBASE_PROJECT_NUMBER=583183369780
FIREBASE_API_KEY=your_actual_api_key_here
FIREBASE_AUTH_DOMAIN=donly-18a81.firebaseapp.com
FIREBASE_STORAGE_BUCKET=donly-18a81.appspot.com
FIREBASE_MESSAGING_SENDER_ID=583183369780
FIREBASE_APP_ID=your_actual_app_id_here
```

**Przykład konfiguracji:**
```env
FIREBASE_API_KEY=AIzaSyBxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
FIREBASE_APP_ID=1:583183369780:web:xxxxxxxxxxxxxxxxxxxxxx
```

### **3. Konfiguracja Firestore**

1. W Firebase Console przejdź do **Firestore Database**
2. Kliknij **Create database**
3. Wybierz **Start in test mode** (dla developmentu)
4. Wybierz lokalizację (np. `europe-west1`)

### **4. Struktura kolekcji**

Firestore automatycznie utworzy następujące kolekcje:

#### **Categories Collection**
```javascript
// Kolekcja: categories
// Dokument: auto-generated-id
{
  name: "Electronics",
  blockchainId: 1,
  createdAt: timestamp,
  updatedAt: timestamp
}
```

#### **Campaigns Collection** (przyszła implementacja)
```javascript
// Kolekcja: campaigns
// Dokument: auto-generated-id
{
  name: "Tech Innovation Fund",
  description: "Supporting innovative technology projects",
  blockchainId: 1,
  createdAt: timestamp,
  updatedAt: timestamp
}
```

#### **Products Collection** (przyszła implementacja)
```javascript
// Kolekcja: products
// Dokument: auto-generated-id
{
  name: "Smartphone",
  description: "Latest generation smartphone",
  blockchainId: 1,
  campaignId: 1,
  categoryId: 1,
  createdAt: timestamp,
  updatedAt: timestamp
}
```

### **5. Reguły bezpieczeństwa Firestore**

Dla developmentu możesz użyć prostych reguł:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /{document=**} {
      allow read, write: if true;
    }
  }
}
```

**⚠️ UWAGA:** Te reguły pozwalają na odczyt i zapis dla wszystkich. W produkcji należy je zaostrzyć!

### **6. Migracja istniejących kategorii**

Po skonfigurowaniu Firebase możesz dodać predefiniowane kategorie:

1. Uruchom frontend: `npm run dev`
2. Przejdź do `/test`
3. W sekcji "Firebase Migration" kliknij **"Migrate Categories to Firebase"**
4. Sprawdź czy kategorie zostały dodane w Firebase Console

**Predefiniowane kategorie:**
- Elektronika
- Zrównoważony rozwój
- Moda
- Dom i ogród
- Sport i rekreacja
- Zdrowie i uroda
- Książki i media
- Motoryzacja
- Żywność i napoje
- Edukacja

### **7. Testowanie integracji**

1. Sprawdź status Firebase w sekcji statystyk
2. Użyj przycisku **"Debug Firebase"** aby sprawdzić konfigurację
3. Stwórz nową kategorię z nazwą
4. Sprawdź czy kategoria pojawiła się w Firebase Console
5. Sprawdź czy dropdowny pokazują nazwy kategorii

### **7. Rozwiązywanie problemów**

#### **Błąd: "Firebase App named '[DEFAULT]' already exists"**
- Sprawdź czy nie masz duplikatów konfiguracji Firebase

#### **Błąd: "Missing or insufficient permissions"**
- Sprawdź reguły Firestore
- Upewnij się, że aplikacja ma odpowiednie uprawnienia

#### **Błąd: "Invalid API key"**
- Sprawdź czy klucz API jest poprawny w `.env.local`
- Upewnij się, że klucz jest aktywny w Firebase Console

### **8. Następne kroki**

- [ ] Dodaj autentykację Firebase
- [ ] Implementuj kolekcje Campaigns i Products
- [ ] Dodaj walidację danych
- [ ] Zaimplementuj cache'owanie
- [ ] Dodaj offline support

---

**🎉 Po skonfigurowaniu Firebase będziesz mógł:**
- Tworzyć kategorie z nazwami
- Wyświetlać nazwy kategorii zamiast ID
- Używać dropdownów z kategoriami
- Oszczędzać miejsce na blockchainie
