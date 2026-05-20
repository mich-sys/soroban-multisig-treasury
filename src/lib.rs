#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

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
        env.events().publish(
            (Symbol::new(&env, "propose"), id),
            (
                proposal.proposer.clone(),
                proposal.recipient.clone(),
                proposal.token.clone(),
                proposal.amount,
            ),
        );
        
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

        let approver_clone = approver.clone();
        approvals.push_back(approver);
        env.storage().persistent().set(&DataKey::Approvals(proposal_id), &approvals);
        env.events().publish(
            (Symbol::new(&env, "approve"), proposal_id),
            (approver_clone, approvals.len() as u32),
        );
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
        env.events().publish(
            (Symbol::new(&env, "execute"), proposal_id),
            (proposal.recipient.clone(), proposal.amount),
        );
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
        env.events().publish(
            (Symbol::new(&env, "reject"), proposal_id),
            caller.clone(),
        );

        let empty_approvals: Vec<Address> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::Approvals(proposal_id), &empty_approvals);
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found")
    }

    pub fn get_approvals(env: Env, proposal_id: u64) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::Approvals(proposal_id))
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn list_owners(env: Env) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::Owners)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_threshold(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Threshold)
            .unwrap_or(0)
    }

    pub fn get_proposal_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Vec, String};

    fn create_env() -> (Env, MultisigTreasuryClient) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, MultisigTreasury);
        let client = MultisigTreasuryClient::new(&env, &contract_id);
        (env, client)
    }

    #[test]
    fn test_initialize() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        let threshold = 2;

        client.initialize(&owners, &threshold);

        env.as_contract(&client.address, || {
            let stored_owners: Vec<Address> = env.storage().persistent().get(&DataKey::Owners).unwrap();
            let stored_threshold: u32 = env.storage().persistent().get(&DataKey::Threshold).unwrap();
            assert_eq!(stored_owners, owners);
            assert_eq!(stored_threshold, threshold);
        });
    }

    #[test]
    fn test_propose() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        assert_eq!(id, 0);

        env.as_contract(&client.address, || {
            let proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(0)).unwrap();
            assert_eq!(proposal.id, 0);
            assert_eq!(proposal.proposer, owner1);
            assert_eq!(proposal.recipient, recipient);
            assert_eq!(proposal.token, token);
            assert_eq!(proposal.amount, amount);
            assert_eq!(proposal.description, description);
            assert_eq!(proposal.executed, false);
            assert_eq!(proposal.rejected, false);
        });
    }

    #[test]
    fn test_approve() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        client.approve(&owner1, &id);

        env.as_contract(&client.address, || {
            let approvals: Vec<Address> = env.storage().persistent().get(&DataKey::Approvals(id)).unwrap();
            assert_eq!(approvals.len(), 1);
            assert_eq!(approvals.get(0).unwrap(), owner1);
        });
    }

    #[test]
    fn test_execute_happy_path() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        // Set up the token contract
        let token_admin = Address::generate(&env);
        let token_address = env.register_stellar_asset_contract(token_admin);
        let token_client = soroban_sdk::token::Client::new(&env, &token_address);
        let token_admin_client = soroban_sdk::token::StellarAssetContractClient::new(&env, &token_address);

        // Mint tokens to the treasury contract
        let amount = 1000i128;
        token_admin_client.mint(&client.address, &amount);

        let recipient = Address::generate(&env);
        let description = String::from_str(&env, "Test proposal");

        // Propose
        let id = client.propose(&owner1, &recipient, &token_address, &amount, &description);

        // Approve by both owners to meet threshold of 2
        client.approve(&owner1, &id);
        client.approve(&owner2, &id);

        // Execute
        client.execute(&owner1, &id);

        // Assert proposal.executed == true
        env.as_contract(&client.address, || {
            let proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(id)).unwrap();
            assert!(proposal.executed);
        });

        // Also assert that recipient received the tokens
        assert_eq!(token_client.balance(&recipient), amount);
        assert_eq!(token_client.balance(&client.address), 0);
    }

    #[test]
    #[should_panic(expected = "threshold not reached")]
    fn test_execute_below_threshold() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        client.approve(&owner1, &id); // Only 1 approval, threshold is 2

        client.execute(&owner1, &id);
    }

    #[test]
    fn test_reject() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        client.reject(&owner1, &id);

        env.as_contract(&client.address, || {
            let proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(id)).unwrap();
            assert!(proposal.rejected);
        });
    }

    #[test]
    #[should_panic(expected = "already approved")]
    fn test_double_approve() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        client.approve(&owner1, &id);
        client.approve(&owner1, &id); // Should panic with "already approved"
    }

    #[test]
    fn test_view_proposal() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone()];
        client.initialize(&owners, &1);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        let proposal = client.get_proposal(&id);
        
        assert_eq!(proposal.id, id);
        assert_eq!(proposal.proposer, owner1);
        assert_eq!(proposal.recipient, recipient);
        assert_eq!(proposal.token, token);
        assert_eq!(proposal.amount, amount);
        assert_eq!(proposal.description, description);
        assert_eq!(proposal.executed, false);
        assert_eq!(proposal.rejected, false);
    }

    #[test]
    #[should_panic(expected = "proposal not found")]
    fn test_view_proposal_not_found() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone()];
        client.initialize(&owners, &1);

        client.get_proposal(&999);
    }

    #[test]
    fn test_view_approvals() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone()];
        client.initialize(&owners, &1);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        let id = client.propose(&owner1, &recipient, &token, &amount, &description);
        
        let approvals_before = client.get_approvals(&id);
        assert_eq!(approvals_before.len(), 0);

        client.approve(&owner1, &id);
        let approvals_after = client.get_approvals(&id);
        assert_eq!(approvals_after.len(), 1);
        assert_eq!(approvals_after.get(0).unwrap(), owner1);
    }

    #[test]
    fn test_view_owners() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone(), owner2.clone()];
        client.initialize(&owners, &2);

        let retrieved_owners = client.list_owners();
        assert_eq!(retrieved_owners.len(), 2);
        assert_eq!(retrieved_owners.get(0).unwrap(), owner1);
        assert_eq!(retrieved_owners.get(1).unwrap(), owner2);
    }

    #[test]
    fn test_view_threshold() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone()];
        client.initialize(&owners, &1);

        assert_eq!(client.get_threshold(), 1);
    }

    #[test]
    fn test_view_proposal_count() {
        let (env, client) = create_env();
        let owner1 = Address::generate(&env);
        let owners = soroban_sdk::vec![&env, owner1.clone()];
        client.initialize(&owners, &1);

        assert_eq!(client.get_proposal_count(), 0);

        let recipient = Address::generate(&env);
        let token = Address::generate(&env);
        let amount = 1000i128;
        let description = String::from_str(&env, "Test proposal");

        client.propose(&owner1, &recipient, &token, &amount, &description);
        assert_eq!(client.get_proposal_count(), 1);
    }
}
