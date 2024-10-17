#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Val, Vec, log};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    /// Deploy the Wasm contract and invoke an init function
    /// with the given arguments afterwards.
    ///
    /// This has to be authorized by `deployer` (Unless the `Deployer` instance
    /// is used itself as the deployer). This way the whole process is atomic
    /// and it is not possible to frontrun the contract initialization.
    ///
    /// Returns the result of the init function and the contract ID.

    pub fn deploy (
        env: Env,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>,
    ) -> (Address, Val) {
        // Skip authorization if deployer is the current contract
        if deployer != env.current_contract_address() {
            deployer.require_auth();
        }

        // Deploy the contract using the uploaded Wasm with the given hash
        let deployed_address = env
            .deployer()
            .with_address(deployer, salt)
            .deploy(wasm_hash);

        // Invoke the init function with the given arguments
        let res: Val = env.invoke_contract(&deployed_address, &init_fn, init_args);

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Deployed address: {}, Init invocation result: {}", deployed_address, res);

        // Return the address of the deployed contract and the
        // result of invoking the init result.

        (deployed_address, res)
    }
}
