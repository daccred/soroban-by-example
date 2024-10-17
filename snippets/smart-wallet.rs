#![no_std]

#[contract]
struct SmartWallet;

use soroban_sdk::{auth::Context, contract, contractimpl, contracttype, BytesN, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
}

#[contractimpl]
impl SmartWallet {
    // Initialize the contract with an owner's ed25519 public key.
    pub fn init(env: Env, public_key: BytesN<32>) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Owner is already set");
        }

        env.storage().instance().set(&DataKey::Owner, &public_key);
    }

    // Verifies the contract is authorized by the owner.
    #[allow(non_snake_case)]
    pub fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signature: BytesN<64>,
        _auth_context: Vec<Context>,
    ) {
        let public_key: BytesN<32> = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::Owner)
            .unwrap();

        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
    }
}
