#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Map, log};

#[contract]
pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    pub fn map_type(env: Env) -> Map<u32, u32> {

        // 1. Create a new Map
        let mut my_map: Map<u32, u32> = Map::new(&env);

        // or create a new map with initial values
        // let my_map_2 = Map::from_array(&env, [(2, 20), (1, 10)]); 
        log!(&env, "Map: {}", my_map);

        
        
        // 2. Add some key-value pairs
        my_map.set(1, 10);
        my_map.set(2, 20);
        my_map.set(3, 30);

        // 3. Log the length of the Map
        log!(&env, "Length of map: {}", my_map.len());

        // 4. Retrieve a value by key
        if let Some(value) = my_map.get(2) {
            log!(&env, "Value for key 2: {}", value);
        } else {
            log!(&env, "Key 2 not found");
        }

        // 5. Check if a key exists
        let key_exists = my_map.contains_key(3);
        log!(&env, "Key 3 exists: {}", key_exists);

        // 6. Remove a key-value pair
        my_map.remove(1);
        log!(&env, "After removing key 1, length of map: {}", my_map.len());

        // 7. Return the map
        my_map

    }
}