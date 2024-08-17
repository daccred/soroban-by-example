#![no_std]
use soroban_sdk::{contract, contractimpl, Env, log};

#[contract]
pub struct HelloWorld;

#[contractimpl]
impl HelloWorld {
    /// Welcome to Soroban-by-example!
    ///
    /// The function logs a "Hello World" message to the Soroban environment.
    ///
    /// Have fun exploring Soroban!
    pub fn say_hello(env: Env) {
        log!(&env, "Hello World");
    }
}