#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    
   pub fn integer_types() {
         // Unsigned Integers
        let unsigned_32_bit: u32 = 42;
        let unsigned_64_bit: u64 = 42;
        let unsigned_128_bit: u128 = 42;

        // Signed Integers
        let signed_32_bit: i32 = -42;
        let signed_64_bit: i64 = -42;
        let signed_128_bit: i128 = -42;
    }
}