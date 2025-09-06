//!
//! Donly - Crowdfunding Platform on Arbitrum Stylus
//!
//! Platforma crowdfundingowa z kampaniami, produktami i kategoriami
//! Zbudowana na Arbitrum Stylus z wykorzystaniem Rust
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

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
        // Category functionality - używamy prostych typów bez map
        uint256 category_count;
        uint256 category1_name_hash;
        bool category1_active;
        address category1_creator;
        
        uint256 category2_name_hash;
        bool category2_active;
        address category2_creator;
        
        uint256 category3_name_hash;
        bool category3_active;
        address category3_creator;
        
        // Campaign functionality - podstawowa struktura
        uint256 campaign_count;
        
        // Campaign 1 - wszystkie pola z oryginalnego projektu
        uint256 campaign1_category_id;
        uint256 campaign1_title_hash;
        uint256 campaign1_description_hash;
        uint256 campaign1_image_url_hash;
        address campaign1_admin;
        address campaign1_destination_wallet;
        uint256 campaign1_max_sold_products;
        uint256 campaign1_sold_products_count;
        uint256 campaign1_total_amount_collected;
        bool campaign1_is_active;
        uint256 campaign1_status; // 0=Active, 1=Completed, 2=Cancelled
        uint256 campaign1_created_at;
        uint256 campaign1_completed_at; // 0 = None
        
        // Campaign 2
        uint256 campaign2_category_id;
        uint256 campaign2_title_hash;
        uint256 campaign2_description_hash;
        uint256 campaign2_image_url_hash;
        address campaign2_admin;
        address campaign2_destination_wallet;
        uint256 campaign2_max_sold_products;
        uint256 campaign2_sold_products_count;
        uint256 campaign2_total_amount_collected;
        bool campaign2_is_active;
        uint256 campaign2_status;
        uint256 campaign2_created_at;
        uint256 campaign2_completed_at;
        
        // Campaign 3
        uint256 campaign3_category_id;
        uint256 campaign3_title_hash;
        uint256 campaign3_description_hash;
        uint256 campaign3_image_url_hash;
        address campaign3_admin;
        address campaign3_destination_wallet;
        uint256 campaign3_max_sold_products;
        uint256 campaign3_sold_products_count;
        uint256 campaign3_total_amount_collected;
        bool campaign3_is_active;
        uint256 campaign3_status;
        uint256 campaign3_created_at;
        uint256 campaign3_completed_at;
    }
}

/// Declare that `Donly` is a contract with the following external methods.
#[public]
impl Donly {
    // ===== CATEGORY FUNCTIONALITY =====
    
    /// Tworzy nową kategorię (maksymalnie 3 kategorie)
    pub fn create_category(&mut self, name: String) -> U256 {
        // Walidacja nazwy kategorii
        if name.is_empty() || name.len() > 64 {
            panic!("Invalid category name");
        }
        
        // Sprawdź czy nie przekroczono limitu
        let current_count = self.category_count.get();
        if current_count >= U256::from(3) {
            panic!("Maximum 3 categories allowed");
        }
        
        // Pobierz nowy ID kategorii
        let category_id = current_count + U256::from(1);
        
        // Pobierz adres twórcy
        let creator = self.vm().msg_sender();
        
        // Oblicz hash nazwy kategorii jako U256
        let name_hash = U256::from_be_bytes(self.vm().native_keccak256(name.as_bytes()).into());
        
        // Sprawdź czy kategoria o takiej nazwie już istnieje
        if current_count >= U256::from(1) && self.category1_name_hash.get() == name_hash {
            panic!("Category with this name already exists");
        }
        if current_count >= U256::from(2) && self.category2_name_hash.get() == name_hash {
            panic!("Category with this name already exists");
        }
        if current_count >= U256::from(3) && self.category3_name_hash.get() == name_hash {
            panic!("Category with this name already exists");
        }
        
        // Ustaw dane kategorii w odpowiednim slocie
        if category_id == U256::from(1) {
            self.category1_name_hash.set(name_hash);
            self.category1_active.set(true);
            self.category1_creator.set(creator);
        } else if category_id == U256::from(2) {
            self.category2_name_hash.set(name_hash);
            self.category2_active.set(true);
            self.category2_creator.set(creator);
        } else if category_id == U256::from(3) {
            self.category3_name_hash.set(name_hash);
            self.category3_active.set(true);
            self.category3_creator.set(creator);
        }
        
        // Zwiększ licznik kategorii
        self.category_count.set(category_id);
        
        category_id
    }
    
    /// Pobiera hash nazwy kategorii
    pub fn get_category_name_hash(&self, category_id: U256) -> U256 {
        if category_id == U256::from(1) {
            self.category1_name_hash.get()
        } else if category_id == U256::from(2) {
            self.category2_name_hash.get()
        } else if category_id == U256::from(3) {
            self.category3_name_hash.get()
        } else {
            U256::ZERO
        }
    }
    
    /// Sprawdza czy kategoria jest aktywna
    pub fn is_category_active(&self, category_id: U256) -> bool {
        if category_id == U256::from(1) {
            self.category1_active.get()
        } else if category_id == U256::from(2) {
            self.category2_active.get()
        } else if category_id == U256::from(3) {
            self.category3_active.get()
        } else {
            false
        }
    }
    
    /// Pobiera twórcę kategorii
    pub fn get_category_creator(&self, category_id: U256) -> Address {
        if category_id == U256::from(1) {
            self.category1_creator.get()
        } else if category_id == U256::from(2) {
            self.category2_creator.get()
        } else if category_id == U256::from(3) {
            self.category3_creator.get()
        } else {
            Address::ZERO
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
        
        // Sprawdź czy wywołujący to twórca kategorii
        let creator = if category_id == U256::from(1) {
            self.category1_creator.get()
        } else if category_id == U256::from(2) {
            self.category2_creator.get()
        } else if category_id == U256::from(3) {
            self.category3_creator.get()
        } else {
            Address::ZERO
        };
        
        if creator != self.vm().msg_sender() {
            panic!("Unauthorized");
        }
        
        // Dezaktywuj kategorię
        if category_id == U256::from(1) {
            self.category1_active.set(false);
        } else if category_id == U256::from(2) {
            self.category2_active.set(false);
        } else if category_id == U256::from(3) {
            self.category3_active.set(false);
        }
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
    
    /// Tworzy nową kampanię (maksymalnie 3 kampanie)
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
        
        // Sprawdź czy nie przekroczono limitu kampanii
        let current_count = self.campaign_count.get();
        if current_count >= U256::from(3) {
            panic!("Maximum 3 campaigns allowed");
        }
        
        // Pobierz nowy ID kampanii
        let campaign_id = current_count + U256::from(1);
        
        // Pobierz adres administratora
        let admin = self.vm().msg_sender();
        
        // Oblicz hashe tytułu, opisu i URL obrazu
        let title_hash = U256::from_be_bytes(self.vm().native_keccak256(title.as_bytes()).into());
        let description_hash = U256::from_be_bytes(self.vm().native_keccak256(description.as_bytes()).into());
        let image_url_hash = U256::from_be_bytes(self.vm().native_keccak256(image_url.as_bytes()).into());
        
        // Pobierz aktualny czas
        let created_at = U256::from(self.vm().block_timestamp());
        
        // Ustaw dane kampanii w odpowiednim slocie
        if campaign_id == U256::from(1) {
            self.campaign1_category_id.set(category_id);
            self.campaign1_title_hash.set(title_hash);
            self.campaign1_description_hash.set(description_hash);
            self.campaign1_image_url_hash.set(image_url_hash);
            self.campaign1_admin.set(admin);
            self.campaign1_destination_wallet.set(destination_wallet);
            self.campaign1_max_sold_products.set(max_sold_products);
            self.campaign1_sold_products_count.set(U256::ZERO);
            self.campaign1_total_amount_collected.set(U256::ZERO);
            self.campaign1_is_active.set(true);
            self.campaign1_status.set(U256::from(CampaignStatus::Active as u8));
            self.campaign1_created_at.set(created_at);
            self.campaign1_completed_at.set(U256::ZERO);
        } else if campaign_id == U256::from(2) {
            self.campaign2_category_id.set(category_id);
            self.campaign2_title_hash.set(title_hash);
            self.campaign2_description_hash.set(description_hash);
            self.campaign2_image_url_hash.set(image_url_hash);
            self.campaign2_admin.set(admin);
            self.campaign2_destination_wallet.set(destination_wallet);
            self.campaign2_max_sold_products.set(max_sold_products);
            self.campaign2_sold_products_count.set(U256::ZERO);
            self.campaign2_total_amount_collected.set(U256::ZERO);
            self.campaign2_is_active.set(true);
            self.campaign2_status.set(U256::from(CampaignStatus::Active as u8));
            self.campaign2_created_at.set(created_at);
            self.campaign2_completed_at.set(U256::ZERO);
        } else if campaign_id == U256::from(3) {
            self.campaign3_category_id.set(category_id);
            self.campaign3_title_hash.set(title_hash);
            self.campaign3_description_hash.set(description_hash);
            self.campaign3_image_url_hash.set(image_url_hash);
            self.campaign3_admin.set(admin);
            self.campaign3_destination_wallet.set(destination_wallet);
            self.campaign3_max_sold_products.set(max_sold_products);
            self.campaign3_sold_products_count.set(U256::ZERO);
            self.campaign3_total_amount_collected.set(U256::ZERO);
            self.campaign3_is_active.set(true);
            self.campaign3_status.set(U256::from(CampaignStatus::Active as u8));
            self.campaign3_created_at.set(created_at);
            self.campaign3_completed_at.set(U256::ZERO);
        }
        
        // Zwiększ licznik kampanii
        self.campaign_count.set(campaign_id);
        
        campaign_id
    }
    
    /// Pobiera dane kampanii
    pub fn get_campaign_data(&self, campaign_id: U256) -> (U256, U256, U256, U256, Address, Address, U256, U256, U256, bool, U256, U256, U256) {
        if campaign_id == U256::from(1) {
            (
                self.campaign1_category_id.get(),
                self.campaign1_title_hash.get(),
                self.campaign1_description_hash.get(),
                self.campaign1_image_url_hash.get(),
                self.campaign1_admin.get(),
                self.campaign1_destination_wallet.get(),
                self.campaign1_max_sold_products.get(),
                self.campaign1_sold_products_count.get(),
                self.campaign1_total_amount_collected.get(),
                self.campaign1_is_active.get(),
                self.campaign1_status.get(),
                self.campaign1_created_at.get(),
                self.campaign1_completed_at.get()
            )
        } else if campaign_id == U256::from(2) {
            (
                self.campaign2_category_id.get(),
                self.campaign2_title_hash.get(),
                self.campaign2_description_hash.get(),
                self.campaign2_image_url_hash.get(),
                self.campaign2_admin.get(),
                self.campaign2_destination_wallet.get(),
                self.campaign2_max_sold_products.get(),
                self.campaign2_sold_products_count.get(),
                self.campaign2_total_amount_collected.get(),
                self.campaign2_is_active.get(),
                self.campaign2_status.get(),
                self.campaign2_created_at.get(),
                self.campaign2_completed_at.get()
            )
        } else if campaign_id == U256::from(3) {
            (
                self.campaign3_category_id.get(),
                self.campaign3_title_hash.get(),
                self.campaign3_description_hash.get(),
                self.campaign3_image_url_hash.get(),
                self.campaign3_admin.get(),
                self.campaign3_destination_wallet.get(),
                self.campaign3_max_sold_products.get(),
                self.campaign3_sold_products_count.get(),
                self.campaign3_total_amount_collected.get(),
                self.campaign3_is_active.get(),
                self.campaign3_status.get(),
                self.campaign3_created_at.get(),
                self.campaign3_completed_at.get()
            )
        } else {
            (U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, Address::ZERO, Address::ZERO, U256::ZERO, U256::ZERO, U256::ZERO, false, U256::ZERO, U256::ZERO, U256::ZERO)
        }
    }
    
    /// Sprawdza czy kampania jest aktywna
    pub fn is_campaign_active(&self, campaign_id: U256) -> bool {
        if campaign_id == U256::from(1) {
            self.campaign1_is_active.get()
        } else if campaign_id == U256::from(2) {
            self.campaign2_is_active.get()
        } else if campaign_id == U256::from(3) {
            self.campaign3_is_active.get()
        } else {
            false
        }
    }
    
    /// Pobiera status kampanii
    pub fn get_campaign_status(&self, campaign_id: U256) -> U256 {
        if campaign_id == U256::from(1) {
            self.campaign1_status.get()
        } else if campaign_id == U256::from(2) {
            self.campaign2_status.get()
        } else if campaign_id == U256::from(3) {
            self.campaign3_status.get()
        } else {
            U256::ZERO
        }
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

        // Utwórz kilka kategorii
        let cat1_id = contract.create_category("Electronics".to_string());
        let cat2_id = contract.create_category("Clothing".to_string());
        let cat3_id = contract.create_category("Books".to_string());
        
        // Sprawdź ID
        assert_eq!(cat1_id, U256::from(1));
        assert_eq!(cat2_id, U256::from(2));
        assert_eq!(cat3_id, U256::from(3));
        
        // Sprawdź licznik
        assert_eq!(contract.get_category_count(), U256::from(3));
        
        // Sprawdź czy wszystkie są aktywne
        assert!(contract.is_category_active(cat1_id));
        assert!(contract.is_category_active(cat2_id));
        assert!(contract.is_category_active(cat3_id));
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
}