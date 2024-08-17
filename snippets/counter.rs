#![no_std]
use soroban_sdk::{contract, contractimpl, Env, log, Symbol, symbol_short};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    const COUNT_KEY: Symbol = symbol_short!("count");

    // Function to get the current count
    pub fn get_count(env: Env) -> u32 {
        env.storage().instance().get(&Self::COUNT_KEY).unwrap_or(0) // Default to 0 if not set
    }

    // Function to increment the count by 1
    pub fn increment(env: Env) {
        let mut count = Self::get_count(env.clone());
        count += 1;
        env.storage().instance().set(&Self::COUNT_KEY, &count);
        log!(&env, "count(+): {}", count);
    }

    // Function to decrement the count by 1
    pub fn decrement(env: Env) {
        let mut count = Self::get_count(env.clone());
        if count > 0 {
            count -= 1;
            env.storage().instance().set(&Self::COUNT_KEY, &count);
        }

        log!(&env, "count(-): {}", count);
    }
}