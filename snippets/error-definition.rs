#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, log, symbol_short, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}

const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: u32 = 5;

// Update code below this line

#[contract]
pub struct Incrementer;

#[contractimpl]
impl Incrementer {
    /// Increment increments an internal counter, and returns the value.
    /// Returns errors if the value is attempted to be incremented past 5.
    pub fn increment(env: Env) -> Result<u32, Error> {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Check if the count exceeds the max.
        if count <= MAX {
            // Save the count.
            env.storage().instance().set(&COUNTER, &count);

            // Return the count to the caller.
            Ok(count)
        } else {
            // Return an error if the max is exceeded.
            Err(Error::LimitReached)
        }
    }
}
