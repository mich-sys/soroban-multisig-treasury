# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-05-20

### Added
- `initialize(env: Env, owners: Vec<Address>, threshold: u32)`: Sets up multisig owners and the execution threshold.
- `propose(env: Env, proposer: Address, recipient: Address, token: Address, amount: i128, description: String) -> u64`: Submits a new treasury transfer proposal.
- `approve(env: Env, approver: Address, proposal_id: u64)`: Approves an active proposal.
- `execute(env: Env, caller: Address, proposal_id: u64)`: Executes a proposal that has reached the threshold.
- `reject(env: Env, caller: Address, proposal_id: u64)`: Rejects a proposal and clears approvals.
- `get_proposal(env: Env, proposal_id: u64) -> Proposal`: Returns proposal details or panics if not found.
- `get_approvals(env: Env, proposal_id: u64) -> Vec<Address>`: Returns the list of addresses that approved the proposal.
- `list_owners(env: Env) -> Vec<Address>`: Returns the registered owners.
- `get_threshold(env: Env) -> u32`: Returns the signature/approval threshold.
- `get_proposal_count(env: Env) -> u64`: Returns the total count of proposals created.
