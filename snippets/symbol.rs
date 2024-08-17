#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    
   pub fn symbol_type(env: Env) {
    
        // Symbols (short and new)
        let symbol_short = symbol_short!("Sample"); // up to 9 chars

        let symbol_new = Symbol::new(&env, "SampleSymbolExpression");
        
    }
}