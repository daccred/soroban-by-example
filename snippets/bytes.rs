#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Bytes, BytesN};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    
   pub fn symbol_type(env: Env) {
    
        // Bytes (Bytes and BytesN)
        let bytes = Bytes::from_slice(&env, &[1; 32]);
        let bytes_n = BytesN::from_array(&env, &[0; 32]);
        
    }
}