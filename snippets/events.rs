#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

// Define a constant symbol for the counter name
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterEvent;

// Implement the functionality for CounterEvent
#[contractimpl]
impl CounterEvent {
  /// This function increments a counter and returns the new value.

  pub fn increment(env: Env) -> u32 {
    // Get the current counter value. If it doesn't exist, assume it's 0.
    let mut count = env.storage().instance().get(&COUNTER).unwrap_or(0);

    // Increase the count by 1.
    count += 1;

    // Save the new count in storage.
    env.storage().instance().set(&COUNTER, &count);

    // Announce that the counter has been incremented.
    // This includes the counter name, "increment" as the event type,
    // and the new count as data.
    const INCREMENT: Symbol = symbol_short!("increment");

    env.events().publish((COUNTER, INCREMENT), count);

    // Return the new counter value.
    count
  }
}
