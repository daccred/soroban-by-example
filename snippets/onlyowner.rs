#![no_std]

pub mod owner {
    use soroban_sdk::{contracterror, contracttype, panic_with_error, Address, Env, Symbol};

    #[derive(Clone)]
    #[contracttype]
    enum OwnerKey {
        Owner,
    }

    #[derive(Copy, Clone)]
    #[contracterror]
    #[repr(u32)]
    pub enum OwnerError {
        OnlyOwner = 1001,
    }

    pub fn has_owner(e: &Env) -> bool {
        let key = OwnerKey::Owner;
        e.storage().instance().has(&key)
    }

    pub fn get_owner(e: &Env) -> Option<Address> {
        let key = OwnerKey::Owner;
        e.storage().instance().get(&key)
    }

    pub fn set_owner(e: &Env, id: &Address) {
        let key = OwnerKey::Owner;
        e.storage().instance().set(&key, id);
        e.events().publish((Symbol::new(e, "OwnerSet"),), *&id);
    }

    pub fn only_owner(e: &Env) {
        let owner = get_owner(e);
        if let Some(owner) = owner {
            owner.require_auth();
        } else {
            panic_with_error!(e, OwnerError::OnlyOwner);
        }
    }
}