#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Val, TryFromVal, ConversionError, token, log};

#[derive(Clone, Copy)]
pub enum DataKey {
    AcceptedToken = 0, // address of the accepted token
    DonationsRecipient = 1 // address of the donations recipient
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// Helper functions
fn put_token_address(e: &Env, token: &Address) {
    e.storage().instance().set(&DataKey::AcceptedToken, token);
}

fn put_donations_recipient(e: &Env, recipient: &Address) {
    e.storage().instance().set(&DataKey::DonationsRecipient, recipient);
}

fn get_token_address(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::AcceptedToken)
        .expect("not initialized")
}

fn get_donations_recipient(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::DonationsRecipient)
        .expect("not initialized")
}

fn get_balance(e: &Env, token_address: &Address) -> i128 {
    let client = token::Client::new(e, token_address);
    client.balance(&e.current_contract_address())
}

// Transfer tokens from the contract to the recipient
fn transfer(e: &Env, to: &Address, amount: &i128) {
    let token_contract_address = &get_token_address(e);
    let client = token::Client::new(e, token_contract_address);
    client.transfer(&e.current_contract_address(), to, amount);
}

// Contract Trait
pub trait DonationsTrait {
    // Sets the recipient address and the token that will be accepted as a donation
    fn initialize(e: Env, recipient: Address, token: Address);

    // Donates specified amount units of the accepted token
    fn donate(e: Env, donor: Address, amount: i128);

    // Transfers all the donated amounts to the recipient. Can be called by anyone.
    fn withdraw(e: Env);

    // Get the token address that is accepted for donations.
    fn token(e: Env) -> Address;

    // Get the donations recipient address.
    fn recipient(e: Env) -> Address;
}

#[contract]
struct Donations;

#[contractimpl]
impl DonationsTrait for Donations {
    // Sets the recipient address and the token that will be accepted as a donation
    fn initialize(e: Env, recipient: Address, token: Address) {
        assert!(
            !e.storage().instance().has(&DataKey::AcceptedToken),
            "already initialized"
        );

        put_token_address(&e, &token);
        put_donations_recipient(&e, &recipient);
    }

    // Donates amount units of the accepted token
    fn donate(e: Env, donor: Address, amount: i128) {
        donor.require_auth();

        let token_address = get_token_address(&e);
        let client = token::Client::new(&e, &token_address);
        client.transfer(&donor, &e.current_contract_address(), &amount);
    }

    // Transfer all donation amounts to recipient. Can be called by anyone.
    fn withdraw(e: Env) {
        let token = get_token_address(&e);
        let recipient = get_donations_recipient(&e);

        transfer(&e, &recipient, &get_balance(&e, &token));

        /// @dev should remove logs before deploying smart contracts
        log!(&e, "Token: {}, Recipient: {}", token, recipient);
    }

    // Get the token address that is accepted as donations
    fn token(e: Env) -> Address {
        get_token_address(&e)
    }

    // Get the donations recipient address
    fn recipient(e: Env) -> Address {
        get_donations_recipient(&e)
    }
}
