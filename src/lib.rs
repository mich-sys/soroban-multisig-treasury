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
        proposer.require_auth();

        let owners: Vec<Address> = env.storage().persistent().get(&DataKey::Owners).expect("not initialized");
        if !owners.contains(&proposer) {
            panic!("not an owner");
        }

        if amount <= 0 {
            panic!("invalid amount");
        }

        let mut count: u64 = env.storage().persistent().get(&DataKey::ProposalCount).unwrap_or(0);
        let id = count;

        let proposal = Proposal {
            id,
            proposer: proposer.clone(),
            recipient,
            token,
            amount,
            description,
            executed: false,
            rejected: false,
        };

        env.storage().persistent().set(&DataKey::Proposal(id), &proposal);
        
        count += 1;
        env.storage().persistent().set(&DataKey::ProposalCount, &count);

        id
    }

    pub fn approve(env: Env, approver: Address, proposal_id: u64) {
        approver.require_auth();

        let owners: Vec<Address> = env.storage().persistent().get(&DataKey::Owners).expect("not initialized");
        if !owners.contains(&approver) {
            panic!("not an owner");
        }

        let proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).expect("proposal not found");
        
        if proposal.executed {
            panic!("already executed");
        }
        if proposal.rejected {
            panic!("already rejected");
        }

        let mut approvals: Vec<Address> = env.storage().persistent().get(&DataKey::Approvals(proposal_id)).unwrap_or_else(|| Vec::new(&env));
        
        if approvals.contains(&approver) {
            panic!("already approved");
        }

        approvals.push_back(approver);
        env.storage().persistent().set(&DataKey::Approvals(proposal_id), &approvals);
    }

    pub fn execute(env: Env, caller: Address, proposal_id: u64) {
        caller.require_auth();

        let owners: Vec<Address> = env.storage().persistent().get(&DataKey::Owners).expect("not initialized");
        if !owners.contains(&caller) {
            panic!("not an owner");
        }

        let mut proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).expect("proposal not found");
        
        if proposal.executed {
            panic!("already executed");
        }
        if proposal.rejected {
            panic!("already rejected");
        }

        let approvals: Vec<Address> = env.storage().persistent().get(&DataKey::Approvals(proposal_id)).unwrap_or_else(|| Vec::new(&env));
        let threshold: u32 = env.storage().persistent().get(&DataKey::Threshold).expect("not initialized");

        if approvals.len() < threshold {
            panic!("threshold not reached");
        }

        soroban_sdk::token::Client::new(&env, &proposal.token).transfer(&env.current_contract_address(), &proposal.recipient, &proposal.amount);

        proposal.executed = true;
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);
    }

    pub fn reject(env: Env, caller: Address, proposal_id: u64) {
        caller.require_auth();

        let owners: Vec<Address> = env.storage().persistent().get(&DataKey::Owners).expect("not initialized");
        if !owners.contains(&caller) {
            panic!("not an owner");
        }

        let mut proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).expect("proposal not found");
        
        if proposal.executed {
            panic!("already executed");
        }
        if proposal.rejected {
            panic!("already rejected");
        }

        proposal.rejected = true;
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);

        let empty_approvals: Vec<Address> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::Approvals(proposal_id), &empty_approvals);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
