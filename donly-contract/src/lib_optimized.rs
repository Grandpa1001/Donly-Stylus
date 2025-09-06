use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*, msg};

// Define the contract's storage
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
    
    pub fn category_count(&self) -> U256 {
        self.category_count.get()
    }

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

    // Batch function to get all category data
    pub fn get_category_data(&self, id: U256) -> (U256, Address, bool) {
        if id == U256::ZERO || id > self.category_count.get() {
            panic!("Invalid ID");
        }
        let name_hash = self.category_name_hash.get(id);
        if name_hash == U256::ZERO {
            panic!("Category not found");
        }
        (
            name_hash,
            self.category_creator.get(id),
            self.category_is_active.get(id)
        )
    }

    // ===== CAMPAIGN FUNCTIONALITY =====
    
    pub fn campaign_count(&self) -> U256 {
        self.campaign_count.get()
    }

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

    // Batch function to get all campaign data
    pub fn get_campaign_data(&self, id: U256) -> (U256, Address, bool, U256, U256, Address, U256, U256, U256) {
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
            self.campaign_title_hash.get(id),               // titleHash
            self.campaign_description_hash.get(id),         // descriptionHash
            self.campaign_destination_wallet.get(id),       // destinationWallet
            self.campaign_max_sold_products.get(id),        // maxSoldProducts
            self.campaign_sold_products_count.get(id),      // soldProductsCount
            self.campaign_total_amount_collected.get(id)    // totalAmountCollected
        )
    }

    // ===== PRODUCT FUNCTIONALITY =====
    
    pub fn product_count(&self) -> U256 {
        self.product_count.get()
    }

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

    // Batch function to get all product data
    pub fn get_product_data(&self, id: U256) -> (U256, U256, U256, U256, U256, Address, bool, bool) {
        if id == U256::ZERO || id > self.product_count.get() {
            panic!("Invalid ID");
        }
        let campaign_id = self.product_campaign_id.get(id);
        if campaign_id == U256::ZERO {
            panic!("Product not found");
        }
        (
            campaign_id,                                    // campaignId
            self.product_category_id.get(id),               // categoryId
            self.product_name_hash.get(id),                 // nameHash
            self.product_description_hash.get(id),          // descriptionHash
            self.product_price.get(id),                     // price
            self.product_owner.get(id),                     // owner
            self.product_is_active.get(id),                 // isActive
            self.product_is_sold.get(id)                    // isSold
        )
    }

    // ===== PURCHASE FUNCTIONALITY =====
    
    #[payable]
    pub fn purchase_product(&mut self, product_id: U256) {
        if product_id == U256::ZERO || product_id > self.product_count.get() {
            panic!("Invalid product ID");
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
        if msg::value() != product_price {
            panic!("Incorrect funds sent");
        }

        // Update product as sold
        self.product_is_sold.setter(product_id).set(true);

        // Update campaign sold products count
        let current_sold = self.campaign_sold_products_count.get(campaign_id);
        self.campaign_sold_products_count.setter(campaign_id).set(current_sold + U256::from(1));

        // Update campaign total amount collected
        let current_amount = self.campaign_total_amount_collected.get(campaign_id);
        self.campaign_total_amount_collected.setter(campaign_id).set(current_amount + msg::value());

        // Transfer funds to destination wallet
        let destination_wallet = self.campaign_destination_wallet.get(campaign_id);
        let caller = msg::sender();
        
        // Note: In a real implementation, you would transfer ETH here
        // For now, we just track the amount collected
    }

    // ===== BATCH LIST FUNCTIONS =====
    
    // Get all categories (returns arrays of data)
    pub fn get_all_categories(&self) -> (Vec<U256>, Vec<U256>, Vec<Address>, Vec<bool>) {
        let count = self.category_count.get();
        let mut ids = Vec::new();
        let mut name_hashes = Vec::new();
        let mut creators = Vec::new();
        let mut is_active = Vec::new();
        
        for i in 1..=count.as_limbs()[0] {
            let id = U256::from(i);
            let name_hash = self.category_name_hash.get(id);
            if name_hash != U256::ZERO {
                ids.push(id);
                name_hashes.push(name_hash);
                creators.push(self.category_creator.get(id));
                is_active.push(self.category_is_active.get(id));
            }
        }
        
        (ids, name_hashes, creators, is_active)
    }

    // Get all campaigns (returns arrays of data)
    pub fn get_all_campaigns(&self) -> (Vec<U256>, Vec<U256>, Vec<Address>, Vec<bool>) {
        let count = self.campaign_count.get();
        let mut ids = Vec::new();
        let mut category_ids = Vec::new();
        let mut admins = Vec::new();
        let mut is_active = Vec::new();
        
        for i in 1..=count.as_limbs()[0] {
            let id = U256::from(i);
            let category_id = self.campaign_category_id.get(id);
            if category_id != U256::ZERO {
                ids.push(id);
                category_ids.push(category_id);
                admins.push(self.campaign_admin.get(id));
                is_active.push(self.campaign_is_active.get(id));
            }
        }
        
        (ids, category_ids, admins, is_active)
    }

    // Get all products (returns arrays of data)
    pub fn get_all_products(&self) -> (Vec<U256>, Vec<U256>, Vec<U256>, Vec<U256>, Vec<Address>, Vec<bool>, Vec<bool>) {
        let count = self.product_count.get();
        let mut ids = Vec::new();
        let mut campaign_ids = Vec::new();
        let mut category_ids = Vec::new();
        let mut prices = Vec::new();
        let mut owners = Vec::new();
        let mut is_active = Vec::new();
        let mut is_sold = Vec::new();
        
        for i in 1..=count.as_limbs()[0] {
            let id = U256::from(i);
            let campaign_id = self.product_campaign_id.get(id);
            if campaign_id != U256::ZERO {
                ids.push(id);
                campaign_ids.push(campaign_id);
                category_ids.push(self.product_category_id.get(id));
                prices.push(self.product_price.get(id));
                owners.push(self.product_owner.get(id));
                is_active.push(self.product_is_active.get(id));
                is_sold.push(self.product_is_sold.get(id));
            }
        }
        
        (ids, campaign_ids, category_ids, prices, owners, is_active, is_sold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation() {
        // This test simply checks that the code compiles
        assert!(true);
    }
}
