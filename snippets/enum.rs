#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct CustomDataTypes;

#[contractimpl]
impl CustomDataTypes {
    pub fn enum_type(env: Env) {
        
        // Define an enum with unit and tuple variants
        pub enum SimpleEnum {
            A,
            B(u32),
        }

        // Example usage of SimpleEnum
        let value_a = SimpleEnum::A;
        let value_b = SimpleEnum::B(42);

        match value_a {
            SimpleEnum::A => log!(&env, "SimpleEnum::A"),
            SimpleEnum::B(val) => log!(&env, "SimpleEnum::B with value: {}", val),
        }

        match value_b {
            SimpleEnum::A => log!(&env, "SimpleEnum::A"),
            SimpleEnum::B(val) => log!(&env, "SimpleEnum::B with value: {}", val),
        }

        // ------------------------------------

        
        // Define an enum with integer variants
        pub enum IntEnum {
            A = 0,
            B = 1,
        }

        // Example usage of IntEnum
        let int_value = IntEnum::B;

        match int_value {
            IntEnum::A => log!(&env, "IntEnum::A with value 0"),
            IntEnum::B => log!(&env, "IntEnum::B with value 1"),
        }
    }
}
