#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env, Vec};

#[contracttype]
pub enum DataKey {
    MultisigState,
    ProposedTx(u32),
    Confirmation(u32, Address),
    MemberConfirmation(u32, Address),
    MemberModification(Address),
    ChangeReqSigs(u32),
    ReqSigsConf(Address),
}

#[derive(Default)]
#[contracttype]
pub struct ReqSigsConf {
    change_req_sigs_id: u32,
    active: bool,
}

#[derive(Default, Debug)]
#[contracttype]
pub struct ChangeReqSigs {
    new_requirement: u32,
    confirmation_count: u32,
    active: bool,
    expiration: u32,
}

#[derive(Default)]
#[contracttype]
pub struct MemberConfirmation {
    active: bool,
}

#[derive(Default)]
#[contracttype]
pub struct MemberModification {
    modification_id: u32,
    active: bool,
    addition: bool,
    confirmation_count: u32,
}

#[derive(Default)]
#[contracttype]
pub struct Confirmation {
    confirmed: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[contracttype]
pub struct ProposedTx {
    // The proposed transaction to be executed
    pub token: Address,        // Token to be transferred
    pub tx_id: u32,            // Transaction ID
    pub transfer_to: Address,  // The receiver of the transfer
    pub transfer_amount: i128, // The amount that will be sent
    pub executed: bool,        // True if the transaction has already been executed
    pub confirmation_count: u32,
}

#[contracttype]
#[derive(Debug)]
pub struct MultisigState {
    owners: Vec<Address>,               // The current owners of the multisig.
    required_signatures: u32, // The amount of needed signatures in order to execute a transaction.
    next_tx_id: u32,          // Next transaction id.
    member_modification_id: u32, // Next id for member modification proposal.
    next_req_sigs_modification_id: u32, // Current id for required signatures modification proposal - There can only be one active proposal at the time.
    req_sigs_mod_expiration: u32, // The amount of time a required signatures modification proposal will be valid for voted before being discarded. (Measured in ledger sequence number.)
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MultisigErr {
    InvalidProposalId = 1,
    MultisigNotInitialized = 2,
    OwnerAlreadyConfirmedTx = 3,
    MultisigAlreadyInitialized = 4,
    OwnersLessThanRequiredSignatures = 5,
    RemovalProposalOngoingForAddress = 6,
    OwnerAlreadyConfirmedModification = 7,
    AdditionProposalOngoingForAddress = 8,
    ProposalAlreadyExecuted = 9,
    InvalidIdForRequiredSigsModification = 10,
    AlreadyOpenSignaturesRequirementModificationProposal = 11,
    CallerIsNotOwner = 12,
    ProposalIsNotActive = 13,
    ProposalAlreadyExpired = 14,
}

#[contract]
pub struct Multisig;

// soroban attribute that exports the public or usable functions from the contracts
#[contractimpl]
impl Multisig {
    pub fn initialize_multisig(
        env: Env,
        owners: Vec<Address>,
        required_signatures: u32,
        req_sigs_mod_expiration: u32,
    ) -> Result<(), MultisigErr> {
        let state = Self::get_multisig_state(env.clone());
        if state.is_ok() {
            return Err(MultisigErr::MultisigAlreadyInitialized);
        }

        if owners.len() < required_signatures {
            return Err(MultisigErr::OwnersLessThanRequiredSignatures);
        }

        let new_ms_state = MultisigState {
            owners,
            required_signatures,
            next_tx_id: 0,
            member_modification_id: 0,
            next_req_sigs_modification_id: 0,
            req_sigs_mod_expiration,
        };
        env.storage()
            .instance()
            .set(&DataKey::MultisigState, &new_ms_state);

        Ok(())
    }

    pub fn approve_owner_addition(
        env: Env,
        new_owner: Address,
        caller: Address,
    ) -> Result<(), MultisigErr> {
        assert!(new_owner != caller);
        assert!(!Self::is_owner(env.clone(), new_owner.clone())?);
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone())?);

        let mut member_conf: MemberConfirmation;
        let mut ms_state = Self::get_multisig_state(env.clone())?;
        let mut member_mod = Self::get_member_modification(env.clone(), new_owner.clone());
        if member_mod.active {
            member_conf = Self::get_member_confirmation(
                env.clone(),
                member_mod.modification_id,
                caller.clone(),
            );

            if member_conf.active {
                return Err(MultisigErr::OwnerAlreadyConfirmedModification);
            }
            // This case should never happen, taking into account that it is verified that a proposal to remove someone that is not a owner does not happen but it is here as a double check
            if !member_mod.addition {
                return Err(MultisigErr::RemovalProposalOngoingForAddress);
            }
        } else {
            member_mod.modification_id = ms_state.member_modification_id;
            ms_state.member_modification_id += 1;
            member_mod.confirmation_count = 0;
            member_mod.active = true;
            member_mod.addition = true;
            member_conf = Self::get_member_confirmation(
                env.clone(),
                member_mod.modification_id,
                caller.clone(),
            );
        }

        member_conf.active = true;
        member_mod.confirmation_count += 1;
        env.storage().instance().set(
            &DataKey::MemberConfirmation(member_mod.modification_id, caller),
            &member_conf,
        );

        if member_mod.confirmation_count == ms_state.required_signatures {
            ms_state = Self::add_owner(env.clone(), new_owner.clone())?;
            member_mod.active = false;
        }

        env.storage()
            .instance()
            .set(&DataKey::MemberModification(new_owner.clone()), &member_mod);
        env.storage()
            .instance()
            .set(&DataKey::MultisigState, &ms_state);

        Ok(())
    }

    fn add_owner(env: Env, new_owner: Address) -> Result<MultisigState, MultisigErr> {
        let mut ms_state = Self::get_multisig_state(env.clone())?;
        ms_state.owners.push_back(new_owner.clone());

        Ok(ms_state)
    }

    pub fn approve_owner_removal(
        env: Env,
        owner: Address,
        caller: Address,
    ) -> Result<(), MultisigErr> {
        assert!(owner != caller);
        assert!(Self::is_owner(env.clone(), owner.clone())?);
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone())?);
        let mut member_conf: MemberConfirmation;
        let mut ms_state = Self::get_multisig_state(env.clone())?;
        let mut member_mod = Self::get_member_modification(env.clone(), owner.clone());
        if member_mod.active {
            member_conf = Self::get_member_confirmation(
                env.clone(),
                member_mod.modification_id,
                caller.clone(),
            );

            if member_conf.active {
                return Err(MultisigErr::OwnerAlreadyConfirmedModification);
            }
            // This case should never happen, taking into account that it is verified that a proposal to add someone that is already an owner does not happen but it is here as a double check
            if member_mod.addition {
                return Err(MultisigErr::AdditionProposalOngoingForAddress);
            }
        } else {
            member_mod.modification_id = ms_state.member_modification_id;
            ms_state.member_modification_id += 1;
            member_mod.confirmation_count = 0;
            member_mod.active = true;
            member_mod.addition = false;
            member_conf = Self::get_member_confirmation(
                env.clone(),
                member_mod.modification_id,
                caller.clone(),
            );
        }

        member_conf.active = true;
        member_mod.confirmation_count += 1;
        env.storage().instance().set(
            &DataKey::MemberConfirmation(member_mod.modification_id, caller),
            &member_conf,
        );

        if member_mod.confirmation_count == ms_state.required_signatures {
            ms_state = Self::remove_owner(env.clone(), owner.clone())?;
            member_mod.active = false;
        }

        env.storage()
            .instance()
            .set(&DataKey::MemberModification(owner.clone()), &member_mod);
        env.storage()
            .instance()
            .set(&DataKey::MultisigState, &ms_state);

        Ok(())
    }

    fn remove_owner(env: Env, owner: Address) -> Result<MultisigState, MultisigErr> {
        let mut ms_state = Self::get_multisig_state(env.clone())?;
        if (ms_state.owners.len() - 1) < ms_state.required_signatures {
            return Err(MultisigErr::OwnersLessThanRequiredSignatures);
        }
        let index = ms_state.owners.iter().position(|x| x == owner).unwrap() as u32;
        ms_state.owners.remove(index);

        Ok(ms_state)
    }

    pub fn submit_tx(
        env: Env,
        token: Address,
        to: Address,
        amount: i128,
        caller: Address,
    ) -> Result<(), MultisigErr> {
        caller.require_auth();
        let mut ms_state = Self::get_multisig_state(env.clone())?;
        assert!(ms_state.owners.contains(caller));
        let tx_id = ms_state.next_tx_id;
        ms_state.next_tx_id += 1;
        let new_tx = ProposedTx {
            token,
            tx_id,
            transfer_to: to,
            transfer_amount: amount,
            executed: false,
            confirmation_count: 0,
        };

        env.storage()
            .instance()
            .set(&DataKey::ProposedTx(tx_id), &new_tx);
        env.storage()
            .instance()
            .set(&DataKey::MultisigState, &ms_state);
        Ok(())
    }

    pub fn confirm_transaction(env: Env, tx_id: u32, owner: Address) -> Result<(), MultisigErr> {
        owner.require_auth();
        assert!(Self::is_owner(env.clone(), owner.clone())?);
        let mut proposed_tx = Self::get_proposed_tx(env.clone(), tx_id)?;

        let mut conf_count = proposed_tx.confirmation_count;
        let mut confirmation = Self::get_confirmation(env.clone(), tx_id, owner.clone());

        if proposed_tx.executed {
            return Err(MultisigErr::ProposalAlreadyExecuted);
        }
        if !confirmation.confirmed {
            conf_count += 1;
            confirmation.confirmed = true;
            proposed_tx.confirmation_count = conf_count;
            env.storage()
                .instance()
                .set(&DataKey::Confirmation(tx_id, owner), &confirmation);
            env.storage()
                .instance()
                .set(&DataKey::ProposedTx(tx_id), &proposed_tx);
        } else {
            return Err(MultisigErr::OwnerAlreadyConfirmedTx);
        }

        Ok(())
    }

    pub fn execute_transaction(env: Env, tx_id: u32) -> Result<(), MultisigErr> {
        let ms_state = Self::get_multisig_state(env.clone())?;
        let mut proposal = Self::get_proposed_tx(env.clone(), tx_id)?;
        assert!(proposal.confirmation_count >= ms_state.required_signatures);
        assert!(!proposal.executed);

        let token_client = token::Client::new(&env, &proposal.token);
        token_client.transfer(
            &env.current_contract_address(),
            &proposal.transfer_to,
            &proposal.transfer_amount,
        );
        proposal.executed = true;

        env.storage()
            .instance()
            .set(&DataKey::ProposedTx(tx_id), &proposal);
        Ok(())
    }

    pub fn is_owner(env: Env, owner: Address) -> Result<bool, MultisigErr> {
        let ms_state = Self::get_multisig_state(env)?;
        Ok(ms_state.owners.contains(owner))
    }

    pub fn propose_required_signatures(
        env: Env,
        required_signatures: u32,
        caller: Address,
    ) -> Result<(), MultisigErr> {
        caller.require_auth();
        if !Self::is_owner(env.clone(), caller.clone())? {
            return Err(MultisigErr::CallerIsNotOwner);
        }

        let mut state = Self::get_multisig_state(env.clone())?;

        if required_signatures > state.owners.len() {
            return Err(MultisigErr::OwnersLessThanRequiredSignatures);
        }
        let next_id = state.next_req_sigs_modification_id;
        let new_proposal = ChangeReqSigs {
            new_requirement: required_signatures,
            confirmation_count: 0,
            active: true,
            expiration: env.ledger().sequence() + state.req_sigs_mod_expiration,
        };

        if next_id != 0 {
            let mut current_prop = Self::get_req_sigs_modification(env.clone(), next_id - 1)?;
            if current_prop.active {
                if current_prop.expiration <= env.ledger().sequence() {
                    current_prop.active = false;
                    env.storage()
                        .instance()
                        .set(&DataKey::ChangeReqSigs(next_id - 1), &current_prop);
                } else {
                    return Err(MultisigErr::AlreadyOpenSignaturesRequirementModificationProposal);
                }
            }
        }
        env.storage()
            .instance()
            .set(&DataKey::ChangeReqSigs(next_id), &new_proposal);
        state.next_req_sigs_modification_id += 1;
        env.storage()
            .instance()
            .set(&DataKey::MultisigState, &state);
        Ok(())
    }

    pub fn confirm_req_sigs_mod(env: Env, owner: Address) -> Result<(), MultisigErr> {
        let mut state = Self::get_multisig_state(env.clone())?;
        owner.require_auth();
        if !Self::is_owner(env.clone(), owner.clone())? {
            return Err(MultisigErr::CallerIsNotOwner);
        }
        let current_proposal_id =
            Self::get_multisig_state(env.clone())?.next_req_sigs_modification_id - 1;
        let mut proposal = Self::get_req_sigs_modification(env.clone(), current_proposal_id)?;
        if !proposal.active {
            return Err(MultisigErr::ProposalIsNotActive);
        }
        if env.ledger().sequence() >= proposal.expiration {
            return Err(MultisigErr::ProposalAlreadyExpired);
        }
        let mut confirmation = Self::get_req_sigs_mod_conf(env.clone(), owner.clone());
        if confirmation.change_req_sigs_id == current_proposal_id && confirmation.active {
            return Err(MultisigErr::OwnerAlreadyConfirmedModification);
        }
        confirmation.change_req_sigs_id = current_proposal_id;
        confirmation.active = true;

        proposal.confirmation_count += 1;

        if proposal.confirmation_count == state.required_signatures {
            state.required_signatures = proposal.new_requirement;
            env.storage()
                .instance()
                .set(&DataKey::MultisigState, &state);
            proposal.active = false;
        }

        env.storage()
            .instance()
            .set(&DataKey::ChangeReqSigs(current_proposal_id), &proposal);
        env.storage()
            .instance()
            .set(&DataKey::ReqSigsConf(owner.clone()), &confirmation);

        Ok(())
    }

    pub fn get_multisig_state(env: Env) -> Result<MultisigState, MultisigErr> {
        let state_op = env.storage().instance().get(&DataKey::MultisigState);
        if let Some(state) = state_op {
            Ok(state)
        } else {
            Err(MultisigErr::MultisigNotInitialized)
        }
    }

    pub fn get_proposed_tx(env: Env, proposal_id: u32) -> Result<ProposedTx, MultisigErr> {
        let state = Self::get_multisig_state(env.clone())?;
        if proposal_id >= state.next_tx_id {
            return Err(MultisigErr::InvalidProposalId);
        }
        let proposal = env
            .storage()
            .instance()
            .get(&DataKey::ProposedTx(proposal_id))
            .unwrap();
        Ok(proposal)
    }

    pub fn get_confirmation(env: Env, proposal_id: u32, owner: Address) -> Confirmation {
        env.storage()
            .instance()
            .get(&DataKey::Confirmation(proposal_id, owner))
            .unwrap_or_default()
    }

    pub fn get_member_modification(env: Env, address: Address) -> MemberModification {
        env.storage()
            .instance()
            .get(&DataKey::MemberModification(address))
            .unwrap_or_default()
    }

    pub fn get_member_confirmation(
        env: Env,
        modification_id: u32,
        owner: Address,
    ) -> MemberConfirmation {
        env.storage()
            .instance()
            .get(&DataKey::MemberConfirmation(modification_id, owner))
            .unwrap_or_default()
    }

    pub fn get_req_sigs_modification(env: Env, id: u32) -> Result<ChangeReqSigs, MultisigErr> {
        let state_op = env.storage().instance().get(&DataKey::ChangeReqSigs(id));
        if let Some(state) = state_op {
            return Ok(state);
        } else {
            return Err(MultisigErr::InvalidIdForRequiredSigsModification);
        }
    }

    pub fn get_req_sigs_mod_conf(env: Env, owner: Address) -> ReqSigsConf {
        env.storage()
            .instance()
            .get(&DataKey::ReqSigsConf(owner))
            .unwrap_or_default()
    }
}