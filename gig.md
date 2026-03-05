
# Rust + Solana Playground Gigs

This repository is a playground for experimenting with Rust programs that simulate or implement Solana-style logic.  
The goal is to practice low-level Rust, smart contract patterns, and deterministic program design similar to on-chain environments.

## Available Gigs

### 1. Command Processor Improvement
File: `Command Processor.rs`

Goal:
Improve the command processing logic to support multiple instructions similar to Solana program entrypoints.

Tasks:
- Add instruction enum parsing
- Implement instruction routing
- Improve error handling
- Add deterministic execution flow

Expected Output:
A clean instruction dispatcher similar to Solana program instruction handling.

---

### 2. Calculator Program Extension
File: `calculator.rs`

Goal:
Extend the calculator logic to support advanced operations.

Tasks:
- Add multiplication and division
- Add overflow-safe arithmetic
- Add unit tests
- Implement instruction-based inputs

Optional:
Convert this into a Solana-style instruction processor.

---

### 3. SOL Split Simulation
File: `solsplit.rs`

Goal:
Simulate a smart contract that splits SOL between multiple accounts.

Tasks:
- Implement proportional splitting
- Add validation for input accounts
- Add deterministic state handling
- Simulate account balances

Bonus:
Add a struct representing a Solana account.

---

### 4. Deterministic Program Architecture

Goal:
Refactor all files into a proper deterministic program architecture similar to Solana programs.

Tasks:
- Create instruction enums
- Implement state structs
- Separate logic into modules
- Ensure no non-deterministic operations

Suggested Structure:
