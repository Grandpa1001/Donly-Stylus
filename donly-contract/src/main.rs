//! Main entry point for ABI export

#[cfg(feature = "export-abi")]
use donly_contract::Donly;

#[cfg(feature = "export-abi")]
fn main() {
    stylus_sdk::abi::export::print_abi::<Donly>("GPL-3.0-or-later", "pragma solidity ^0.8.23;");
}

#[cfg(not(feature = "export-abi"))]
fn main() {
    // Empty main for compilation without export-abi feature
}
