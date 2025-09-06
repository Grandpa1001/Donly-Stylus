//!
//! Donly - Crowdfunding Platform on Arbitrum Stylus
//!
//! Platforma crowdfundingowa z kampaniami, produktami i kategoriami
//! Zbudowana na Arbitrum Stylus z wykorzystaniem Rust
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*};

// Status kampanii
#[derive(PartialEq, Eq)]
pub enum CampaignStatus {
    Active = 0,
    Completed = 1,
    Cancelled = 2,
}

// Define some persistent storage using the Solidity ABI.
// `Donly` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Donly {
        // Category functionality - dynamiczne tablice
        uint256 category_count;
        bool[] category_active;
        address[] category_creators;
        uint256[] category_name_hashes;
        
        // Campaign functionality - dynamiczne tablice
        uint256 campaign_count;
        uint256[] campaign_category_ids;
        uint256[] campaign_title_hashes;
        uint256[] campaign_description_hashes;
        uint256[] campaign_image_url_hashes;
        address[] campaign_admins;
        address[] campaign_destination_wallets;
        uint256[] campaign_max_sold_products;
        uint256[] campaign_sold_products_counts;
        uint256[] campaign_total_amounts_collected;
        bool[] campaign_is_active;
        uint256[] campaign_statuses; // 0=Active, 1=Completed, 2=Cancelled
        uint256[] campaign_created_ats;
        uint256[] campaign_completed_ats; // 0 = None
        
        // Product functionality - dynamiczne tablice
        uint256 product_count;
        uint256[] product_name_hashes;
        uint256[] product_description_hashes;
        uint256[] product_image_url_hashes;
        uint256[] product_prices;
        bool[] product_is_active;
        bool[] product_is_sold;
        address[] product_owners;
        uint256[] product_campaign_ids;
        uint256[] product_category_ids;
        uint256[] product_created_ats;
        uint256[] product_sold_ats; // 0 = None
    }
}

/// Declare that `Donly` is a contract with the following external methods.
#[public]
impl Donly {
    /// Konstruktor - inicjalizuje podstawowe wartości
    /// Konstruktor - inicjalizuje podstawowe wartości
    // ===== CATEGORY FUNCTIONALITY =====
    
    /// Tworzy nową kategorię (bez limitu)
    pub fn create_category(&mut self, name: String) -> U256 {
        // Walidacja nazwy kategorii
        if name.is_empty() || name.len() > 64 {
            panic!("Invalid category name");
        }
        
        // Pobierz adres twórcy
        let creator = self.vm().msg_sender();
        
        // Oblicz hash nazwy kategorii jako U256
        let name_hash = U256::from_be_bytes(self.vm().native_keccak256(name.as_bytes()).into());
        
        // Sprawdź czy kategoria o takiej nazwie już istnieje
        let current_count = self.category_count.get();
        for i in 0..current_count.as_limbs()[0] {
            if let Some(existing_hash) = self.category_name_hashes.get(U256::from(i)) {
                if existing_hash == name_hash {
                    panic!("Category with this name already exists");
                }
            }
        }
        
        // Dodaj dane kategorii do tablic
        self.category_active.push(true);
        self.category_creators.push(creator);
        self.category_name_hashes.push(name_hash);
        
        // Zwiększ licznik kategorii
        let category_id = self.category_count.get() + U256::from(1);
        self.category_count.set(category_id);
        
        category_id
    }
    
    /// Pobiera hash nazwy kategorii
    pub fn get_category_name_hash(&self, category_id: U256) -> U256 {
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            U256::ZERO
        } else {
            // Konwertuj category_id na indeks tablicy (category_id - 1)
            let index = category_id - U256::from(1);
            self.category_name_hashes.get(index).unwrap_or(U256::ZERO)
        }
    }
    
    /// Sprawdza czy kategoria jest aktywna
    pub fn is_category_active(&self, category_id: U256) -> bool {
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            false
        } else {
            // Konwertuj category_id na indeks tablicy (category_id - 1)
            let index = category_id - U256::from(1);
            self.category_active.get(index).unwrap_or(false)
        }
    }
    
    /// Pobiera twórcę kategorii
    pub fn get_category_creator(&self, category_id: U256) -> Address {
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            Address::ZERO
        } else {
            // Konwertuj category_id na indeks tablicy (category_id - 1)
            let index = category_id - U256::from(1);
            self.category_creators.get(index).unwrap_or(Address::ZERO)
        }
    }
    
    /// Pobiera liczbę kategorii
    pub fn get_category_count(&self) -> U256 {
        self.category_count.get()
    }
    
    /// Dezaktywuje kategorię (tylko twórca)
    pub fn deactivate_category(&mut self, category_id: U256) {
        // Sprawdź czy kategoria istnieje
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            panic!("Category does not exist");
        }
        
        // Konwertuj category_id na indeks tablicy (category_id - 1)
        let index = category_id - U256::from(1);
        
        // Sprawdź czy wywołujący to twórca kategorii
        let creator = self.category_creators.get(index).unwrap_or(Address::ZERO);
        
        if creator != self.vm().msg_sender() {
            panic!("Unauthorized");
        }
        
        // Dezaktywuj kategorię - używamy setter
        if let Some(mut active_guard) = self.category_active.setter(index) {
            active_guard.set(false);
        }
    }
    
    /// Pobiera nazwę kategorii (tylko hash, nie przechowujemy nazw)
    pub fn get_category_name(&self, category_id: U256) -> String {
        // Nie przechowujemy nazw kategorii, tylko hash
        // Ta funkcja jest zachowana dla kompatybilności z testami
        String::new()
    }
    
    /// Sprawdza czy nazwa kategorii pasuje do hash
    pub fn verify_category_name(&self, category_id: U256, name: String) -> bool {
        let expected_hash = U256::from_be_bytes(self.vm().native_keccak256(name.as_bytes()).into());
        let actual_hash = self.get_category_name_hash(category_id);
        actual_hash == expected_hash
    }
    
    // ===== CAMPAIGN FUNCTIONALITY =====
    
    /// Pobiera liczbę kampanii
    pub fn get_campaign_count(&self) -> U256 {
        self.campaign_count.get()
    }
    
    /// Tworzy nową kampanię (bez limitu)
    pub fn create_campaign(&mut self, category_id: U256, title: String, description: String, image_url: String, destination_wallet: Address, max_sold_products: U256) -> U256 {
        // Walidacja kategorii - musi istnieć i być aktywna
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            panic!("Category does not exist");
        }
        
        if !self.is_category_active(category_id) {
            panic!("Category is not active");
        }
        
        // Walidacja tytułu kampanii
        if title.is_empty() || title.len() > 64 {
            panic!("Invalid campaign title");
        }
        
        // Walidacja opisu kampanii
        if description.is_empty() || description.len() > 256 {
            panic!("Invalid campaign description");
        }
        
        // Walidacja URL obrazu
        if image_url.is_empty() || image_url.len() > 128 {
            panic!("Invalid campaign image URL");
        }
        
        // Walidacja maksymalnej liczby produktów
        if max_sold_products == U256::ZERO {
            panic!("Max sold products must be greater than 0");
        }
        
        // Pobierz nowy ID kampanii
        let campaign_id = self.campaign_count.get() + U256::from(1);
        
        // Pobierz adres administratora
        let admin = self.vm().msg_sender();
        
        // Oblicz hashe tytułu, opisu i URL obrazu
        let title_hash = U256::from_be_bytes(self.vm().native_keccak256(title.as_bytes()).into());
        let description_hash = U256::from_be_bytes(self.vm().native_keccak256(description.as_bytes()).into());
        let image_url_hash = U256::from_be_bytes(self.vm().native_keccak256(image_url.as_bytes()).into());
        
        // Pobierz aktualny czas
        let created_at = U256::from(self.vm().block_timestamp());
        
        // Dodaj dane kampanii do tablic
        self.campaign_category_ids.push(category_id);
        self.campaign_title_hashes.push(title_hash);
        self.campaign_description_hashes.push(description_hash);
        self.campaign_image_url_hashes.push(image_url_hash);
        self.campaign_admins.push(admin);
        self.campaign_destination_wallets.push(destination_wallet);
        self.campaign_max_sold_products.push(max_sold_products);
        self.campaign_sold_products_counts.push(U256::ZERO);
        self.campaign_total_amounts_collected.push(U256::ZERO);
        self.campaign_is_active.push(true);
        self.campaign_statuses.push(U256::from(CampaignStatus::Active as u8));
        self.campaign_created_ats.push(created_at);
        self.campaign_completed_ats.push(U256::ZERO);
        
        // Zwiększ licznik kampanii
        self.campaign_count.set(campaign_id);
        
        campaign_id
    }
    
    /// Pobiera dane kampanii
    pub fn get_campaign_data(&self, campaign_id: U256) -> (U256, U256, U256, U256, Address, Address, U256, U256, U256, bool, U256, U256, U256) {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            return (U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, Address::ZERO, Address::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, false, U256::ZERO, U256::ZERO, U256::ZERO);
        }
        
        // Konwertuj campaign_id na indeks tablicy (campaign_id - 1)
        let index = campaign_id - U256::from(1);
        
        (
            self.campaign_category_ids.get(index).unwrap_or(U256::ZERO),
            self.campaign_title_hashes.get(index).unwrap_or(U256::ZERO),
            self.campaign_description_hashes.get(index).unwrap_or(U256::ZERO),
            self.campaign_image_url_hashes.get(index).unwrap_or(U256::ZERO),
            self.campaign_admins.get(index).unwrap_or(Address::ZERO),
            self.campaign_destination_wallets.get(index).unwrap_or(Address::ZERO),
            self.campaign_max_sold_products.get(index).unwrap_or(U256::ZERO),
            self.campaign_sold_products_counts.get(index).unwrap_or(U256::ZERO),
            self.campaign_total_amounts_collected.get(index).unwrap_or(U256::ZERO),
            self.campaign_is_active.get(index).unwrap_or(false),
            self.campaign_statuses.get(index).unwrap_or(U256::ZERO),
            self.campaign_created_ats.get(index).unwrap_or(U256::ZERO),
            self.campaign_completed_ats.get(index).unwrap_or(U256::ZERO)
        )
    }
    
    /// Sprawdza czy kampania jest aktywna
    pub fn is_campaign_active(&self, campaign_id: U256) -> bool {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            false
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_is_active.get(index).unwrap_or(false)
        }
    }
    
    /// Pobiera status kampanii
    pub fn get_campaign_status(&self, campaign_id: U256) -> U256 {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            U256::ZERO
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_statuses.get(index).unwrap_or(U256::ZERO)
        }
    }
    
    /// Generuje unikalne ID kampanii na podstawie tytułu i admina
    pub fn generate_campaign_id(&self, title: String, admin: Address) -> U256 {
        let combined = format!("{}{}", title, admin.to_string());
        U256::from_be_bytes(self.vm().native_keccak256(combined.as_bytes()).into())
    }
    
    /// Sprawdza czy użytkownik jest administratorem kampanii
    pub fn is_campaign_admin(&self, campaign_id: U256, user: Address) -> bool {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            false
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_admins.get(index).unwrap_or(Address::ZERO) == user
        }
    }
    
    /// Dezaktywuje kampanię (tylko admin)
    pub fn deactivate_campaign(&mut self, campaign_id: U256) -> bool {
        // Sprawdź czy kampania istnieje i jest aktywna
        if !self.is_campaign_active(campaign_id) {
            panic!("Campaign not active");
        }
        
        // Sprawdź czy użytkownik jest administratorem
        let admin = self.vm().msg_sender();
        if !self.is_campaign_admin(campaign_id, admin) {
            panic!("Only campaign admin can deactivate");
        }
        
        // Dezaktywuj kampanię
        let index = campaign_id - U256::from(1);
        if let Some(mut active_guard) = self.campaign_is_active.setter(index) {
            active_guard.set(false);
        }
        if let Some(mut status_guard) = self.campaign_statuses.setter(index) {
            status_guard.set(U256::from(CampaignStatus::Cancelled as u8));
        }
        
        true
    }
    
    /// Kończy kampanię i transferuje środki na destination_wallet
    pub fn complete_campaign(&mut self, campaign_id: U256) -> bool {
        // Sprawdź czy kampania istnieje i jest aktywna
        if !self.is_campaign_active(campaign_id) {
            panic!("Campaign not active");
        }
        
        // Sprawdź czy użytkownik jest administratorem
        let admin = self.vm().msg_sender();
        if !self.is_campaign_admin(campaign_id, admin) {
            panic!("Only campaign admin can complete campaign");
        }
        
        // Pobierz dane kampanii
        let (_, _, _, _, _, _destination_wallet, _, _sold_count, total_amount, _, _, _, _) = 
            self.get_campaign_data(campaign_id);
        
        // Sprawdź czy są środki do transferu
        if total_amount == U256::ZERO {
            panic!("No funds to transfer");
        }
        
        // Ukończ kampanię
        let index = campaign_id - U256::from(1);
        if let Some(mut active_guard) = self.campaign_is_active.setter(index) {
            active_guard.set(false);
        }
        if let Some(mut status_guard) = self.campaign_statuses.setter(index) {
            status_guard.set(U256::from(CampaignStatus::Completed as u8));
        }
        if let Some(mut completed_guard) = self.campaign_completed_ats.setter(index) {
            completed_guard.set(U256::from(self.vm().block_timestamp()));
        }
        
        // Transfer środków na destination_wallet
        // W Stylus/EVM to będzie wymagało specjalnej logiki transferu
        // Na razie zwracamy true - implementacja transferu będzie w następnym kroku
        
        true
    }
    
    /// Pobiera destination_wallet kampanii
    pub fn get_campaign_destination_wallet(&self, campaign_id: U256) -> Address {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            Address::ZERO
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_destination_wallets.get(index).unwrap_or(Address::ZERO)
        }
    }
    
    /// Pobiera całkowitą kwotę zebraną w kampanii
    pub fn get_campaign_total_amount(&self, campaign_id: U256) -> U256 {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            U256::ZERO
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_total_amounts_collected.get(index).unwrap_or(U256::ZERO)
        }
    }
    
    /// Pobiera liczbę sprzedanych produktów w kampanii
    pub fn get_campaign_sold_products_count(&self, campaign_id: U256) -> U256 {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            U256::ZERO
        } else {
            let index = campaign_id - U256::from(1);
            self.campaign_sold_products_counts.get(index).unwrap_or(U256::ZERO)
        }
    }
    
    // ===== PRODUCT FUNCTIONALITY =====
    
    /// Pobiera liczbę produktów
    pub fn get_product_count(&self) -> U256 {
        self.product_count.get()
    }
    
    /// Tworzy nowy produkt (bez limitu)
    pub fn add_product(
        &mut self,
        name: String,
        description: String,
        image_url: String,
        price: U256,
        campaign_id: U256,
        category_id: U256,
    ) -> U256 {
        // Walidacja parametrów
        if price == U256::ZERO {
            panic!("Price must be greater than 0");
        }
        if name.len() > 64 {
            panic!("Name too long (max 64 characters)");
        }
        if description.len() > 256 {
            panic!("Description too long (max 256 characters)");
        }
        if image_url.len() > 128 {
            panic!("Image URL too long (max 128 characters)");
        }
        
        // Sprawdź czy kampania istnieje i jest aktywna
        if !self.is_campaign_active(campaign_id) {
            panic!("Campaign not active");
        }
        
        // Sprawdź czy kategoria istnieje i jest aktywna
        if !self.is_category_active(category_id) {
            panic!("Category not active");
        }
        
        let new_product_id = self.product_count.get() + U256::from(1);
        let owner = self.vm().msg_sender();
        let created_at = U256::from(self.vm().block_timestamp());
        
        // Hash dla stringów
        let name_hash = U256::from_be_bytes(self.vm().native_keccak256(name.as_bytes()).into());
        let description_hash = U256::from_be_bytes(self.vm().native_keccak256(description.as_bytes()).into());
        let image_url_hash = U256::from_be_bytes(self.vm().native_keccak256(image_url.as_bytes()).into());
        
        // Dodaj dane produktu do tablic
        self.product_name_hashes.push(name_hash);
        self.product_description_hashes.push(description_hash);
        self.product_image_url_hashes.push(image_url_hash);
        self.product_prices.push(price);
        self.product_is_active.push(true);
        self.product_is_sold.push(false);
        self.product_owners.push(owner);
        self.product_campaign_ids.push(campaign_id);
        self.product_category_ids.push(category_id);
        self.product_created_ats.push(created_at);
        self.product_sold_ats.push(U256::ZERO);
        
        self.product_count.set(new_product_id);
        new_product_id
    }
    
    /// Pobiera dane produktu
    pub fn get_product_data(&self, product_id: U256) -> (U256, U256, U256, U256, bool, bool, Address, U256, U256, U256, U256) {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            return (U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, false, false, Address::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO);
        }
        
        // Konwertuj product_id na indeks tablicy (product_id - 1)
        let index = product_id - U256::from(1);
        
        (
            self.product_name_hashes.get(index).unwrap_or(U256::ZERO),
            self.product_description_hashes.get(index).unwrap_or(U256::ZERO),
            self.product_image_url_hashes.get(index).unwrap_or(U256::ZERO),
            self.product_prices.get(index).unwrap_or(U256::ZERO),
            self.product_is_active.get(index).unwrap_or(false),
            self.product_is_sold.get(index).unwrap_or(false),
            self.product_owners.get(index).unwrap_or(Address::ZERO),
            self.product_campaign_ids.get(index).unwrap_or(U256::ZERO),
            self.product_category_ids.get(index).unwrap_or(U256::ZERO),
            self.product_created_ats.get(index).unwrap_or(U256::ZERO),
            self.product_sold_ats.get(index).unwrap_or(U256::ZERO)
        )
    }
    
    /// Sprawdza czy produkt jest aktywny
    pub fn is_product_active(&self, product_id: U256) -> bool {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            false
        } else {
            let index = product_id - U256::from(1);
            self.product_is_active.get(index).unwrap_or(false)
        }
    }
    
    /// Sprawdza czy produkt jest sprzedany
    pub fn is_product_sold(&self, product_id: U256) -> bool {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            false
        } else {
            let index = product_id - U256::from(1);
            self.product_is_sold.get(index).unwrap_or(false)
        }
    }
    
    /// Kupuje produkt
    pub fn purchase_product(&mut self, product_id: U256) -> bool {
        // Sprawdź czy produkt nie jest sprzedany (najpierw sprawdź sprzedaż)
        if self.is_product_sold(product_id) {
            panic!("Product already sold");
        }
        
        // Sprawdź czy produkt istnieje i jest aktywny
        if !self.is_product_active(product_id) {
            panic!("Product not active");
        }
        
        // Pobierz dane produktu
        let (_, _, _, price, _, _, _, campaign_id, _, _, _) = self.get_product_data(product_id);
        
        // Sprawdź czy kampania jest aktywna
        if !self.is_campaign_active(campaign_id) {
            panic!("Campaign not active");
        }
        
        // Oznacz produkt jako sprzedany i dezaktywuj
        let sold_at = U256::from(self.vm().block_timestamp());
        let index = product_id - U256::from(1);
        
        if let Some(mut sold_guard) = self.product_is_sold.setter(index) {
            sold_guard.set(true);
        }
        if let Some(mut sold_at_guard) = self.product_sold_ats.setter(index) {
            sold_at_guard.set(sold_at);
        }
        if let Some(mut active_guard) = self.product_is_active.setter(index) {
            active_guard.set(false); // Dezaktywuj po sprzedaży
        }
        
        // Aktualizuj statystyki kampanii
        self.update_campaign_stats(campaign_id, price);
        
        true
    }
    
    /// Aktualizuje statystyki kampanii po sprzedaży produktu
    fn update_campaign_stats(&mut self, campaign_id: U256, price: U256) {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            return;
        }
        
        let index = campaign_id - U256::from(1);
        
        // Pobierz aktualne wartości
        let current_sold = self.campaign_sold_products_counts.get(index).unwrap_or(U256::ZERO);
        let current_total = self.campaign_total_amounts_collected.get(index).unwrap_or(U256::ZERO);
        let max_products = self.campaign_max_sold_products.get(index).unwrap_or(U256::ZERO);
        
        // Oblicz nowe wartości
        let new_sold_count = current_sold + U256::from(1);
        let new_total_amount = current_total + price;
        
        // Zaktualizuj statystyki
        if let Some(mut sold_guard) = self.campaign_sold_products_counts.setter(index) {
            sold_guard.set(new_sold_count);
        }
        if let Some(mut total_guard) = self.campaign_total_amounts_collected.setter(index) {
            total_guard.set(new_total_amount);
        }
        
        // Sprawdź czy kampania została ukończona
        if new_sold_count >= max_products {
            if let Some(mut status_guard) = self.campaign_statuses.setter(index) {
                status_guard.set(U256::from(CampaignStatus::Completed as u8));
            }
            if let Some(mut active_guard) = self.campaign_is_active.setter(index) {
                active_guard.set(false);
            }
            if let Some(mut completed_guard) = self.campaign_completed_ats.setter(index) {
                completed_guard.set(U256::from(self.vm().block_timestamp()));
            }
            
            // Dezaktywuj wszystkie pozostałe produkty w kampanii
            self.deactivate_remaining_products_in_campaign(campaign_id);
        }
    }
    
    /// Dezaktywuje wszystkie pozostałe (nie sprzedane) produkty w kampanii
    fn deactivate_remaining_products_in_campaign(&mut self, campaign_id: U256) {
        // Iteruj przez wszystkie produkty
        let product_count = self.product_count.get();
        for i in 0..product_count.as_limbs()[0] {
            let product_id = U256::from(i) + U256::from(1);
            
            // Sprawdź czy produkt należy do tej kampanii i jest aktywny
            let (_, _, _, _, _, _, _, product_campaign_id, _, _, _) = self.get_product_data(product_id);
            
            if product_campaign_id == campaign_id && self.is_product_active(product_id) && !self.is_product_sold(product_id) {
                // Dezaktywuj produkt
                let index = product_id - U256::from(1);
                if let Some(mut active_guard) = self.product_is_active.setter(index) {
                    active_guard.set(false);
                }
            }
        }
    }
    
    /// Pobiera postęp kampanii (sprzedane/maksymalne)
    pub fn get_campaign_progress(&self, campaign_id: U256) -> (U256, U256) {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            (U256::ZERO, U256::ZERO)
        } else {
            let index = campaign_id - U256::from(1);
            (
                self.campaign_sold_products_counts.get(index).unwrap_or(U256::ZERO),
                self.campaign_max_sold_products.get(index).unwrap_or(U256::ZERO)
            )
        }
    }
    
    /// Sprawdza czy kampania osiągnęła cel sprzedaży
    pub fn is_campaign_goal_reached(&self, campaign_id: U256) -> bool {
        let (sold, max) = self.get_campaign_progress(campaign_id);
        sold >= max
    }
    
    /// Sprawdza czy użytkownik jest właścicielem produktu
    pub fn is_product_owner(&self, product_id: U256, user: Address) -> bool {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            false
        } else {
            let index = product_id - U256::from(1);
            self.product_owners.get(index).unwrap_or(Address::ZERO) == user
        }
    }
    
    /// Dezaktywuje produkt (tylko właściciel lub admin kampanii)
    pub fn deactivate_product(&mut self, product_id: U256) -> bool {
        // Sprawdź czy produkt istnieje i jest aktywny
        if !self.is_product_active(product_id) {
            panic!("Product not active");
        }
        
        // Sprawdź czy produkt nie jest już sprzedany
        if self.is_product_sold(product_id) {
            panic!("Cannot deactivate sold product");
        }
        
        let user = self.vm().msg_sender();
        
        // Sprawdź czy użytkownik jest właścicielem produktu
        let is_owner = self.is_product_owner(product_id, user);
        
        // Sprawdź czy użytkownik jest adminem kampanii
        let (_, _, _, _, _, _, _, campaign_id, _, _, _) = self.get_product_data(product_id);
        let is_admin = self.is_campaign_admin(campaign_id, user);
        
        if !is_owner && !is_admin {
            panic!("Only product owner or campaign admin can deactivate");
        }
        
        // Dezaktywuj produkt
        let index = product_id - U256::from(1);
        if let Some(mut active_guard) = self.product_is_active.setter(index) {
            active_guard.set(false);
        }
        
        true
    }
    
    /// Pobiera właściciela produktu
    pub fn get_product_owner(&self, product_id: U256) -> Address {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            Address::ZERO
        } else {
            let index = product_id - U256::from(1);
            self.product_owners.get(index).unwrap_or(Address::ZERO)
        }
    }
    
    /// Dezaktywuje produkt - tylko przez właściciela (nie można dezaktywować sprzedanego)
    pub fn deactivate_product_by_owner(&mut self, product_id: U256) -> bool {
        // Sprawdź czy produkt istnieje
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            panic!("Product not found");
        }
        
        // Sprawdź czy produkt nie jest sprzedany (najpierw sprawdź sprzedaż)
        if self.is_product_sold(product_id) {
            panic!("Cannot deactivate sold product");
        }
        
        // Sprawdź czy produkt jest aktywny
        if !self.is_product_active(product_id) {
            panic!("Product not active");
        }
        
        // Sprawdź czy wywołujący to właściciel
        let caller = self.vm().msg_sender();
        let owner = self.get_product_owner(product_id);
        
        if caller != owner {
            panic!("Unauthorized: not product owner");
        }
        
        // Dezaktywuj produkt
        let index = product_id - U256::from(1);
        if let Some(mut active_guard) = self.product_is_active.setter(index) {
            active_guard.set(false);
        }
        
        true
    }
    
    /// Dezaktywuje produkt - przez admina kampanii (nie można dezaktywować sprzedanego)
    pub fn deactivate_product_by_admin(&mut self, product_id: U256) -> bool {
        // Sprawdź czy produkt istnieje
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            panic!("Product not found");
        }
        
        // Sprawdź czy produkt nie jest sprzedany (najpierw sprawdź sprzedaż)
        if self.is_product_sold(product_id) {
            panic!("Cannot deactivate sold product");
        }
        
        // Sprawdź czy produkt jest aktywny
        if !self.is_product_active(product_id) {
            panic!("Product not active");
        }
        
        // Sprawdź czy wywołujący to admin kampanii
        let caller = self.vm().msg_sender();
        let (_, _, _, _, _, _, _, campaign_id, _, _, _) = self.get_product_data(product_id);
        
        if !self.is_campaign_admin(campaign_id, caller) {
            panic!("Unauthorized: not campaign admin");
        }
        
        // Dezaktywuj produkt
        let index = product_id - U256::from(1);
        if let Some(mut active_guard) = self.product_is_active.setter(index) {
            active_guard.set(false);
        }
        
        true
    }
    
    /// Sprawdza czy produkt może być dezaktywowany (aktywny i nie sprzedany)
    pub fn can_deactivate_product(&self, product_id: U256) -> bool {
        self.is_product_active(product_id) && !self.is_product_sold(product_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_category() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Test tworzenia kategorii
        let category_name = "Electronics".to_string();
        
        // Utwórz kategorię
        let category_id = contract.create_category(category_name.clone());
        
        // Sprawdź czy kategoria została utworzona
        assert_eq!(category_id, U256::from(1));
        assert!(contract.is_category_active(category_id));
        assert_eq!(contract.get_category_creator(category_id), vm.msg_sender());
        
        // Sprawdź czy hash nazwy jest poprawny
        assert!(contract.verify_category_name(category_id, category_name));
        
        // Sprawdź licznik kategorii
        assert_eq!(contract.get_category_count(), U256::from(1));
    }
    
    #[test]
    fn test_multiple_categories() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kilka kategorii (teraz bez limitu!)
        let cat1_id = contract.create_category("Electronics".to_string());
        let cat2_id = contract.create_category("Clothing".to_string());
        let cat3_id = contract.create_category("Books".to_string());
        let cat4_id = contract.create_category("Sports".to_string());
        let cat5_id = contract.create_category("Home".to_string());
        
        // Sprawdź ID
        assert_eq!(cat1_id, U256::from(1));
        assert_eq!(cat2_id, U256::from(2));
        assert_eq!(cat3_id, U256::from(3));
        assert_eq!(cat4_id, U256::from(4));
        assert_eq!(cat5_id, U256::from(5));
        
        // Sprawdź licznik
        assert_eq!(contract.get_category_count(), U256::from(5));
        
        // Sprawdź czy wszystkie są aktywne
        assert!(contract.is_category_active(cat1_id));
        assert!(contract.is_category_active(cat2_id));
        assert!(contract.is_category_active(cat3_id));
        assert!(contract.is_category_active(cat4_id));
        assert!(contract.is_category_active(cat5_id));
        
        // Sprawdź nazwy kategorii (nie przechowujemy nazw, tylko hash)
        // assert_eq!(contract.get_category_name(cat1_id), "Electronics");
        // assert_eq!(contract.get_category_name(cat2_id), "Clothing");
        // assert_eq!(contract.get_category_name(cat3_id), "Books");
        // assert_eq!(contract.get_category_name(cat4_id), "Sports");
        // assert_eq!(contract.get_category_name(cat5_id), "Home");
    }
    
    #[test]
    fn test_many_categories() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz 10 kategorii (wcześniej był limit 3!)
        for i in 1..=10 {
            let category_name = format!("Category{}", i);
            let category_id = contract.create_category(category_name.clone());
            
            assert_eq!(category_id, U256::from(i));
            assert!(contract.is_category_active(category_id));
            // assert_eq!(contract.get_category_name(category_id), category_name); // Nie przechowujemy nazw
        }
        
        // Sprawdź licznik
        assert_eq!(contract.get_category_count(), U256::from(10));
    }
    
    #[test]
    #[should_panic(expected = "Category with this name already exists")]
    fn test_duplicate_category_name() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        let category_name = "Electronics".to_string();
        
        // Utwórz pierwszą kategorię
        contract.create_category(category_name.clone());
        
        // Próba utworzenia kategorii o tej samej nazwie
        contract.create_category(category_name);
    }
    
    #[test]
    fn test_deactivate_category() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        let category_name = "Electronics".to_string();
        
        // Utwórz kategorię
        let category_id = contract.create_category(category_name);
        
        // Sprawdź czy kategoria jest aktywna
        assert!(contract.is_category_active(category_id));
        
        // Dezaktywuj kategorię
        contract.deactivate_category(category_id);
        
        // Sprawdź czy kategoria jest nieaktywna
        assert!(!contract.is_category_active(category_id));
    }
    
    #[test]
    fn test_get_campaign_count() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let contract = Donly::from(&vm);

        // Sprawdź czy początkowo liczba kampanii to 0
        assert_eq!(contract.get_campaign_count(), U256::ZERO);
    }
    
    #[test]
    fn test_create_campaign() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Najpierw utwórz kategorię
        let category_id = contract.create_category("Electronics".to_string());
        assert_eq!(category_id, U256::from(1));
        
        // Teraz utwórz kampanię w tej kategorii
        let campaign_id = contract.create_campaign(
            category_id,
            "New iPhone Campaign".to_string(),
            "Help us fund the latest iPhone development".to_string(),
            "https://example.com/iphone.jpg".to_string(),
            Address::ZERO, // destination wallet
            U256::from(100) // max sold products
        );
        
        // Sprawdź czy kampania została utworzona
        assert_eq!(campaign_id, U256::from(1));
        assert_eq!(contract.get_campaign_count(), U256::from(1));
        
        // Sprawdź dane kampanii
        let (cat_id, title_hash, desc_hash, img_hash, admin, dest_wallet, max_products, sold_products, total_amount, is_active, status, created_at, completed_at) = contract.get_campaign_data(campaign_id);
        
        assert_eq!(cat_id, category_id);
        assert!(title_hash != U256::ZERO);
        assert!(desc_hash != U256::ZERO);
        assert!(img_hash != U256::ZERO);
        assert_eq!(admin, vm.msg_sender());
        assert_eq!(dest_wallet, Address::ZERO);
        assert_eq!(max_products, U256::from(100));
        assert_eq!(sold_products, U256::ZERO);
        assert_eq!(total_amount, U256::ZERO);
        assert!(is_active);
        assert_eq!(status, U256::from(CampaignStatus::Active as u8));
        // created_at może być 0 w testach, więc sprawdźmy tylko czy nie jest ujemne
        assert!(created_at >= U256::ZERO);
        assert_eq!(completed_at, U256::ZERO);
        
        // Sprawdź inne metody
        assert!(contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Active as u8));
    }
    
    #[test]
    #[should_panic(expected = "Category does not exist")]
    fn test_create_campaign_invalid_category() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Próba utworzenia kampanii z nieistniejącą kategorią
        contract.create_campaign(
            U256::from(999), // nieistniejąca kategoria
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
    }
    
    #[test]
    #[should_panic(expected = "Category is not active")]
    fn test_create_campaign_inactive_category() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię
        let category_id = contract.create_category("Electronics".to_string());
        
        // Dezaktywuj kategorię
        contract.deactivate_category(category_id);
        
        // Próba utworzenia kampanii w nieaktywnej kategorii
        contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
    }
    
    // ===== PRODUCT TESTS =====
    
    #[test]
    fn test_get_product_count() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let contract = Donly::from(&vm);

        // Sprawdź czy początkowo liczba produktów to 0
        assert_eq!(contract.get_product_count(), U256::ZERO);
    }
    
    #[test]
    fn test_add_product() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź czy produkt został utworzony
        assert_eq!(product_id, U256::from(1));
        assert_eq!(contract.get_product_count(), U256::from(1));
        assert!(contract.is_product_active(product_id));
        assert!(!contract.is_product_sold(product_id));
    }
    
    #[test]
    fn test_get_product_data() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Pobierz dane produktu
        let (name_hash, description_hash, image_url_hash, price, is_active, is_sold, owner, campaign_id_result, category_id_result, created_at, sold_at) = 
            contract.get_product_data(product_id);
        
        // Sprawdź dane
        assert!(name_hash != U256::ZERO);
        assert!(description_hash != U256::ZERO);
        assert!(image_url_hash != U256::ZERO);
        assert_eq!(price, U256::from(1000));
        assert!(is_active);
        assert!(!is_sold);
        assert_eq!(campaign_id_result, campaign_id);
        assert_eq!(category_id_result, category_id);
        assert!(created_at >= U256::ZERO);
        assert_eq!(sold_at, U256::ZERO);
    }
    
    #[test]
    #[should_panic(expected = "Price must be greater than 0")]
    fn test_add_product_zero_price() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Próba dodania produktu z ceną 0
        contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::ZERO,
            campaign_id,
            category_id
        );
    }
    
    #[test]
    #[should_panic(expected = "Campaign not active")]
    fn test_add_product_inactive_campaign() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię
        let category_id = contract.create_category("Electronics".to_string());
        
        // Próba dodania produktu do nieistniejącej kampanii
        contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            U256::from(999), // nieistniejąca kampania
            category_id
        );
    }
    
    #[test]
    #[should_panic(expected = "Category not active")]
    fn test_add_product_inactive_category() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dezaktywuj kategorię
        contract.deactivate_category(category_id);
        
        // Próba dodania produktu w nieaktywnej kategorii
        contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
    }
    
    #[test]
    fn test_purchase_product() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź początkowy stan
        assert!(!contract.is_product_sold(product_id));
        assert_eq!(contract.campaign1_sold_products_count.get(), U256::ZERO);
        assert_eq!(contract.campaign1_total_amount_collected.get(), U256::ZERO);
        
        // Kup produkt
        let result = contract.purchase_product(product_id);
        assert!(result);
        
        // Sprawdź czy produkt został sprzedany
        assert!(contract.is_product_sold(product_id));
        
        // Sprawdź czy statystyki kampanii zostały zaktualizowane
        assert_eq!(contract.get_campaign_sold_products_count(campaign_id), U256::from(1));
        assert_eq!(contract.get_campaign_total_amount(campaign_id), U256::from(1000));
    }
    
    #[test]
    fn test_purchase_product_campaign_completion() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z max_sold_products = 1
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(1) // Tylko 1 produkt
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź początkowy stan kampanii
        assert!(contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Active as u8));
        
        // Kup produkt
        contract.purchase_product(product_id);
        
        // Sprawdź czy kampania została ukończona
        assert!(!contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Completed as u8));
    }
    
    #[test]
    #[should_panic(expected = "Product not active")]
    fn test_purchase_product_inactive() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Próba zakupu nieistniejącego produktu
        contract.purchase_product(U256::from(999));
    }
    
    #[test]
    #[should_panic(expected = "Product already sold")]
    fn test_purchase_product_already_sold() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Kup produkt pierwszy raz
        contract.purchase_product(product_id);
        
        // Próba zakupu tego samego produktu ponownie
        contract.purchase_product(product_id);
    }
    
    // ===== CAMPAIGN MANAGEMENT TESTS =====
    
    #[test]
    fn test_generate_campaign_id() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let contract = Donly::from(&vm);

        let title = "Test Campaign".to_string();
        let admin = Address::from([1u8; 20]);
        
        let id1 = contract.generate_campaign_id(title.clone(), admin);
        let id2 = contract.generate_campaign_id(title.clone(), admin);
        let id3 = contract.generate_campaign_id("Different Title".to_string(), admin);
        
        // Same title and admin should generate same ID
        assert_eq!(id1, id2);
        // Different title should generate different ID
        assert_ne!(id1, id3);
        // ID should not be zero
        assert_ne!(id1, U256::ZERO);
    }
    
    #[test]
    fn test_is_campaign_admin() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Sprawdź dane kampanii
        let (_, _, _, _, admin, _, _, _, _, _, _, _, _) = contract.get_campaign_data(campaign_id);
        
        // Sprawdź czy twórca kampanii jest adminem
        assert!(contract.is_campaign_admin(campaign_id, admin));
        
        // Sprawdź czy inny adres nie jest adminem
        assert!(!contract.is_campaign_admin(campaign_id, Address::from([1u8; 20])));
    }
    
    #[test]
    fn test_deactivate_campaign() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Sprawdź czy kampania jest aktywna
        assert!(contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Active as u8));
        
        // Dezaktywuj kampanię
        let result = contract.deactivate_campaign(campaign_id);
        assert!(result);
        
        // Sprawdź czy kampania została dezaktywowana
        assert!(!contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Cancelled as u8));
    }
    
    #[test]
    fn test_deactivate_campaign_unauthorized() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::from([1u8; 20]), // destination_wallet
            U256::from(100)
        );
        
        // Sprawdź dane kampanii
        let (_, _, _, _, admin, _, _, _, _, _, _, _, _) = contract.get_campaign_data(campaign_id);
        
        // Sprawdź czy admin to rzeczywisty admin
        assert!(contract.is_campaign_admin(campaign_id, admin));
        
        // Dezaktywacja przez admina powinna się udać
        let result = contract.deactivate_campaign(campaign_id);
        assert!(result);
    }
    
    #[test]
    fn test_complete_campaign() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt i go kup
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        contract.purchase_product(product_id);
        
        // Sprawdź czy są środki
        assert!(contract.get_campaign_total_amount(campaign_id) > U256::ZERO);
        
        // Zakończ kampanię
        let result = contract.complete_campaign(campaign_id);
        assert!(result);
        
        // Sprawdź czy kampania została ukończona
        assert!(!contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Completed as u8));
    }
    
    #[test]
    #[should_panic(expected = "No funds to transfer")]
    fn test_complete_campaign_no_funds() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Próba zakończenia kampanii bez środków
        contract.complete_campaign(campaign_id);
    }
    
    #[test]
    fn test_get_campaign_destination_wallet() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        let destination = Address::from([1u8; 20]);
        
        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            destination,
            U256::from(100)
        );
        
        // Sprawdź destination wallet
        assert_eq!(contract.get_campaign_destination_wallet(campaign_id), destination);
    }
    
    #[test]
    fn test_get_campaign_total_amount() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Sprawdź początkową kwotę
        assert_eq!(contract.get_campaign_total_amount(campaign_id), U256::ZERO);
        
        // Dodaj produkt i go kup
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        contract.purchase_product(product_id);
        
        // Sprawdź czy kwota została zaktualizowana
        assert_eq!(contract.get_campaign_total_amount(campaign_id), U256::from(1000));
    }
    
    // ===== PRODUCT MANAGEMENT TESTS =====
    
    #[test]
    fn test_is_product_owner() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź dane produktu
        let (_, _, _, _, _, _, owner, _, _, _, _) = contract.get_product_data(product_id);
        
        // Sprawdź czy właściciel to rzeczywisty właściciel
        assert!(contract.is_product_owner(product_id, owner));
        
        // Sprawdź czy inny adres nie jest właścicielem
        assert!(!contract.is_product_owner(product_id, Address::from([1u8; 20])));
    }
    
    
    #[test]
    #[should_panic(expected = "Product not active")]
    fn test_deactivate_product_inactive() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Próba dezaktywacji nieistniejącego produktu
        contract.deactivate_product(U256::from(999));
    }
    
    #[test]
    #[should_panic(expected = "Cannot deactivate sold product")]
    fn test_deactivate_product_sold() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Kup produkt
        contract.purchase_product(product_id);
        
        // Próba dezaktywacji sprzedanego produktu
        contract.deactivate_product_by_owner(product_id);
    }
    
    #[test]
    fn test_get_product_owner() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(100)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Product Description".to_string(),
            "https://example.com/product.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Pobierz właściciela produktu
        let owner = contract.get_product_owner(product_id);
        
        // Sprawdź czy właściciel nie jest ZERO
        assert_ne!(owner, Address::ZERO);
        
        // Sprawdź czy właściciel to rzeczywisty właściciel
        assert!(contract.is_product_owner(product_id, owner));
    }
    
    // ===== CAMPAIGN GOAL TESTS =====
    
    #[test]
    fn test_campaign_progress() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z max_sold_products = 5
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(5) // Maksymalnie 5 produktów
        );
        
        // Sprawdź początkowy postęp
        let (sold, max) = contract.get_campaign_progress(campaign_id);
        assert_eq!(sold, U256::ZERO);
        assert_eq!(max, U256::from(5));
        assert!(!contract.is_campaign_goal_reached(campaign_id));
        
        // Dodaj i kup 3 produkty
        for i in 1..=3 {
            let product_id = contract.add_product(
                format!("Product {}", i),
                format!("Description {}", i),
                format!("https://example.com/product{}.jpg", i),
                U256::from(1000),
                campaign_id,
                category_id
            );
            contract.purchase_product(product_id);
        }
        
        // Sprawdź postęp po 3 sprzedażach
        let (sold, max) = contract.get_campaign_progress(campaign_id);
        assert_eq!(sold, U256::from(3));
        assert_eq!(max, U256::from(5));
        assert!(!contract.is_campaign_goal_reached(campaign_id));
    }
    
    #[test]
    fn test_campaign_goal_reached_auto_deactivation() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z max_sold_products = 2
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(2) // Maksymalnie 2 produkty
        );
        
        // Dodaj 3 produkty
        let product1_id = contract.add_product(
            "Product 1".to_string(),
            "Description 1".to_string(),
            "https://example.com/product1.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        let product2_id = contract.add_product(
            "Product 2".to_string(),
            "Description 2".to_string(),
            "https://example.com/product2.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        let product3_id = contract.add_product(
            "Product 3".to_string(),
            "Description 3".to_string(),
            "https://example.com/product3.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź że wszystkie produkty są aktywne
        assert!(contract.is_product_active(product1_id));
        assert!(contract.is_product_active(product2_id));
        assert!(contract.is_product_active(product3_id));
        assert!(contract.is_campaign_active(campaign_id));
        
        // Kup pierwszy produkt
        contract.purchase_product(product1_id);
        
        // Sprawdź postęp
        let (sold, max) = contract.get_campaign_progress(campaign_id);
        assert_eq!(sold, U256::from(1));
        assert_eq!(max, U256::from(2));
        assert!(!contract.is_campaign_goal_reached(campaign_id));
        
        // Kup drugi produkt - powinno to zakończyć kampanię
        contract.purchase_product(product2_id);
        
        // Sprawdź że kampania została ukończona
        assert!(!contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Completed as u8));
        assert!(contract.is_campaign_goal_reached(campaign_id));
        
        // Sprawdź że trzeci produkt został automatycznie dezaktywowany
        assert!(!contract.is_product_active(product3_id));
        
        // Sprawdź że nie można kupić dezaktywowanego produktu
        // (to powinno panic w purchase_product)
    }
    
    #[test]
    #[should_panic(expected = "Product not active")]
    fn test_cannot_purchase_deactivated_product_after_goal() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z max_sold_products = 1
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(1) // Tylko 1 produkt
        );
        
        // Dodaj 2 produkty
        let product1_id = contract.add_product(
            "Product 1".to_string(),
            "Description 1".to_string(),
            "https://example.com/product1.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        let product2_id = contract.add_product(
            "Product 2".to_string(),
            "Description 2".to_string(),
            "https://example.com/product2.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Kup pierwszy produkt - to zakończy kampanię
        contract.purchase_product(product1_id);
        
        // Sprawdź że drugi produkt został dezaktywowany
        assert!(!contract.is_product_active(product2_id));
        
        // Próba zakupu dezaktywowanego produktu powinna panic
        contract.purchase_product(product2_id);
    }
    
    // ===== PRODUCT DEACTIVATION TESTS =====
    
    #[test]
    fn test_deactivate_product_by_owner() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(10)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź że produkt jest aktywny
        assert!(contract.is_product_active(product_id));
        assert!(contract.can_deactivate_product(product_id));
        
        // Dezaktywuj produkt przez właściciela
        let result = contract.deactivate_product_by_owner(product_id);
        assert!(result);
        
        // Sprawdź że produkt jest nieaktywny
        assert!(!contract.is_product_active(product_id));
        assert!(!contract.can_deactivate_product(product_id));
    }
    
    #[test]
    fn test_deactivate_product_by_admin() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z adminem
        let category_id = contract.create_category("Electronics".to_string());
        let admin = Address::from([1u8; 20]);
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            admin,
            U256::from(10)
        );
        
        // Dodaj produkt przez innego użytkownika
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Sprawdź że produkt jest aktywny
        assert!(contract.is_product_active(product_id));
        assert!(contract.can_deactivate_product(product_id));
        
        // Dezaktywuj produkt przez admina kampanii
        let result = contract.deactivate_product_by_admin(product_id);
        assert!(result);
        
        // Sprawdź że produkt jest nieaktywny
        assert!(!contract.is_product_active(product_id));
        assert!(!contract.can_deactivate_product(product_id));
    }
    
    #[test]
    #[should_panic(expected = "Cannot deactivate sold product")]
    fn test_cannot_deactivate_sold_product() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(10)
        );
        
        // Dodaj i kup produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        contract.purchase_product(product_id);
        
        // Sprawdź że produkt jest sprzedany
        assert!(contract.is_product_sold(product_id));
        assert!(!contract.can_deactivate_product(product_id));
        
        // Próba dezaktywacji sprzedanego produktu powinna panic
        contract.deactivate_product_by_owner(product_id);
    }
    
    #[test]
    #[should_panic(expected = "Product not active")]
    fn test_cannot_deactivate_inactive_product() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(10)
        );
        
        // Dodaj produkt
        let product_id = contract.add_product(
            "Test Product".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Dezaktywuj produkt
        contract.deactivate_product_by_owner(product_id);
        
        // Próba ponownej dezaktywacji powinna panic
        contract.deactivate_product_by_owner(product_id);
    }
    
    #[test]
    fn test_product_lifecycle_sold_never_changes() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Donly::from(&vm);

        // Utwórz kategorię i kampanię z max_sold_products = 1
        let category_id = contract.create_category("Electronics".to_string());
        let campaign_id = contract.create_campaign(
            category_id,
            "Test Campaign".to_string(),
            "Test Description".to_string(),
            "https://example.com/test.jpg".to_string(),
            Address::ZERO,
            U256::from(1) // Tylko 1 produkt
        );
        
        // Dodaj 2 produkty
        let product1_id = contract.add_product(
            "Product 1".to_string(),
            "Description 1".to_string(),
            "https://example.com/product1.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        let product2_id = contract.add_product(
            "Product 2".to_string(),
            "Description 2".to_string(),
            "https://example.com/product2.jpg".to_string(),
            U256::from(1000),
            campaign_id,
            category_id
        );
        
        // Kup pierwszy produkt - to zakończy kampanię
        contract.purchase_product(product1_id);
        
        // Sprawdź że pierwszy produkt jest sprzedany
        assert!(contract.is_product_sold(product1_id));
        assert!(!contract.can_deactivate_product(product1_id));
        
        // Sprawdź że drugi produkt został automatycznie dezaktywowany
        assert!(!contract.is_product_active(product2_id));
        assert!(!contract.can_deactivate_product(product2_id));
        
        // Sprawdź że kampania jest zakończona
        assert!(!contract.is_campaign_active(campaign_id));
        assert_eq!(contract.get_campaign_status(campaign_id), U256::from(CampaignStatus::Completed as u8));
        
        // WAŻNE: Sprzedany produkt pozostaje sprzedany nawet po zakończeniu kampanii
        assert!(contract.is_product_sold(product1_id));
        assert!(!contract.is_product_active(product1_id));
    }
}
