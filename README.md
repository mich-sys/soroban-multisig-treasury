# Soroban Multisig Treasury

A secure, multi-signature treasury smart contract built on the **Stellar Soroban** platform.

This smart contract manages a shared treasury of tokens, ensuring that transfers must be proposed, approved by a set threshold of owners, and then executed.

---

## Features

- **Multi-Signature Control**: Decouples the initiation of a transaction from its execution, requiring validation and authorization from multiple trusted accounts.
- **Dynamic Proposals**: Allows owners to propose token transfers with specific targets, amounts, and descriptions.
- **Approval Tracking**: Manages and persists approvals for each active proposal.
- **Strict Verification**: Ensures that only designated owners can propose, approve, execute, or reject transactions.
- **Event-Driven Architecture**: Emits diagnostic events for core operations (`propose`, `approve`, `execute`, `reject`) to facilitate downstream indexing and off-chain visibility.

---

## Contract Interface

### 1. `initialize`
Initializes the contract with a list of unique owner addresses and an approval threshold.
```rust
pub fn initialize(env: Env, owners: Vec<Address>, threshold: u32)
```

### 2. `propose`
Proposes a token transfer. Returns the generated unique `proposal_id`.
```rust
pub fn propose(
    env: Env,
    proposer: Address,
    recipient: Address,
    token: Address,
    amount: i128,
    description: String,
) -> u64
```
* **Event emitted**: `("propose", proposal_id)` with payload `(proposer, recipient, token, amount)`.

### 3. `approve`
Approves a proposed transfer.
```rust
pub fn approve(env: Env, approver: Address, proposal_id: u64)
```
* **Event emitted**: `("approve", proposal_id)` with payload `(approver, approvals_count)`.

### 4. `execute`
Executes an approved proposal if the threshold is met, transferring the tokens.
```rust
pub fn execute(env: Env, caller: Address, proposal_id: u64)
```
* **Event emitted**: `("execute", proposal_id)` with payload `(recipient, amount)`.

### 5. `reject`
Rejects a proposal, resetting the approvals list.
```rust
pub fn reject(env: Env, caller: Address, proposal_id: u64)
```
* **Event emitted**: `("reject", proposal_id)` with payload `caller`.

---

## Development & Usage

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli)

### Compilation
To compile the smart contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Type Checking
To check that the codebase compiles:
```bash
cargo check
```
