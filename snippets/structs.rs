#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct CustomDataTypes;

#[contractimpl]
impl CustomDataTypes {
    pub fn struct_type(env: Env) {
        
        // Struct with named fields
        pub struct StateNamed {
            pub count: u32,
            pub last_incr: u32,
        }

        // Creating an instance of StateNamed
        let state_named = StateNamed {
            count: 10,
            last_incr: 5,
        };

        log!(&env, "StateNamed - count: {}, last_incr: {}", state_named.count, state_named.last_incr);

        // Struct with unnamed fields (tuple struct)
        pub struct StateTuple(u32, u32);

        // Creating an instance of StateTuple
        let state_tuple = StateTuple(20, 10);
        log!(&env, "StateTuple - first: {}, second: {}", state_tuple.0, state_tuple.1);

        // Example of a different tuple struct
        struct Tuple(u32, String);

        // Creating an instance of Tuple
        let tuple = Tuple(42, String::from_str(&env, "Hello"));
        log!(&env, "Tuple - number: {}, text: {}", tuple.0, tuple.1);
    }
}
