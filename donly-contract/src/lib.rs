//!
//! Donly - Crowdfunding Platform on Arbitrum Stylus
//!
//! Simple implementation using basic storage types
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;

use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Donly {
        // Category storage
        uint256 category_count;
        mapping(uint256 => address) category_creator;
        mapping(uint256 => bool) category_is_active;

        // Campaign storage
        uint256 campaign_count;
        mapping(uint256 => uint256) campaign_category_id;
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
        mapping(uint256 => uint256) product_price;
        mapping(uint256 => address) product_owner;
        mapping(uint256 => bool) product_is_active;
        mapping(uint256 => bool) product_is_sold;
    }
}



#[public]
impl Donly {
    // ===== CATEGORY FUNCTIONALITY =====
    
    /// Gets the total number of categories created.
    pub fn category_count(&self) -> U256 {
        self.category_count.get()
    }

    /// Creates a new category.
    pub fn create_category(&mut self) -> U256 {
        let new_id = self.category_count.get() + U256::from(1);
        self.category_count.set(new_id);
        let sender = self.vm().msg_sender();
        self.category_creator.setter(new_id).set(sender);
        self.category_is_active.setter(new_id).set(true);
        
        new_id
    }


    /// Gets a category's creator by its ID.
    pub fn get_category_creator(&self, id: U256) -> Address {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        self.category_creator.get(id)
    }

    /// Gets a category's active status by its ID.
    pub fn get_category_is_active(&self, id: U256) -> bool {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        self.category_is_active.get(id)
    }

    /// Deactivates a category. Only the creator can do this.
    pub fn deactivate_category(&mut self, id: U256) {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        

        if self.category_creator.get(id) != self.vm().msg_sender() {
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
        destination_wallet: Address,
        max_sold_products: U256,
    ) -> U256 {
        // Validate category exists and is active
        if category_id == U256::ZERO || category_id > self.category_count.get() {
            panic!("Invalid category ID");
        }
        
        if !self.category_is_active.get(category_id) {
            panic!("Category not active");
        }

        // Validate campaign data
        if max_sold_products == U256::ZERO {
            panic!("Invalid max sold products");
        }

        let new_id = self.campaign_count.get() + U256::from(1);
        self.campaign_count.set(new_id);

        self.campaign_category_id.setter(new_id).set(category_id);
        let sender = self.vm().msg_sender();
        self.campaign_admin.setter(new_id).set(sender);
        self.campaign_destination_wallet.setter(new_id).set(destination_wallet);
        self.campaign_max_sold_products.setter(new_id).set(max_sold_products);
        self.campaign_sold_products_count.setter(new_id).set(U256::ZERO);
        self.campaign_total_amount_collected.setter(new_id).set(U256::ZERO);
        self.campaign_is_active.setter(new_id).set(true);

        new_id
    }

    /// Gets campaign data by ID.

    /// Gets all campaign data in one call
    pub fn get_campaign_data(&self, id: U256) -> (U256, Address, bool, U256, U256, Address) {
        if id == U256::ZERO || id > self.campaign_count.get() {
            panic!("Invalid ID");
        }
        let category_id = self.campaign_category_id.get(id);
        if category_id == U256::ZERO {
            panic!("Campaign not found");
        }
        
        (
            category_id,                                    // categoryId
            self.campaign_admin.get(id),                    // admin
            self.campaign_is_active.get(id),                // isActive
            self.campaign_sold_products_count.get(id),      // soldProductsCount
            self.campaign_max_sold_products.get(id),        // maxSoldProducts
            self.campaign_destination_wallet.get(id)        // destinationWallet
        )
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

        if self.campaign_admin.get(id) != self.vm().msg_sender() {
            panic!("Unauthorized");
        }
        if !self.campaign_is_active.get(id) {
            panic!("Campaign not active");
        }

        self.campaign_is_active.setter(id).set(false);
    }

    /// Withdraws campaign funds to the destination wallet. Only the campaign admin can do this.
    pub fn withdraw_campaign_funds(&mut self, campaign_id: U256) -> bool {
        if campaign_id == U256::ZERO || campaign_id > self.campaign_count.get() {
            return false;
        }
        
        let category_id = self.campaign_category_id.get(campaign_id);
        if category_id == U256::ZERO {
            return false;
        }

        if self.campaign_admin.get(campaign_id) != self.vm().msg_sender() {
            return false;
        }

        let total_collected = self.campaign_total_amount_collected.get(campaign_id);
        if total_collected == U256::ZERO {
            return false;
        }

        let destination_wallet = self.campaign_destination_wallet.get(campaign_id);
        
        // Transfer all collected funds to the destination wallet
        if let Err(_) = self.vm().transfer_eth(destination_wallet, total_collected) {
            return false;
        }
        
        // Reset the collected amount to prevent double withdrawal
        self.campaign_total_amount_collected.setter(campaign_id).set(U256::ZERO);
        
        true
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
        
        if !self.category_is_active.get(category_id) {
            panic!("Category not active");
        }

        // Validate product data
        if price == U256::ZERO {
            panic!("Invalid price");
        }

        let new_id = self.product_count.get() + U256::from(1);
        self.product_count.set(new_id);

        self.product_campaign_id.setter(new_id).set(campaign_id);
        self.product_category_id.setter(new_id).set(category_id);
        self.product_price.setter(new_id).set(price);
        let sender = self.vm().msg_sender();
        self.product_owner.setter(new_id).set(sender);
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

    /// Gets a product's owner by its ID.
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
    pub fn purchase_product(&mut self, product_id: U256) -> bool {
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
        if self.vm().msg_value() != product_price {
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
        
        // Check if the campaign goal has been reached
        if current_sold + U256::from(1) >= max_products {
            self.campaign_is_active.setter(campaign_id).set(false);
            
            // Automatically withdraw all collected funds to the destination wallet
            let destination_wallet = self.campaign_destination_wallet.get(campaign_id);
            let total_collected = current_amount + self.vm().msg_value();
            
            if let Err(_) = self.vm().transfer_eth(destination_wallet, total_collected) {
                // If transfer fails, we can't panic in a payable function
                // The funds will remain in the contract and can be withdrawn manually
                self.campaign_total_amount_collected.setter(campaign_id).set(total_collected);
            } else {
                // Reset the collected amount after successful transfer
                self.campaign_total_amount_collected.setter(campaign_id).set(U256::ZERO);
            }
        } else {
            // Update the collected amount if campaign is not finished
            let value = self.vm().msg_value();
            self.campaign_total_amount_collected.setter(campaign_id).set(current_amount + value);
        }

        true
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

        let caller = self.vm().msg_sender();
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