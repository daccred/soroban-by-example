#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct LoggingExample;

#[contractimpl]
impl LoggingExample {
    /// This is an implementation of logging in Soroban.

    pub fn output(env: Env, info: String) {
        log!(&env, "Message: {}!", info);
    }
}
