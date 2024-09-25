#![no_std]
use soroban_sdk::{contract, contractimpl, Env, log};

extern crate alloc;

use alloc::vec::Vec;

#[contract]
pub struct TransientStorage;

#[contractimpl]
impl TransientStorage {
    /// Allocates a temporary vector holding the values 0 to the set count-1, performs a computation and returns the sum.
    pub fn sum(env: Env, count: u32) {
        // Create a new empty vector to store the values.
        let mut values = Vec::new();

        // Perform the vector with values from 0 to count-1.
        for i in 0..count {
            values.push(i);
        }

        // Calculate the sum of the values in the vector.
        let sum: u32 = values.iter().sum();

        // Return the calculated value.
        log!(&env, "Sum of values: {}", sum);
    }
}
