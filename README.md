# Soroban Multisig Treasury

## Overview
The Soroban Multisig Treasury contract is a multi-signature smart contract designed to manage shared digital assets on the Stellar network. By separating transaction submission from execution, the contract ensures that no single account can unilaterally transfer funds. Instead, any transfer of tokens must be formally proposed, receive a predefined threshold of approvals from a designated set of owners, and then be executed. This pattern is ideal for decentralized autonomous organizations (DAOs), investment pools, and corporate treasuries requiring robust, decentralized governance over communal funds.

## Architecture
The contract functions are grouped into the following categories:

* **Setup**
  * `initialize(env: Env, owners: Vec<Address>, threshold: u32)`: Sets the list of unique owner addresses and the signature threshold.
* **Proposals**
  * `propose(env: Env, proposer: Address, recipient: Address, token: Address, amount: i128, description: String) -> u64`: Submits a new proposal for transferring tokens.
  * `approve(env: Env, approver: Address, proposal_id: u64)`: Submits an approval signature for an active proposal.
  * `execute(env: Env, caller: Address, proposal_id: u64)`: Executes the token transfer once the required threshold of approvals is met.
  * `reject(env: Env, caller: Address, proposal_id: u64)`: Rejects a proposal and clears all its accumulated approvals.
* **Views**
  * `get_proposal(env: Env, proposal_id: u64) -> Proposal`: Returns detailed information for a specific proposal.
  * `get_approvals(env: Env, proposal_id: u64) -> Vec<Address>`: Lists the addresses that have approved the given proposal.
  * `list_owners(env: Env) -> Vec<Address>`: Lists all registered owners.
  * `get_threshold(env: Env) -> u32`: Returns the current approval threshold.
  * `get_proposal_count(env: Env) -> u64`: Returns the total number of proposals created.

## Storage Layout
All data is stored in the contract's persistent storage using the following layout:

| DataKey | Type | Description |
| :--- | :--- | :--- |
| `Owners` | `Vec<Address>` | A vector containing the unique addresses of all registered multisig owners. |
| `Threshold` | `u32` | The minimum number of approvals needed to execute a proposal. |
| `ProposalCount` | `u64` | The running count of proposals, used to assign auto-incrementing proposal IDs. |
| `Proposal(id)` | `Proposal` | The struct representing a proposal's details (proposer, recipient, token, amount, execution/rejection status). |
| `Approvals(id)` | `Vec<Address>` | The vector of owner addresses that have approved the given proposal ID. |

## Proposal Lifecycle
A proposal progresses through the following sequential stages:
1. **Initialize**: The contract is deployed and initialized with the list of multisig owners and the approval threshold.
2. **Propose**: An owner calls `propose` to create a new proposal, which is assigned a sequential ID starting at `0`.
3. **Approve**: Owners call `approve` to sign off on the proposal. This step must be repeated until the number of approvals reaches or exceeds the threshold.
4. **Execute or Reject**: 
   - **Execute**: Once the threshold is met, any owner can call `execute` to trigger the token transfer and close the proposal.
   - **Reject**: Alternatively, any owner can call `reject` at any time prior to execution to permanently invalidate the proposal and clear its approvals.

## CLI Examples
You can interact with the contract using the Stellar CLI. The examples below utilize realistic placeholder values:

```bash
# 1. Initialize the contract with owners and a threshold of 2
stellar contract invoke --id CC12345... --source alice --network testnet -- initialize --owners '["GBOWNER1...", "GBOWNER2...", "GBOWNER3..."]' --threshold 2

# 2. Propose a transfer of 1000 tokens (at contract CDTOKEN...) to recipient GBRECIPIENT...
stellar contract invoke --id CC12345... --source alice --network testnet -- propose --proposer GBOWNER1... --recipient GBRECIPIENT... --token CDTOKEN... --amount 1000 --description "Q2 Grants Funding"

# 3. Approve proposal with ID 0
stellar contract invoke --id CC12345... --source bob --network testnet -- approve --approver GBOWNER2... --proposal_id 0

# 4. Execute proposal with ID 0
stellar contract invoke --id CC12345... --source alice --network testnet -- execute --caller GBOWNER1... --proposal_id 0

# 5. Reject proposal with ID 0 (alternative to execution)
stellar contract invoke --id CC12345... --source charlie --network testnet -- reject --caller GBOWNER3... --proposal_id 0

# 6. Retrieve detailed info for proposal with ID 0
stellar contract invoke --id CC12345... --source alice --network testnet -- get_proposal --proposal_id 0
```

## Events
The contract publishes diagnostic events for key transactions to enable indexing and off-chain tracking:

| Event | Topics | Data |
| :--- | :--- | :--- |
| `propose` | `(Symbol::new(env, "propose"), proposal_id: u64)` | `(proposer: Address, recipient: Address, token: Address, amount: i128)` |
| `approve` | `(Symbol::new(env, "approve"), proposal_id: u64)` | `(approver: Address, approvals_count: u32)` |
| `execute` | `(Symbol::new(env, "execute"), proposal_id: u64)` | `(recipient: Address, amount: i128)` |
| `reject` | `(Symbol::new(env, "reject"), proposal_id: u64)` | `caller: Address` |

## Contributing
We welcome contributions to the Soroban Multisig Treasury contract. If you identify bugs, design gaps, or performance improvements, please check our open GitHub issues or submit a pull request. This project is proudly built in alignment with the Stellar Wave Program to support robust tooling and open-source infrastructure across the Stellar network ecosystem.
