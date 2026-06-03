# Soroban Smart Contracts — Technical Reference

## Overview

All smart contracts are written in **Rust** using the [Soroban SDK](https://developers.stellar.org/docs/smart-contracts). Contracts are compiled to WebAssembly (WASM) and deployed on the Stellar network.

---

## Contract 1: `real-estate-token`
**Path:** `contracts/real-estate-token/`  
**Milestone:** Tranche 1  
**License:** Apache 2.0

### Purpose
Issues and manages tokenized real estate fractional ownership on Stellar. Each token represents a regulated security under CVM Resolution 88.

### Interface (planned)

```rust
pub trait RealEstateToken {
    // Admin functions
    fn initialize(env: Env, admin: Address, kyc_contract: Address, metadata: OfferingMetadata);
    fn mint(env: Env, to: Address, amount: i128);
    fn set_kyc_contract(env: Env, kyc_contract: Address);

    // Token standard (SEP-41 compatible)
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn balance(env: Env, id: Address) -> i128;
    fn total_supply(env: Env) -> i128;

    // Offering metadata
    fn get_offering(env: Env) -> OfferingMetadata;
}

pub struct OfferingMetadata {
    pub offering_id: String,        // CVM 88 offering number
    pub property_address: String,
    pub total_raise: i128,          // BRL (in centavos)
    pub target_irr: u32,            // basis points (e.g., 2080 = 20.80%)
    pub maturity_date: u64,         // Unix timestamp
    pub cvm_authorization: String,  // CVM authorization document hash
}
```

### Transfer Restrictions
Every `transfer()` call validates:
1. `from` has sufficient balance
2. `to` is in the `kyc-whitelist` contract
3. Transfer amount > 0
4. Contract is not paused (admin emergency stop)

---

## Contract 2: `kyc-whitelist`
**Path:** `contracts/kyc-whitelist/`  
**Milestone:** Tranche 1  
**License:** Apache 2.0

### Purpose
Manages the list of CVM 88-verified investor addresses. Acts as the on-chain compliance gate for all token operations.

### Interface (planned)

```rust
pub trait KYCWhitelist {
    // Admin functions (BWB compliance team)
    fn initialize(env: Env, admin: Address);
    fn add(env: Env, address: Address, category: InvestorCategory);
    fn remove(env: Env, address: Address);
    fn add_admin(env: Env, new_admin: Address);

    // Query functions (called by real-estate-token)
    fn is_approved(env: Env, address: Address) -> bool;
    fn get_entry(env: Env, address: Address) -> Option<WhitelistEntry>;
    fn get_all_approved(env: Env) -> Vec<Address>;

    // Events emitted
    // KYCAdded(address, category, timestamp)
    // KYCRevoked(address, timestamp)
}

pub struct WhitelistEntry {
    pub investor_category: InvestorCategory,
    pub approved_at: u64,
    pub approved_by: Address,
}

pub enum InvestorCategory {
    Retail,       // CVM 88: standard retail investor
    Qualified,    // CVM 88: R$1M+ financial assets
    Professional, // CVM 88: institutional / R$10M+
}
```

---

## Contract 3: `distribution`
**Path:** `contracts/distribution/`  
**Milestone:** Tranche 3  
**License:** Apache 2.0

### Purpose
Automates proportional yield distributions (rental income, interest) to all token holders. Eliminates manual off-chain calculations and EVM gas overhead.

### Interface (planned)

```rust
pub trait Distribution {
    // Admin functions
    fn initialize(env: Env, admin: Address, token_contract: Address, kyc_contract: Address);
    fn deposit_yield(env: Env, amount: i128, asset: Address);  // BRLA
    fn trigger_distribution(env: Env);

    // Query functions
    fn get_distribution_history(env: Env) -> Vec<DistributionEvent>;
    fn get_holder_distributions(env: Env, holder: Address) -> Vec<DistributionEvent>;

    // Events emitted
    // YieldDeposited(amount, asset, timestamp)
    // DistributionExecuted(total_amount, holder_count, timestamp)
    // HolderPaid(holder, amount, timestamp)
}

pub struct DistributionEvent {
    pub distribution_id: u64,
    pub total_amount: i128,
    pub per_token_amount: i128,
    pub holder_count: u32,
    pub executed_at: u64,
    pub asset: Address,  // BRLA contract address
}
```

---

## Development Setup

```bash
# Install Rust + Soroban CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli

# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/real_estate_token.wasm \
  --network testnet \
  --source <DEPLOYER_SECRET_KEY>
```

---

## Testing Strategy

### Unit Tests (`tests/unit/`)
- Each contract function tested in isolation
- Edge cases: zero amounts, unauthorized callers, non-whitelisted addresses
- Target: ≥80% code coverage

### Integration Tests (`tests/integration/`)
- Full flow: mint → whitelist → transfer → distribution
- Multi-contract interactions (token ↔ kyc-whitelist)
- Testnet deployment verification

### Compliance Tests (`tests/compliance/`)
- CVM 88 restriction enforcement
- Transfer to non-whitelisted address (must revert)
- Admin-only functions (must revert for non-admin)
- Investor category enforcement

---

## References

- [Soroban Developer Docs](https://developers.stellar.org/docs/smart-contracts)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Tigerblocks KYC Contract (SCF #22)](https://github.com/tigerblocks) — reference implementation
- [SEP-41 Token Standard](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md)
