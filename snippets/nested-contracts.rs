#![no_std]

pub mod contract_a {
    use soroban_sdk::{
        auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
        contract, contractimpl, vec, Env, Address, Symbol, IntoVal, log
    };

    use crate::contract_b::ContractBClient;

    #[contract]
    pub struct ContractA;

    #[contractimpl]
    impl ContractA {
        pub fn call_b(env: Env, contract_b_address: Address, contract_c_address: Address) {
            /// @dev should remove logs before deploying smart contracts
            log!(&env, "Contract B address: {}, Contract C address: {}", contract_b_address, contract_c_address);

            env.authorize_as_current_contract(vec![
               &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                       contract: contract_c_address.clone(),
                       fn_name: Symbol::new(&env, "authorized_fn_c"),
                       args: (env.current_contract_address(),).into_val(&env),
                    },

                    sub_invocations: vec![&env],
                }),
            ]);
        }
    }
}

pub mod contract_b {
    use soroban_sdk::{contract, contractimpl, Env, Address, log};

    use crate::contract_c::ContractCClient;

    #[contract]
    pub struct ContractB;

    #[contractimpl]
    impl ContractB {
        pub fn authorized_fn_b(env: Env, authorizer: Address, contract_c_address: Address) {
            log!(&env, "Authorizer: {}, Contract C address: {}", authorizer, contract_c_address);

            authorizer.require_auth();
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }
    }
}

pub mod contract_c {
    use soroban_sdk::{contract, contractimpl, Env, Address, log};

    #[contract]
    pub struct ContractC;

    #[contractimpl]
    impl ContractC {
        pub fn authorized_fn_c(_env: Env, authorizer: Address) {
            /// @dev should remove logs before deploying smart contracts
            log!(&env, "Authorizer: {}", authorizer);

            authorizer.require_auth();
        }
    }
}
