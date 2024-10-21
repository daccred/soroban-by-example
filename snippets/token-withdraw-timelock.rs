#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec, log};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// `timelock`: Check that provided timestamp is before or after
// the provided timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        /// @dev should remove logs before deploying smart contracts
        log!(&env, "There are {} number of claimants", claimants.len());

        if claimants.len() > 10 {
            panic!("Too many claimants");
        }

        if is_initialized(&env) {
            panic!("Contract has already been initialized");
        }

        // The 'from' address authorizes the deposit call
        from.require_auth();

        // Transfer to the specified contract address
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        // Storage transaction info for claimant
        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                claimants,
                time_bound,
            }
        );

        // Mark contract as initialized to prevent double-usage.
        env.storage().instance().set(&DataKey::Balance, &());
    }

    pub fn claimant(env: Env, claimant: Address) {
        // Verify claimant identity for transaction authorization
        claimant.require_auth();

        // Retrieve claimable balance
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&DataKey::Balance).unwrap();

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Claimable balance: {}", claimable_balance);

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("Time requirement not fulfilled");
        }

        let claimants = &claimable_balance.claimants;

        if !claimants.contains(&claimant) {
            panic!("Claimant supplied not allowed to claim this balance");
        }

        // Transfer authorized sum to validated claimant
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );

        // Remove claimed balance to prevent further claims
        env.storage().instance().remove(&DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}
