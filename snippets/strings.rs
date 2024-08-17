#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    
   pub fn string_types(env: Env) {
    
        // String
        let msg: &str = "Hello";
        String::from_str(&env, msg);
        
    }
}