#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, BytesN, Env, log};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Admin,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
}

#[contract]
pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Admin address: {}", admin);

        Ok(())
    }

    pub fn version() -> u32 {
        1
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        admin.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
