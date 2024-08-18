#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contractimpl, Env, Vec, log};

#[contract]
pub struct LoopExample;

#[contractimpl]
impl LoopExample {
    /// This function takes a vector of i32 values, iterates through each element, and returns the sum.
    /// It checks for overflow to ensure safe operations and minimizes loop iterations to avoid excessive gas consumption.
    pub fn sum_elements(env: Env, values: Vec<i32>) -> i32 {
        let mut sum: i32 = 0; // i32 is used, be mindful of its limits: -2,147,483,648 to 2,147,483,647
        let length: u32 = values.len();

        // Iterate over the vector and sum its elements
        for i in 0..length {
            // Fetch the value at index `i`
            let value: i32 = values.get(i).unwrap_or(0);

            // Check for overflow
            match sum.checked_add(value) {
                Some(new_sum) => sum = new_sum,
                None => {
                    log!(&env, "Overflow detected, returning current sum: {}", sum);
                    break;
                }
            }
        }

        log!(&env, "Final sum: {}", sum);
        sum
    }

    /// A demonstration of how you can loop through a Vec<u32> and do some operation.
    /// It also shows how to use the loop and break patterns with consideration for gas fees.
    pub fn count_even_numbers(env: Env, values: Vec<u32>) -> u32 {
        let mut count: u32 = 0;
        let length: u32 = values.len();

        for i in 0..length {
            let value: u32 = values.get(i).unwrap_or(0);

            if value % 2 == 0 {
                count += 1;

                // Break if count reaches a high number to avoid excessive gas fees
                if count > 1000 {
                    log!(&env, "Count exceeds 1000, stopping early to save gas.");
                    break;
                }
            }
        }

        log!(&env, "Total even numbers: {}", count);
        count
    }

    /// Allocates a temporary vector holding values (1..=count), then computes and returns their product.
    /// If any overflow occurs, the function logs the current product and stops.
    pub fn multiply(_env: Env, count: u32) -> u128 {
        let mut v1 = alloc::vec![]; // Use alloc crate to create a temporary vector
        (1..=count).for_each(|i| v1.push(i));

        let mut product: u128 = 1; // Use u128 to handle larger multiplication results
        for i in v1 {
            // Check for overflow
            match product.checked_mul(i as u128) {
                Some(new_product) => product = new_product,
                None => {
                    // Overflow detected, return current product
                    log!(&_env, "Overflow detected, returning current product: {}", product);
                    break;
                }
            }
        }

        log!(&_env, "Final product: {}", product);
        product
    }
}