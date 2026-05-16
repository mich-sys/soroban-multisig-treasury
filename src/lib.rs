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
        todo!()
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
