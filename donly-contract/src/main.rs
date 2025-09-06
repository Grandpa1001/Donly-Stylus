//! Main entry point for ABI export

#[cfg(feature = "export-abi")]
use donly_contract::Donly;

#[cfg(feature = "export-abi")]
fn main() {
    stylus_sdk::abi::export::print_abi::<Donly>("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}

#[cfg(not(feature = "export-abi"))]
fn main() {
    // Empty main for compilation without export-abi feature
}
