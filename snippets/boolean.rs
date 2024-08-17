#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    
   pub fn boolean_types() {
        // Boolean
        let on: bool = true;
        let off: bool = false; 
    }
}
