#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String, log};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    pub fn address_type(env: Env, strkey: String) {
        // Create an Address from a strkey
        let address = Address::from_string(&strkey);
        
        // Log the Address as a string
        let addr_str = address.to_string();
        log!(&env, "Address: {}", addr_str);

        // Ensure the address has authorized the current contract invocation
        address.require_auth();
    }
}