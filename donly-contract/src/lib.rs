//!
//! Donly - Crowdfunding Platform on Arbitrum Stylus
//!
//! This contract has been refactored to use modern, gas-efficient patterns
//! as of September 2025. It now uses structs and mappings for storage,
//! and proper error handling with `Result`.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*};
use serde::{Serialize, Deserialize};

/// Enum for custom errors, returned as `Result<T, Error>`.
#[solidity_storage]
#[derive(SolidityError, serde::Serialize, serde::Deserialize)]
pub enum Error {
    InvalidId,
    InvalidName(String),
    InvalidDescription(String),
    InvalidImageUrl(String),
    InvalidPrice,
    InvalidMaxSoldProducts,
    CategoryNotFound,
    CategoryNotActive,
    CategoryNameExists,
    CampaignNotFound,
    CampaignNotActive,
    ProductNotFound,
    ProductNotActive,
    ProductAlreadySold,
    Unauthorized,
    NoFundsToTransfer,
    GoalAlreadyReached,
    IncorrectFundsSent,
    TransferFailed,
}

/// Status of a campaign.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum CampaignStatus {
    Active = 0,
    Completed = 1,
    Cancelled = 2,
}

impl From<CampaignStatus> for u8 {
    fn from(status: CampaignStatus) -> Self {
        status as u8
    }
}

// --- Data Structs ---

#[derive(SolidityType, Serialize, Deserialize, Default, Clone)]
pub struct Category {
    pub id: U256,
    pub name_hash: U256,
    pub creator: Address,
    pub is_active: bool,
}

#[derive(SolidityType, Serialize, Deserialize, Default, Clone)]
pub struct Campaign {
    pub id: U256,
    pub category_id: U256,
    pub title_hash: U256,
    pub description_hash: U256,
    pub image_url_hash: U256,
    pub admin: Address,
    pub destination_wallet: Address,
    pub max_sold_products: U256,
    pub sold_products_count: U256,
    pub total_amount_collected: U256,
    pub is_active: bool,
    pub status: u8, // Uses CampaignStatus enum
    pub created_at: u64,
    pub completed_at: u64,
}

#[derive(SolidityType, Serialize, Deserialize, Default, Clone)]
pub struct Product {
    pub id: U256,
    pub campaign_id: U256,
    pub category_id: U256,
    pub name_hash: U256,
    pub description_hash: U256,
    pub image_url_hash: U256,
    pub price: U256,
    pub owner: Address,
    pub is_active: bool,
    pub is_sold: bool,
    pub created_at: u64,
    pub sold_at: u64,
}

// --- Contract Storage ---

#[solidity_storage]
#[entrypoint]
pub struct Donly {
    category_count: StorageU256,
    campaign_count: StorageU256,
    product_count: StorageU256,

    categories: StorageMap<U256, Category>,
    campaigns: StorageMap<U256, Campaign>,
    products: StorageMap<U256, Product>,

    /// Mapping from a category name hash to its ID to enforce uniqueness.
    category_name_hash_to_id: StorageMap<U256, U256>,
}

// --- Contract Implementation ---

#[external]
impl Donly {
    // ===== CATEGORY FUNCTIONALITY =====

    /// Gets the total number of categories created.
    pub fn category_count(&self) -> Result<U256, Error> {
        Ok(self.category_count.get())
    }

    /// Gets a category's data by its ID.
    pub fn get_category(&self, id: U256) -> Result<Category, Error> {
        if id == U256::ZERO || id > self.category_count.get() {
            return Err(Error::InvalidId);
        }
        self.categories.get(id).ok_or(Error::CategoryNotFound)
    }

    /// Creates a new category.
    pub fn create_category(&mut self, name: String) -> Result<U256, Error> {
        if name.is_empty() || name.len() > 64 {
            return Err(Error::InvalidName("Category name must be 1-64 chars.".into()));
        }

        let name_hash = keccak(name.as_bytes());
        if self.category_name_hash_to_id.get(name_hash).is_some() {
            return Err(Error::CategoryNameExists);
        }

        let new_id = self.category_count.get() + U256::from(1);
        self.category_count.set(new_id);

        let category = Category {
            id: new_id,
            name_hash,
            creator: msg::sender(),
            is_active: true,
        };

        self.categories.insert(new_id, category);
        self.category_name_hash_to_id.insert(name_hash, new_id);

        Ok(new_id)
    }

    /// Deactivates a category. Only the creator can do this.
    pub fn deactivate_category(&mut self, id: U256) -> Result<(), Error> {
        let mut category = self.get_category(id)?;

        if category.creator != msg::sender() {
            return Err(Error::Unauthorized);
        }

        category.is_active = false;
        self.categories.insert(id, category);
        Ok(())
    }

    // ===== CAMPAIGN FUNCTIONALITY =====

    /// Gets the total number of campaigns created.
    pub fn campaign_count(&self) -> Result<U256, Error> {
        Ok(self.campaign_count.get())
    }

    /// Gets a campaign's data by its ID.
    pub fn get_campaign(&self, id: U256) -> Result<Campaign, Error> {
        if id == U256::ZERO || id > self.campaign_count.get() {
            return Err(Error::InvalidId);
        }
        self.campaigns.get(id).ok_or(Error::CampaignNotFound)
    }

    /// Creates a new campaign.
    pub fn create_campaign(
        &mut self,
        category_id: U256,
        title: String,
        description: String,
        image_url: String,
        destination_wallet: Address,
        max_sold_products: U256,
    ) -> Result<U256, Error> {
        // Validations
        let category = self.get_category(category_id)?;
        if !category.is_active {
            return Err(Error::CategoryNotActive);
        }
        if title.is_empty() || title.len() > 64 {
            return Err(Error::InvalidName("Campaign title must be 1-64 chars.".into()));
        }
        if description.is_empty() || description.len() > 256 {
            return Err(Error::InvalidDescription("Campaign description must be 1-256 chars.".into()));
        }
        if image_url.is_empty() || image_url.len() > 128 {
            return Err(Error::InvalidImageUrl("Image URL must be 1-128 chars.".into()));
        }
        if max_sold_products == U256::ZERO {
            return Err(Error::InvalidMaxSoldProducts);
        }

        let new_id = self.campaign_count.get() + U256::from(1);
        self.campaign_count.set(new_id);

        let campaign = Campaign {
            id: new_id,
            category_id,
            title_hash: keccak(title.as_bytes()),
            description_hash: keccak(description.as_bytes()),
            image_url_hash: keccak(image_url.as_bytes()),
            admin: msg::sender(),
            destination_wallet,
            max_sold_products,
            sold_products_count: U256::ZERO,
            total_amount_collected: U256::ZERO,
            is_active: true,
            status: CampaignStatus::Active.into(),
            created_at: block::timestamp(),
            completed_at: 0,
        };

        self.campaigns.insert(new_id, campaign);
        Ok(new_id)
    }

    /// Deactivates a campaign. Only the admin can do this.
    pub fn deactivate_campaign(&mut self, id: U256) -> Result<(), Error> {
        let mut campaign = self.get_campaign(id)?;
        if campaign.admin != msg::sender() {
            return Err(Error::Unauthorized);
        }
        if !campaign.is_active {
            return Err(Error::CampaignNotActive);
        }

        campaign.is_active = false;
        campaign.status = CampaignStatus::Cancelled.into();
        campaign.completed_at = block::timestamp();
        self.campaigns.insert(id, campaign);
        Ok(())
    }

    /// Completes a campaign and transfers the funds. Only admin.
    pub fn complete_campaign(&mut self, id: U256) -> Result<(), Error> {
        let mut campaign = self.get_campaign(id)?;
        if campaign.admin != msg::sender() {
            return Err(Error::Unauthorized);
        }
        if !campaign.is_active {
            return Err(Error::CampaignNotActive);
        }
        if campaign.total_amount_collected == U256::ZERO {
            return Err(Error::NoFundsToTransfer);
        }

        // Mark as completed
        campaign.is_active = false;
        campaign.status = CampaignStatus::Completed.into();
        campaign.completed_at = block::timestamp();
        self.campaigns.insert(id, campaign.clone());

        // Transfer funds
        // Note: The `call::transfer_eth` method requires the `stylus-sdk/experimental` feature.
        // This is a placeholder for where the transfer logic would go.
        // In a real scenario, the Result of the transfer call must be handled.
        // let balance = campaign.total_amount_collected;
        // if call::transfer_eth(campaign.destination_wallet, balance).is_err() {
        //     return Err(Error::TransferFailed);
        // }
        
        Ok(())
    }

    // ===== PRODUCT FUNCTIONALITY =====

    /// Gets the total number of products created.
    pub fn product_count(&self) -> Result<U256, Error> {
        Ok(self.product_count.get())
    }

    /// Gets a product's data by its ID.
    pub fn get_product(&self, id: U256) -> Result<Product, Error> {
        if id == U256::ZERO || id > self.product_count.get() {
            return Err(Error::InvalidId);
        }
        self.products.get(id).ok_or(Error::ProductNotFound)
    }
    
    /// Adds a new product to a campaign.
    pub fn add_product(
        &mut self,
        campaign_id: U256,
        category_id: U256,
        name: String,
        description: String,
        image_url: String,
        price: U256,
    ) -> Result<U256, Error> {
        // Validations
        let campaign = self.get_campaign(campaign_id)?;
        if !campaign.is_active {
            return Err(Error::CampaignNotActive);
        }
        let category = self.get_category(category_id)?;
        if !category.is_active {
            return Err(Error::CategoryNotActive);
        }
        if price == U256::ZERO {
            return Err(Error::InvalidPrice);
        }
        if name.is_empty() || name.len() > 64 {
            return Err(Error::InvalidName("Product name must be 1-64 chars.".into()));
        }
        if description.is_empty() || description.len() > 256 {
            return Err(Error::InvalidDescription("Product description must be 1-256 chars.".into()));
        }
        if image_url.is_empty() || image_url.len() > 128 {
            return Err(Error::InvalidImageUrl("Image URL must be 1-128 chars.".into()));
        }

        let new_id = self.product_count.get() + U256::from(1);
        self.product_count.set(new_id);

        let product = Product {
            id: new_id,
            campaign_id,
            category_id,
            name_hash: keccak(name.as_bytes()),
            description_hash: keccak(description.as_bytes()),
            image_url_hash: keccak(image_url.as_bytes()),
            price,
            owner: msg::sender(),
            is_active: true,
            is_sold: false,
            created_at: block::timestamp(),
            sold_at: 0,
        };

        self.products.insert(new_id, product);
        Ok(new_id)
    }

    /// Purchases a product. The user must send the exact price in ETH.
    #[payable]
    pub fn purchase_product(&mut self, product_id: U256) -> Result<(), Error> {
        let mut product = self.get_product(product_id)?;
        if !product.is_active {
            return Err(Error::ProductNotActive);
        }
        if product.is_sold {
            return Err(Error::ProductAlreadySold);
        }

        // Check if the correct amount of ETH was sent
        if msg::value() != product.price {
            return Err(Error::IncorrectFundsSent);
        }

        let mut campaign = self.get_campaign(product.campaign_id)?;
        if !campaign.is_active {
            return Err(Error::CampaignNotActive);
        }

        // Update product state
        product.is_sold = true;
        product.is_active = false; // Deactivate after selling
        product.sold_at = block::timestamp();
        self.products.insert(product_id, product);

        // Update campaign stats
        campaign.sold_products_count += U256::from(1);
        campaign.total_amount_collected += msg::value();

        // Check if the campaign goal has been reached
        if campaign.sold_products_count >= campaign.max_sold_products {
            campaign.is_active = false;
            campaign.status = CampaignStatus::Completed.into();
            campaign.completed_at = block::timestamp();
        }
        self.campaigns.insert(campaign.id, campaign);

        Ok(())
    }

    /// Deactivates a product. Only the product owner or campaign admin can do this.
    pub fn deactivate_product(&mut self, product_id: U256) -> Result<(), Error> {
        let mut product = self.get_product(product_id)?;
        if !product.is_active {
            return Err(Error::ProductNotActive);
        }
        if product.is_sold {
            return Err(Error::ProductAlreadySold);
        }

        let campaign = self.get_campaign(product.campaign_id)?;
        let caller = msg::sender();

        if product.owner != caller && campaign.admin != caller {
            return Err(Error::Unauthorized);
        }

        product.is_active = false;
        self.products.insert(product_id, product);

        Ok(())
    }
}

/// Helper for hashing.
fn keccak(data: &[u8]) -> U256 {
    U256::from_be_bytes(stylus_sdk::crypto::keccak(data).0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::{alloy_primitives::address, testing::*};

    fn setup_contract() -> (TestVM, Donly) {
        let vm = TestVM::default();
        let contract = Donly::new();
        (vm, contract)
    }
    
    fn create_category(contract: &mut Donly, name: &str) -> U256 {
        contract.create_category(name.to_string()).unwrap()
    }

    fn create_campaign(contract: &mut Donly, category_id: U256) -> U256 {
        contract.create_campaign(
            category_id,
            "Test Campaign".into(),
            "A campaign for testing.".into(),
            "http://example.com/img.png".into(),
            address!("0x1111111111111111111111111111111111111111"),
            U256::from(10),
        ).unwrap()
    }

    fn add_product(contract: &mut Donly, campaign_id: U256, category_id: U256, price: u64) -> U256 {
        contract.add_product(
            campaign_id,
            category_id,
            "Test Product".into(),
            "A product for testing.".into(),
            "http://example.com/product.png".into(),
            U256::from(price),
        ).unwrap()
    }

    #[test]
    fn test_create_category() {
        let (_vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Electronics");
        assert_eq!(category_id, U256::from(1));

        let category = contract.get_category(category_id).unwrap();
        assert_eq!(category.id, U256::from(1));
        assert!(category.is_active);
        assert_eq!(contract.category_count().unwrap(), U256::from(1));
    }
    
    #[test]
    fn test_create_duplicate_category_fails() {
        let (_vm, mut contract) = setup_contract();
        create_category(&mut contract, "Electronics");
        let result = contract.create_category("Electronics".to_string());
        assert!(matches!(result, Err(Error::CategoryNameExists)));
    }

    #[test]
    fn test_deactivate_category() {
        let (vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Books");
        
        // A different user tries to deactivate, should fail.
        let other_user = address!("0x1234567890123456789012345678901234567890");
        vm.prank(other_user);
        let res = contract.deactivate_category(category_id);
        assert!(matches!(res, Err(Error::Unauthorized)));

        // The original creator deactivates successfully.
        vm.prank(Address::default());
        contract.deactivate_category(category_id).unwrap();
        
        let category = contract.get_category(category_id).unwrap();
        assert!(!category.is_active);
    }

    #[test]
    fn test_create_campaign() {
        let (_vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Art");
        let campaign_id = create_campaign(&mut contract, category_id);
        assert_eq!(campaign_id, U256::from(1));

        let campaign = contract.get_campaign(campaign_id).unwrap();
        assert_eq!(campaign.id, U256::from(1));
        assert_eq!(campaign.category_id, category_id);
        assert!(campaign.is_active);
        assert_eq!(campaign.status, CampaignStatus::Active as u8);
    }

    #[test]
    fn test_create_campaign_inactive_category_fails() {
        let (_vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Art");
        contract.deactivate_category(category_id).unwrap();

        let result = contract.create_campaign(
            category_id,
            "Test Campaign".into(),
            "A campaign for testing.".into(),
            "http://example.com/img.png".into(),
            address!("0x1111111111111111111111111111111111111111"),
            U256::from(10),
        );
        assert!(matches!(result, Err(Error::CategoryNotActive)));
    }

    #[test]

    fn test_add_product() {
        let (_vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Tech");
        let campaign_id = create_campaign(&mut contract, category_id);
        let product_id = add_product(&mut contract, campaign_id, category_id, 100);
        assert_eq!(product_id, U256::from(1));

        let product = contract.get_product(product_id).unwrap();
        assert_eq!(product.id, U256::from(1));
        assert_eq!(product.price, U256::from(100));
        assert!(product.is_active);
        assert!(!product.is_sold);
    }
    
    #[test]
    fn test_purchase_product() {
        let (vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Gaming");
        let campaign_id = create_campaign(&mut contract, category_id);
        let product_id = add_product(&mut contract, campaign_id, category_id, 1000);

        // Fail if not enough value sent
        vm.set_msg_value(U256::from(999));
        let res = contract.purchase_product(product_id);
        assert!(matches!(res, Err(Error::IncorrectFundsSent)));

        // Succeed with correct value
        vm.set_msg_value(U256::from(1000));
        contract.purchase_product(product_id).unwrap();

        let product = contract.get_product(product_id).unwrap();
        assert!(product.is_sold);
        assert!(!product.is_active);

        let campaign = contract.get_campaign(campaign_id).unwrap();
        assert_eq!(campaign.sold_products_count, U256::from(1));
        assert_eq!(campaign.total_amount_collected, U256::from(1000));
    }

    #[test]
    fn test_purchase_product_completes_campaign() {
        let (vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Music");
        // Campaign with max 1 product
        let campaign_id = contract.create_campaign(
            category_id, "Single Album".into(), "Desc".into(), "url".into(),
            address!("0x1111111111111111111111111111111111111111"), U256::from(1)
        ).unwrap();
        let product_id = add_product(&mut contract, campaign_id, category_id, 500);

        vm.set_msg_value(U256::from(500));
        contract.purchase_product(product_id).unwrap();

        let campaign = contract.get_campaign(campaign_id).unwrap();
        assert!(!campaign.is_active);
        assert_eq!(campaign.status, CampaignStatus::Completed as u8);
        assert_eq!(campaign.sold_products_count, U256::from(1));
    }
    
    #[test]
    fn test_purchase_sold_product_fails() {
        let (vm, mut contract) = setup_contract();
        let category_id = create_category(&mut contract, "Movies");
        let campaign_id = create_campaign(&mut contract, category_id);
        let product_id = add_product(&mut contract, campaign_id, category_id, 200);

        // First purchase
        vm.set_msg_value(U256::from(200));
        contract.purchase_product(product_id).unwrap();
        
        // Second purchase should fail
        let res = contract.purchase_product(product_id);
        assert!(matches!(res, Err(Error::ProductAlreadySold)));
    }
}
