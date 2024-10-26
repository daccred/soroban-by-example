#![no_std]

use soroban_sdk::{
  contract, contractimpl, contracterror, contracttype, Address, Env, Symbol, Val, Vec, log
};

#[contracttype]
pub enum DataKey {
    GovState,
    Proposal(u32),
    Vote(u32, Address),
}

#[derive(Default)]
#[contracttype]
pub struct Vote {
    active: bool,
    value: bool,
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Proposal {
    pub executed: bool,
    pub expiration_date: u32, // The timestamp when the proposal can no longer be voted on.
    pub positive_votes: u32,
    pub negative_votes: u32,
    pub tx: Transaction,
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct Transaction {
    pub contract_id: Address, // The address of the contract the tx will invoke if approved.
    pub function: Symbol, // The function to be invoked if approved.
    pub func_arguments: Vec<Val>, // The parameters to the function.
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct GovState {
    next_tx_id: u32, // Next proposal id to be assigned.
    supermajority: bool, // Indicates whether the amount of positive votes has to be higher than 50%.
    supermajority_percentage: Option<u32>, // Indicates the minimum percentage the amount of positive votes has to reach. For example, 80% over total votes.
    voting_period: u32, // Indicates how much time the voting of a proposal remains open from the moment of its creation.
    whitelist: Vec<Address>,
    quorum: u32,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum GovError {
    InvalidSupermajorityPercentage = 1,
    GovernanceAlreadyInitialized = 2,
    InvalidQuorumValue = 3,
    AddressNotInWhitelist = 4,
    ProposalAlreadyExecuted = 5,
    UserAlreadyVoted = 6,
    InvalidProposalId = 7,
    QuorumNotReached = 8,
    GovernanceNotInitialized = 9,
    WhitelistMustNotBeEmpty = 10,
}

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    pub fn initialize(
        env: Env,
        supermajority: bool,
        supermajority_percentage: Option<u32>,
        voting_period: u32,
        whitelist: Vec<Address>,
        quorum: u32,
    ) -> Result<(), GovError> {
        let state = Self::get_state(env.clone());

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "State output: {}", state);

        if state.is_ok() {
            return Err(GovError::GovernanceAlreadyInitialized);
        }

        if !(whitelist.len() > 0) {
            return Err(GovError::WhitelistMustNotBeEmpty)
        }

        if !(quorum > 0 && quorum <= 100) {
            return Err(GovError::InvalidQuorumValue);
        }

        if supermajority {
            match supermajority_percentage {
                Some(x) => assert!(x <= 100 && x > 50),
                None => return Err(GovError::InvalidSupermajorityPercentage),
            }
        }

        let new_gov_state = GovState {
            next_tx_id: 0,
            supermajority,
            supermajority_percentage,
            voting_period,
            whitelist,
            quorum,
        };

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "New state: {}", new_gov_state);

        env.storage().instance().set(&DataKey::GovState, &new_gov_state);

        Ok(())
    }

    pub fn propose_tx(
        env: Env,
        contract_id: Address,
        func_name: Symbol,
        func_args: Vec<Val>,
        caller: Address,
    ) -> Result<(), GovError> {
        let mut state = Self::get_state(env.clone())?;

        if !Self::whitelisted(state.clone(), caller.clone()) {
            return Err(GovError::AddressNotInWhitelist);
        }

        let now = env.ledger().sequence();

        let proposal_id = state.next_tx_id;

        state.next_tx_id += 1;

        let new_tx = Transaction {
            contract_id,
            function: func_name,
            func_arguments: func_args,
        };

        let new_proposal = Proposal {
            executed: false,
            expiration_date: now + state.voting_period,
            positive_votes: 0,
            negative_votes: 0,
            tx: new_tx,
        };

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &new_proposal);

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "New proposal: {}", new_proposal);

        env.storage().instance().set(&DataKey::GovState, &state);

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "State: {}", state);

        Ok(())
    }

    pub fn vote_proposal(
        env: Env,
        voter: Address,
        proposal_id: u32,
        vote_value: bool,
    ) -> Result<(), GovError> {
        let state = Self::get_state(env.clone())?;

        let mut proposal = Self::get_proposal(env.clone(), proposal_id)?;

        let now = env.ledger().sequence();

        if !Self::whitelisted(state.clone(), voter.clone()) {
            return Err(GovError::AddressNotInWhitelist);
        }

        voter.require_auth();

        assert!(now <= proposal.expiration_date);

        let vote = Self::get_vote(env.clone(), proposal_id.clone(), voter.clone());

        if vote.active {
            return Err(GovError::UserAlreadyVoted);
        }

        env.storage().instance().set(
            &DataKey::Vote(proposal_id, voter),
            &Vote {
                active: true,
                value: vote_value,
            },
        );

        if vote_value {
            proposal.positive_votes += 1;
        } else {
            proposal.negative_votes += 1;
        }

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Proposal value: {}", proposal);

        Ok(())
    }

     pub fn close_proposal(
         env: Env,
         proposal_id: u32
     ) -> Result<(), GovError> {
        let state = Self::get_state(env.clone())?;

        let proposal = Self::get_proposal(env.clone(), proposal_id)?;

        assert!(proposal.expiration_date < env.ledger().sequence());

        if proposal.executed {
            return Err(GovError::ProposalAlreadyExecuted);
        }

        let total_votes = proposal.positive_votes + proposal.negative_votes;

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Total votes: {}", total_votes);

        let min_votes_multiplied_100 = state.whitelist.len() * state.quorum;

        if total_votes * 100 < min_votes_multiplied_100 {
            return Err(GovError::QuorumNotReached);
        }

        match state.supermajority {
            true => {
                let percentage = state.supermajority_percentage.unwrap();

                if proposal.positive_votes * 100 >= (total_votes * percentage) {
                    let _res: Val = env.invoke_contract(
                        &proposal.tx.contract_id,
                        &proposal.tx.function,
                        proposal.tx.func_arguments,
                    );
                }

            }

            false => {
                if proposal.positive_votes > proposal.negative_votes {
                    let _res: Val = env.invoke_contract(
                        &proposal.tx.contract_id,
                        &proposal.tx.function,
                        proposal.tx.func_arguments,
                    );
                }

            }
        };

        Ok(())
    }


    pub fn get_state(
        env: Env
    ) -> Result<GovState, GovError> {
        let state_op = env.storage().instance().get(&DataKey::GovState);

        if state_op.is_some() {
            Ok(state_op.unwrap())
        } else {
            Err(GovError::GovernanceNotInitialized)
        }
    }

    pub fn get_proposal(
        env: Env,
        proposal_id: u32
    ) -> Result<Proposal, GovError> {
        let state = Self::get_state(env.clone())?;

        if proposal_id >= state.next_tx_id {
            return Err(GovError::InvalidProposalId);
        }

        let proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Proposal: {}", proposal);

        Ok(proposal)
    }

    pub fn whitelisted(
        state: GovState,
        caller: Address
    ) -> bool {
        state.whitelist.contains(caller)
    }


    pub fn get_vote(
        env: Env,
        proposal_id: u32,
        voter: Address
    ) -> Vote {
        env.storage()
        .instance()
        .get(&DataKey::Vote(proposal_id, voter))
        .unwrap_or_default()
    }
}
