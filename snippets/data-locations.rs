#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    MyKey
}

#[contract]
pub struct DataLocations;

#[contractimpl]
impl DataLocations {
    /// Create contract entry

    pub fn setup(env: Env) {
        env.storage().instance().set(&DataKey::MyKey, &0);
    }

    /// Extend the TTL (Time-To-Live) to 10000 ledgers
    /// in the case that TTL is less than 2000 ledgers

    pub fn extend_instance(env: Env) {
        env.storage()
            .instance()
            .extend_ttl(2000, 10000);
    }
}
