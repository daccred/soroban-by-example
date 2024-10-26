#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contract]
pub struct VestingContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub token: Address,
    pub beneficiary: Address,
    pub admin: Address,
    pub start_time: u64,
    pub end_time: u64,
    pub locked: i128,
    pub paid_out: i128,
}

impl State {
    pub fn new(
        token: Address,
        beneficiary: Address,
        admin: Address,
        start_time: u64,
        end_time: u64,
    ) -> State {
        State {
            token,
            beneficiary,
            admin,
            start_time,
            end_time,
            locked: 0,
            paid_out: 0,
        }
    }
}

#[contracttype]
pub enum DataKey {
    NextId,
    State(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VestError {
    InvalidDuration = 1,
    ArithmeticError = 2,
    InvalidAmount = 3,
}

#[contractimpl]
impl VestingContract {
    pub fn new_vesting(
        env: Env,
        token: Address,
        beneficiary: Address,
        start_time: u64,
        duration: u64,
        admin: Address,
    ) -> Result<u64, VestError> {
        if duration < 1 {
            return Err(VestError::InvalidDuration);
        }

        let end_time = start_time
            .checked_add(duration)
            .ok_or(VestError::InvalidDuration)?;

        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .unwrap_or_default();
        let state = State::new(token, beneficiary, admin, start_time, end_time);

        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        VestingContract::save_state(env, id, &state);

        Ok(id)
    }

    fn save_state(env: Env, id: u64, state: &State) {
        env.storage().persistent().set(&DataKey::State(id), state);
    }

    fn get_state(env: &Env, id: u64) -> State {
        env.storage().persistent().get(&DataKey::State(id)).unwrap()
    }

    fn time(env: &Env, state: &State) -> u64 {
        let now = env.ledger().timestamp();
        if now <= state.start_time {
            return 0;
        }
        if now >= state.end_time {
            return state.end_time - state.start_time;
        }
        now - state.start_time
    }

     fn retrievable_balance_internal(env: &Env, state: &State) -> Result<i128, VestError> {
        let now = VestingContract::time(env, state);
        if now == 0 {
            return Ok(0);
        }

        let duration: i128 = (state.end_time - state.start_time).into();

        //total is the amount the amount the beneficiary is owed at this time if they never cashed out.
        let total = state
            .locked
            .checked_mul(now.into())
            .and_then(|res| res.checked_div(duration))
            .ok_or(VestError::ArithmeticError)?;state
            .locked
            .checked_mul(now.into())
            .and_then(|res| res.checked_div(duration))
            .ok_or(VestError::ArithmeticError)?;

        //Subtract from total the amount that the beneficiary has already cashed
        //out to obtain how much they're owed.
        Ok(total
            .checked_sub(state.paid_out)
            .ok_or(VestError::ArithmeticError)?)
    }

    pub fn retrievable_balance(env: Env, id: u64) -> Result<i128, VestError> {
        VestingContract::retrievable_balance_internal(&env, &VestingContract::get_state(&env, id))
    }

    pub fn add_vest(
        env: Env,
        id: u64,
        token: Address,
        from: Address,
        amount: i128,
    ) -> Result<i128, VestError> {
        let mut state = VestingContract::get_state(&env, id);
        if token != state.token {
            panic!("token doesn't match!");
        }

        state.admin.require_auth();
        state.locked = state
            .locked
            .checked_add(amount)
            .ok_or(VestError::ArithmeticError)?;

        from.require_auth();
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        VestingContract::save_state(env, id, &state);

        Ok(state.locked)
    }

    pub fn pay_out(env: Env, id: u64) -> Result<i128, VestError> {
        let mut state = VestingContract::get_state(&env, id);

        let available = VestingContract::retrievable_balance_internal(&env, &state)?;
        if available == 0 {
            return Ok(0);
        }

        state.paid_out = state
            .paid_out
            .checked_add(available)
            .ok_or(VestError::ArithmeticError)?;
        token::Client::new(&env, &state.token).transfer(
            &env.current_contract_address(),
            &state.beneficiary,
            &available,
        );

        VestingContract::save_state(env, id, &state);

        Ok(available)
    }
}
