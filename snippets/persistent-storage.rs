#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    MyKey
}

#[contract]
pub struct PersistentStorage;

#[contractimpl]
impl PersistentStorage {
    /// Create contract entry

    pub fn setup(env: Env) {
        env.storage().persistent().set(&DataKey::MyKey, &0);
    }

    /// Extend the TTL (Time-To-Live) to 5000 ledgers
    /// in the case that TTL is less than 1000 ledgers

    pub fn extend_persistent(env: Env) {
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::MyKey, 1000, 5000);
    }
}
