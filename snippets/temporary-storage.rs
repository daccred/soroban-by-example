#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    MyKey
}

#[contract]
pub struct TemporaryStorage;

#[contractimpl]
impl TemporaryStorage {
    /// Create contract entry

    pub fn setup(env: Env) {
        env.storage().temporary().set(&DataKey::MyKey, &0);
    }

    /// Extend the TTL (Time-To-Live) to 7000 ledgers
    /// in the case that TTL is less than 3000 ledgers

    pub fn extend_temporary(env: Env) {
        env.storage()
            .temporary()
            .extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}
