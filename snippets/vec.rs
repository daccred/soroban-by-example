#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, log, Val};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    pub fn vector_type(env: Env) -> Vec<i32>{


         // 1. Creating a New Empty Vec
        let mut my_vec: Vec<i32> = Vec::new(&env);
        
        // Also Creates a Vec with elements 5, 6, 7, 8 using the vec! macro
        // let vec_macro: Vec<i32> = vec![&env, 5, 6, 7, 8]; 

        // 2. Add elements to the Vec (using from_array)
        my_vec = Vec::from_array(&env, [1, 2, 3]);

        // 3. Get the Length of the Vec
        let length: u32 = my_vec.len();
        log!(&env, "Length of vec: {}", length);

        // 4. Check if the Vec is Empty
        let is_empty: bool = my_vec.is_empty();
        log!(&env, "Is vec empty?: {}", is_empty);

        // 5. Get an Element by Index
        let value: Option<i32> = my_vec.get(1);
        log!(&env, "Value at index 1: {}", value);

        // 6. Set an Element by Index
        my_vec.set(1, 10);

        // 7. Insert an Element at a Specific Index
        my_vec.insert(1, 99);

        // 8. Remove an Element by Index
        my_vec.remove(1);

        // 9. Push an Element to the Back
        my_vec.push_back(4);

        // 10. Push an Element to the Front
        my_vec.push_front(0);

        // 11. Pop an Element from the Back
        let popped_back: Option<i32> = my_vec.pop_back();
        log!(&env, "Popped from back: {}", popped_back);

        // 12. Pop an Element from the Front
        let popped_front: Option<i32> = my_vec.pop_front();
        log!(&env, "Popped from front: {}", popped_front);

        // 13. Slice a Vec
        let slice: Vec<i32> = my_vec.slice(1..2);
        log!(&env, "Slice of vec: {:?}", slice);

        // 14. Check if Vec Contains an Element
        let contains_value: bool = my_vec.contains(&2);
        log!(&env, "Does vec contain 2?: {}", contains_value);

        // 15. Getting the First Index of an Element
        let first_index: Option<u32> = my_vec.first_index_of(&2);
        log!(&env, "First index of 2: {}", first_index);

        // 16. Get the Last Index of an Element
        let last_index: Option<u32> = my_vec.last_index_of(&2);
        log!(&env, "Last index of 2: {}", last_index);

        // 17. Binary Search for an Element
        // let search_result: Result<u32, u32> = my_vec.binary_search(&2);
        // log!(&env, "Binary search result for 2: {}", search_result);

        // 18. Concatenate Multiple Vecs
        let concatenated_vec = vec![&env, vec![&env, 1, 2], vec![&env, 3, 4]].concat();
        log!(&env, "Concatenated vec: {}", concatenated_vec);

        // 19. Iterate Over a Vec
        for item in my_vec.iter() {
            log!(&env, "Iterating value: {}", item);
        }

        // 20. Convert Vec to a List of Vals
        let vals: Vec<Val> = my_vec.to_vals();
        log!(&env, "Converted vals: {}", vals);

        // Return the final Vec
        my_vec


    }
}