#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    panic_with_error, xdr::ToXdr, Address, Bytes, BytesN,
    Env, InvokeError, Symbol, TryFromVal, Val, Vec
};


const DONE_TIMESTAMP: u64 = 1;
pub const MAX_MIN_DELAY: u64 = 30 * 24 * 60 * 60; // 30 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Scheduler(BytesN<32>),
    MinDelay,
    Initialized,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
#[repr(u8)]
enum OperationState {
    Unset = 1,
    Waiting = 2,
    Ready = 3,
    Executed = 4,
}

#[derive(Copy, Clone)]
#[contracterror]
#[repr(u32)]
pub enum TimeLockError {
    InvalidParams = 0,
    NotInitialized = 1,
    AlreadyInitialized = 2,
    AlreadyExists = 3,
    InsufficientDelay = 4,
    TimeNotReady = 5,
    PredecessorNotDone = 6,
    InvalidStatus = 8,
    NotPermitted = 9,
    ExecuteFailed = 10,
    InvalidFuncName = 11,
    DelayTooLong = 12,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CallExecutedEvent {
    pub opt_id: BytesN<32>,
    pub target: Address,
    pub fn_name: Symbol,
    pub data: Vec<Val>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CallScheduledEvent {
    pub opt_id: BytesN<32>,
    pub target: Address,
    pub fn_name: Symbol,
    pub data: Vec<Val>,
    pub predecessor: BytesN<32>,
    pub delay: u64,
}




#[contract]
pub struct TimeLockContract;

#[contractimpl]
impl TimeLockContract {
    pub fn initialize(
        e: Env,
        min_delay: u64,
    ) {
        if min_delay > MAX_MIN_DELAY {
            panic_with_error!(e, TimeLockError::DelayTooLong);
        }

        if !e.storage().instance().has(&DataKey::Initialized) {
            e.storage().instance().set(&DataKey::Initialized, &true);
        } else {
            panic_with_error!(e, TimeLockError::AlreadyInitialized);
        }

        e.storage().instance().set(&DataKey::MinDelay, &min_delay);
    }

    pub fn schedule(
        e: Env,
        target: Address,
        fn_name: Symbol,
        data: Vec<Val>,
        salt: BytesN<32>,
        predecessor: Option<BytesN<32>>,
        delay: u64,
    ) -> BytesN<32> {
        if delay < e.storage().instance().get(&DataKey::MinDelay).unwrap() {
            panic_with_error!(e, TimeLockError::InsufficientDelay);
        }

        let operation_id = Self::hash_call(&e, &target, &fn_name, &data, &salt, &predecessor);
        Self::add_operation(&e, &operation_id, delay);

        let actual_predecessor = match predecessor {
            Some(predecessor) => predecessor,
            None => BytesN::from_array(&e, &[0_u8; 32]),
        };

        e.events().publish(
            (Symbol::new(&e, "CallScheduled"),),
            CallScheduledEvent {
                opt_id: operation_id.clone(),
                target,
                fn_name,
                data,
                predecessor: actual_predecessor,
                delay,
            },
        );

        operation_id
    }

    pub fn execute(
        e: Env,
        target: Address,
        fn_name: Symbol,
        data: Vec<Val>,
        salt: BytesN<32>,
        predecessor: Option<BytesN<32>>,
        is_native: bool,
    ) {
        let operation_id = Self::hash_call(&e, &target, &fn_name, &data, &salt, &predecessor);
        Self::check_execute(&e, &operation_id, &predecessor);

        if is_native {
            Self::exec_native(&e, &fn_name, &data);
        } else {
            Self::exec_external(&e, &target, &fn_name, &data);
        }

        e.storage()
            .persistent()
            .set(&DataKey::Scheduler(operation_id.clone()), &DONE_TIMESTAMP);

        e.events().publish(
            (Symbol::new(&e, "CallExecuted"),),
            CallExecutedEvent {
                opt_id: operation_id,
                target,
                fn_name,
                data,
            },
        );
    }

    pub fn cancel(e: Env, operation_id: BytesN<32>) {
        let state = Self::get_operation_state(&e, &operation_id);
        if state == OperationState::Ready || state == OperationState::Waiting {
            e.storage()
                .persistent()
                .remove(&DataKey::Scheduler(operation_id.clone()));
        } else {
            panic_with_error!(e, TimeLockError::InvalidStatus);
        }

        e.events().publish(
            (Symbol::new(&e, "OperationCancelled"),),
            operation_id,
        );
    }

    pub fn update_min_delay(e: Env, delay: u64) {
        e.storage().instance().set(&DataKey::MinDelay, &delay);
        e.events()
            .publish((Symbol::new(&e, "MinDelayUpdated"),), delay);
    }

    pub fn get_schedule_lock_time(e: Env, operation_id: BytesN<32>) -> u64 {
        let key = DataKey::Scheduler(operation_id.clone());
        if let Some(schedule) = e.storage().persistent().get::<DataKey, u64>(&key) {
            schedule
        } else {
            0_u64
        }
    }

    fn get_operation_state(e: &Env, operation_id: &BytesN<32>) -> OperationState {
        let ledger_time = e.ledger().timestamp();
        let lock_time = Self::get_schedule_lock_time(e.clone(), operation_id.clone());
        if lock_time == 0 {
            OperationState::Unset
        } else if lock_time == DONE_TIMESTAMP {
            OperationState::Executed
        } else if ledger_time < lock_time {
            OperationState::Waiting
        } else {
            OperationState::Ready
        }
    }

    fn add_operation(e: &Env, operation_id: &BytesN<32>, delay: u64) {
        let ledger_time = e.ledger().timestamp();
        if Self::get_operation_state(e, operation_id) != OperationState::Unset {
            panic_with_error!(e, TimeLockError::AlreadyExists);
        }

        let time = ledger_time + delay;
        e.storage()
            .persistent()
            .set(&DataKey::Scheduler(operation_id.clone()), &time);
    }

    fn check_execute(e: &Env, operation_id: &BytesN<32>, predecessor: &Option<BytesN<32>>) {
        if Self::get_operation_state(e, operation_id) != OperationState::Ready {
            panic_with_error!(e, TimeLockError::TimeNotReady);
        }

        if let Some(predecessor) = predecessor {
            if Self::get_operation_state(e, predecessor) != OperationState::Executed {
                panic_with_error!(e, TimeLockError::PredecessorNotDone);
            }
        }
    }

    fn exec_external(e: &Env, target: &Address, fn_name: &Symbol, data: &Vec<Val>) {
        let result = e.try_invoke_contract::<(), InvokeError>(target, fn_name, data.clone());

        match result {
            Ok(_) => {}
            Err(_) => {
                panic_with_error!(e, TimeLockError::ExecuteFailed);
            }
        }
    }

    fn exec_native(e: &Env, fn_name: &Symbol, data: &Vec<Val>) {
        let fn_name = fn_name.clone();
        if fn_name == Symbol::new(&e, "update_min_delay") {
            Self::update_min_delay_from_data(&e, data);
        } else {
            panic_with_error!(e, TimeLockError::InvalidFuncName);
        }
    }

    fn hash_call(
        e: &Env,
        target: &Address,
        fn_name: &Symbol,
        data: &Vec<Val>,
        salt: &BytesN<32>,
        predecessor: &Option<BytesN<32>>,
    ) -> BytesN<32> {
        let mut calldata = Bytes::new(e);

        // Clone the values where necessary to avoid moving out of borrowed values.
        calldata.append(&target.clone().to_xdr(e));
        calldata.append(&fn_name.clone().to_xdr(e));
        calldata.append(&data.clone().to_xdr(e)); // Clone `data` to satisfy ownership rules

        // Handle the predecessor value by cloning it if it exists.
        if let Some(predecessor) = predecessor {
            calldata.append(&predecessor.clone().to_xdr(e));
        }

        calldata.append(&salt.clone().to_xdr(e)); // Clone `salt` to satisfy ownership rules

        // Convert the result of sha256 to BytesN<32>
        e.crypto().sha256(&calldata).into()
    }


    fn update_min_delay_from_data(e: &Env, data: &Vec<Val>) {
        let delay = data.get(0);
        if let Some(delay) = delay {
            let p = u64::try_from_val(e, &delay);
            if let Ok(delay) = p {
                Self::update_min_delay(e.clone(), delay);
            } else {
                panic_with_error!(e, TimeLockError::InvalidParams);
            }
        } else {
            panic_with_error!(e, TimeLockError::InvalidParams);
        }
    }
}
