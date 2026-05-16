#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owners,
    Threshold,
    ProposalCount,
    Proposal(u64),
    Approvals(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub description: String,
    pub executed: bool,
    pub rejected: bool,
}

#[contract]
pub struct MultisigTreasury;

#[contractimpl]
impl MultisigTreasury {
    pub fn initialize(env: Env, owners: Vec<Address>, threshold: u32) {
        if env.storage().persistent().has(&DataKey::Owners) {
            panic!("already initialized");
        }

        if owners.is_empty() {
            panic!("owners list is empty");
        }

        if threshold == 0 {
            panic!("threshold must be at least 1");
        }

        if threshold > owners.len() {
            panic!("threshold cannot be greater than the number of owners");
        }

        // Validate no duplicate addresses and require auth
        let mut unique_owners = Vec::new(&env);
        for owner in owners.iter() {
            owner.require_auth();
            if unique_owners.contains(&owner) {
                panic!("duplicate owner address");
            }
            unique_owners.push_back(owner);
        }

        env.storage().persistent().set(&DataKey::Owners, &owners);
        env.storage().persistent().set(&DataKey::Threshold, &threshold);
        env.storage().persistent().set(&DataKey::ProposalCount, &0u64);
    }

    pub fn propose(
        env: Env,
        proposer: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        description: String,
    ) -> u64 {
        todo!()
    }

    pub fn approve(env: Env, caller: Address, proposal_id: u64) {
        todo!()
    }

    pub fn execute(env: Env, proposal_id: u64) {
        todo!()
    }

    pub fn reject(env: Env, proposal_id: u64) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
