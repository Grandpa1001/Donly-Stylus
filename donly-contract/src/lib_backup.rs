//!
//! Donly - Crowdfunding Platform on Arbitrum Stylus
//!
//! Simple implementation using basic storage types
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;

use alloc::string::String;
use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*, msg};

sol_storage! {
    #[entrypoint]
    pub struct Donly {
        // Category storage
        uint256 category_count;
        mapping(uint256 => uint256) category_name_hash;
        mapping(uint256 => address) category_creator;
        mapping(uint256 => bool) category_is_active;
        mapping(uint256 => uint256) category_name_hash_to_id;

        // Campaign storage
        uint256 campaign_count;
        mapping(uint256 => uint256) campaign_category_id;
        mapping(uint256 => uint256) campaign_title_hash;
        mapping(uint256 => uint256) campaign_description_hash;
        mapping(uint256 => address) campaign_admin;
        mapping(uint256 => address) campaign_destination_wallet;
        mapping(uint256 => uint256) campaign_max_sold_products;
        mapping(uint256 => uint256) campaign_sold_products_count;
        mapping(uint256 => uint256) campaign_total_amount_collected;
        mapping(uint256 => bool) campaign_is_active;

        // Product storage
        uint256 product_count;
        mapping(uint256 => uint256) product_campaign_id;
        mapping(uint256 => uint256) product_category_id;
        mapping(uint256 => uint256) product_name_hash;
        mapping(uint256 => uint256) product_description_hash;
        mapping(uint256 => uint256) product_price;
        mapping(uint256 => address) product_owner;
        mapping(uint256 => bool) product_is_active;
        mapping(uint256 => bool) product_is_sold;
    }
}

/// Helper for hashing.
fn keccak(data: &[u8]) -> U256 {
    U256::from_be_bytes(stylus_sdk::crypto::keccak(data).0)
}

#[public]
impl Donly {
    // ===== CATEGORY FUNCTIONALITY =====
    
    /// Gets the total number of categories created.
    pub fn category_count(&self) -> U256 {
        self.category_count.get()
    }

    /// Creates a new category.
    pub fn create_category(&mut self, name: String) -> U256 {
        if name.is_empty() || name.len() > 64 {
            panic!("Invalid name");
        }

        let name_hash = keccak(name.as_bytes());
        if self.category_name_hash_to_id.get(name_hash) != U256::ZERO {
            panic!("Category name exists");
        }

        let new_id = self.category_count.get() + U256::from(1);
        self.category_count.set(new_id);

        self.category_name_hash.setter(new_id).set(name_hash);
        self.category_creator.setter(new_id).set(msg::sender());
        self.category_is_active.setter(new_id).set(true);
        self.category_name_hash_to_id.setter(name_hash).set(new_id);

        new_id
    }

    /// Gets a category's name hash by its ID.
    pub fn get_category_name_hash(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        let name_hash = self.category_name_hash.get(id);
        if name_hash == U256::ZERO {
            panic!("Category not found");
        }
        name_hash
    }

    /// Gets a category's creator by its ID.
    pub fn get_category_creator(&self, id: U256) -> Address {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        let name_hash = self.category_name_hash.get(id);
        if name_hash == U256::ZERO {
            panic!("Category not found");
        }
        self.category_creator.get(id)
    }

    /// Gets a category's active status by its ID.
    pub fn get_category_is_active(&self, id: U256) -> bool {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        let name_hash = self.category_name_hash.get(id);
        if name_hash == U256::ZERO {
            panic!("Category not found");
        }
        self.category_is_active.get(id)
    }

    /// Deactivates a category. Only the creator can do this.
    pub fn deactivate_category(&mut self, id: U256) {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        
        let name_hash = self.category_name_hash.get(id);
        if name_hash == U256::ZERO {
            panic!("Category not found");
        }

        if self.category_creator.get(id) != msg::sender() {
            panic!("Unauthorized");
        }

        self.category_is_active.setter(id).set(false);
    }

    // ===== CAMPAIGN FUNCTIONALITY =====

    /// Gets the total number of campaigns created.
    pub fn campaign_count(&self) -> U256 {
        self.campaign_count.get()
    }

    /// Creates a new campaign.
    pub fn create_campaign(
        &mut self,
        category_id: U256,
        title: String,
        description: String,
        destination_wallet: Address,
        max_sold_products: U256,
    ) -> U256 {
        // Validate category exists and is active
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            panic!("Invalid category ID");
        }
        
        let category_name_hash = self.category_name_hash.get(category_id);
        if category_name_hash == U256::ZERO {
            panic!("Category not found");
        }
        
        if !self.category_is_active.get(category_id) {
            panic!("Category not active");
        }

        // Validate campaign data
        if title.is_empty() || title.len() > 64 {
            panic!("Invalid title");
        }
        if description.is_empty() || description.len() > 256 {
            panic!("Invalid description");
        }
        if max_sold_products == U256::ZERO {
            panic!("Invalid max sold products");
        }

        let new_id = self.campaign_count.get() + U256::from(1);
        self.campaign_count.set(new_id);

        self.campaign_category_id.setter(new_id).set(category_id);
        self.campaign_title_hash.setter(new_id).set(keccak(title.as_bytes()));
        self.campaign_description_hash.setter(new_id).set(keccak(description.as_bytes()));
        self.campaign_admin.setter(new_id).set(msg::sender());
        self.campaign_destination_wallet.setter(new_id).set(destination_wallet);
        self.campaign_max_sold_products.setter(new_id).set(max_sold_products);
        self.campaign_sold_products_count.setter(new_id).set(U256::ZERO);
        self.campaign_total_amount_collected.setter(new_id).set(U256::ZERO);
        self.campaign_is_active.setter(new_id).set(true);

        new_id
    }

    /// Gets campaign data by ID.
    pub fn get_campaign_category_id(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        category_id
    }

    pub fn get_campaign_admin(&self, id: U256) -> Address {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_admin.get(id)
    }

    pub fn get_campaign_is_active(&self, id: U256) -> bool {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_is_active.get(id)
    }

    pub fn get_campaign_sold_products_count(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_sold_products_count.get(id)
    }

    pub fn get_campaign_max_sold_products(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_max_sold_products.get(id)
    }

    pub fn get_campaign_title_hash(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_title_hash.get(id)
    }

    pub fn get_campaign_description_hash(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_description_hash.get(id)
    }

    pub fn get_campaign_destination_wallet(&self, id: U256) -> Address {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_destination_wallet.get(id)
    }

    pub fn get_campaign_total_amount_collected(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        self.campaign_total_amount_collected.get(id)
    }

    /// Deactivates a campaign. Only the admin can do this.
    pub fn deactivate_campaign(&mut self, id: U256) {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }

        if self.campaign_admin.get(id) != msg::sender() {
            panic!("Unauthorized");
        }
        if !self.campaign_is_active.get(id) {
            panic!("Campaign not active");
        }

        self.campaign_is_active.setter(id).set(false);
    }

    // ===== PRODUCT FUNCTIONALITY =====

    /// Gets the total number of products created.
    pub fn product_count(&self) -> U256 {
        self.product_count.get()
    }
    
    /// Adds a new product to a campaign.
    pub fn add_product(
        &mut self,
        campaign_id: U256,
        category_id: U256,
        name: String,
        description: String,
        price: U256,
    ) -> U256 {
        // Validate campaign exists and is active
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            panic!("Invalid campaign ID");
        }
        
        let campaign_category_id = self.campaign_category_id.get(campaign_id);
        if campaign_category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        
        if !self.campaign_is_active.get(campaign_id) {
            panic!("Campaign not active");
        }

        // Validate category exists and is active
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            panic!("Invalid category ID");
        }
        
        let category_name_hash = self.category_name_hash.get(category_id);
        if category_name_hash == U256::ZERO {
            panic!("Category not found");
        }
        
        if !self.category_is_active.get(category_id) {
            panic!("Category not active");
        }

        // Validate product data
        if price == U256::ZERO {
            panic!("Invalid price");
        }
        if name.is_empty() || name.len() > 64 {
            panic!("Invalid name");
        }
        if description.is_empty() || description.len() > 256 {
            panic!("Invalid description");
        }

        let new_id = self.product_count.get() + U256::from(1);
        self.product_count.set(new_id);

        self.product_campaign_id.setter(new_id).set(campaign_id);
        self.product_category_id.setter(new_id).set(category_id);
        self.product_name_hash.setter(new_id).set(keccak(name.as_bytes()));
        self.product_description_hash.setter(new_id).set(keccak(description.as_bytes()));
        self.product_price.setter(new_id).set(price);
        self.product_owner.setter(new_id).set(msg::sender());
        self.product_is_active.setter(new_id).set(true);
        self.product_is_sold.setter(new_id).set(false);

        new_id
    }

    /// Gets product data by ID.
    pub fn get_product_campaign_id(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        campaign_id
    }

    pub fn get_product_price(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_price.get(id)
    }

    pub fn get_product_is_active(&self, id: U256) -> bool {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_is_active.get(id)
    }

    pub fn get_product_is_sold(&self, id: U256) -> bool {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_is_sold.get(id)
    }

    pub fn get_product_name_hash(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_name_hash.get(id)
    }

    pub fn get_product_description_hash(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_description_hash.get(id)
    }

    pub fn get_product_category_id(&self, id: U256) -> U256 {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_category_id.get(id)
    }

    pub fn get_product_owner(&self, id: U256) -> Address {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        self.product_owner.get(id)
    }

    /// Purchases a product. The user must send the exact price in ETH.
    #[payable]
    pub fn purchase_product(&mut self, product_id: U256) {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            panic!("Invalid ID");
        }
        
        let campaign_id = self.product_campaign_id.get(product_id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }

        if !self.product_is_active.get(product_id) {
            panic!("Product not active");
        }
        if self.product_is_sold.get(product_id) {
            panic!("Product already sold");
        }

        let product_price = self.product_price.get(product_id);
        
        // Check if the correct amount of ETH was sent
        if msg::value() != product_price {
            panic!("Incorrect funds sent");
        }

        if !self.campaign_is_active.get(campaign_id) {
            panic!("Campaign not active");
        }

        // Update product state
        self.product_is_sold.setter(product_id).set(true);
        self.product_is_active.setter(product_id).set(false);

        // Update campaign stats
        let current_sold = self.campaign_sold_products_count.get(campaign_id);
        let max_products = self.campaign_max_sold_products.get(campaign_id);
        let current_amount = self.campaign_total_amount_collected.get(campaign_id);
        
        self.campaign_sold_products_count.setter(campaign_id).set(current_sold + U256::from(1));
        self.campaign_total_amount_collected.setter(campaign_id).set(current_amount + msg::value());

        // Check if the campaign goal has been reached
        if current_sold + U256::from(1) >= max_products {
            self.campaign_is_active.setter(campaign_id).set(false);
        }
    }

    /// Deactivates a product. Only the product owner or campaign admin can do this.
    pub fn deactivate_product(&mut self, product_id: U256) {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            panic!("Invalid ID");
        }
        
        let campaign_id = self.product_campaign_id.get(product_id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }

        if !self.product_is_active.get(product_id) {
            panic!("Product not active");
        }
        if self.product_is_sold.get(product_id) {
            panic!("Product already sold");
        }

        let caller = msg::sender();
        let product_owner = self.product_owner.get(product_id);
        let campaign_admin = self.campaign_admin.get(campaign_id);

        if product_owner != caller && campaign_admin != caller {
            panic!("Unauthorized");
        }

        self.product_is_active.setter(product_id).set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation() {
        // This test simply checks that the code compiles
        // Real testing would require proper VM setup which is complex
        assert!(true);
    }
}