# Soroban Contracts — Specification

Three Soroban smart contracts implement BWB's on-chain logic. All contracts are written in Rust, licensed Apache 2.0, and live in `contracts/`.

---

## Contract 1 — `kyc-whitelist`

**Path:** `contracts/kyc-whitelist/src/lib.rs`  
**Purpose:** On-chain KYC registry enforcing CVM Resolution 88 transfer restrictions  
**Tranche:** T1 (required for testnet deployment)

### Storage

| Key | Type | Storage tier | Description |
|---|---|---|---|
| `Admin` | `Address` | Instance | Contract administrator (cold wallet) |
| `PendingAdmin` | `Address` | Instance | Proposed successor admin (two-step handover) |
| `Operator` | `Address` | Instance | Hot wallet for daily KYC operations |
| `Entry(Address)` | `WhitelistEntry` | Persistent | KYC approval record per investor |

Persistent storage is used for per-investor entries because investor data must survive ledger compaction. Instance storage is used for governance config (admin, operator).

### Data Types

```rust
pub enum InvestorCategory {
    Retail,       // Standard retail — CVM 88 base limits
    Qualified,    // R$1M+ financial assets
    Professional, // R$10M+ / institutional
}

pub struct WhitelistEntry {
    pub investor_category: InvestorCategory,
    pub approved_at: u64,   // ledger timestamp
    pub approved_by: Address,
}
```

### Functions

**Initialization**

| Function | Auth | Description |
|---|---|---|
| `initialize(admin)` | admin | Deploy and set admin address. Panics if called twice. |

**Admin governance**

| Function | Auth | Description |
|---|---|---|
| `propose_admin(new_admin)` | admin | Step 1 of two-step handover — proposes a successor |
| `accept_admin()` | pending admin | Step 2 — completes handover, previous admin loses access |
| `set_operator(operator)` | admin | Set the hot wallet for daily KYC operations |
| `remove_operator()` | admin | Revoke operator — only admin can operate after this |

**KYC operations**

| Function | Auth | Description |
|---|---|---|
| `add(caller, address, category)` | caller (admin or operator) | Add investor to whitelist with CVM category |
| `remove(caller, address)` | caller (admin or operator) | Remove investor — panics if not in whitelist |

**Read (no auth)**

| Function | Returns | Description |
|---|---|---|
| `is_ok(address)` | `bool` | Check approval — called by `real-estate-token` on every transfer |
| `get_entry(address)` | `Option<WhitelistEntry>` | Full entry details or None |
| `get_admin()` | `Address` | Current admin address |
| `get_operator()` | `Option<Address>` | Current operator or None |

**TTL management**

| Function | Auth | Description |
|---|---|---|
| `extend_ttl()` | none | Extend instance storage TTL (heartbeat — call periodically) |
| `extend_entry_ttl(address)` | none | Extend TTL for a specific investor entry |

### Events

| Event | Payload | Trigger |
|---|---|---|
| `kyc_add` | `address` | Investor approved |
| `kyc_rm` | `address` | Investor removed |
| `adm_prop` | `new_admin` | Admin handover proposed |
| `adm_new` | `new_admin` | Admin handover completed |
| `op_set` | `operator` | Operator set |

### Invariants

- Only `Admin` or `Operator` can modify the whitelist
- `Admin` is always set — contract is unusable without it
- `Operator` is optional — if unset, only admin can perform KYC operations
- Admin handover requires two transactions (propose + accept) — prevents accidental lockout
- `is_ok` is a pure read — no auth, callable by any contract at zero cost
- Removing an investor preserves the on-chain event trail (audit log intact)

---

## Contract 2 — `real-estate-token`

**Path:** `contracts/real-estate-token/src/lib.rs`  
**Purpose:** CVM 88-compliant RWA token — one contract per offering  
**Tranche:** T1 (core issuance logic)

### Storage

| Key | Type | Storage tier | Description |
|---|---|---|---|
| `Admin` | `Address` | Instance | Contract administrator |
| `KycContract` | `Address` | Instance | Address of kyc-whitelist contract |
| `TotalSupply` | `i128` | Instance | Aggregate token supply |
| `Metadata` | `OfferingMetadata` | Instance | Immutable offering details |
| `Paused` | `bool` | Instance | Emergency pause flag |
| `Balance(Address)` | `i128` | Persistent | Per-investor token balance |

### Data Types

```rust
pub struct OfferingMetadata {
    pub offering_id: String,        // BWB internal ID (e.g. "ARTP-HS")
    pub property_address: String,   // Brazilian property address
    pub total_raise: i128,          // Total raise in BRL cents
    pub target_irr_bps: u32,        // Target IRR in basis points (2080 = 20.80%)
    pub maturity_date: u64,         // Unix timestamp
    pub cvm_authorization: String,  // CVM authorization code
}
```

### Functions

| Function | Auth | Description |
|---|---|---|
| `initialize(admin, kyc_contract, metadata)` | admin | Deploy with offering details |
| `mint(to, amount)` | admin | Issue tokens to KYC-approved investor |
| `transfer(from, to, amount)` | from | Transfer — recipient must be KYC-approved |
| `balance(id) → i128` | none | Query investor balance |
| `total_supply() → i128` | none | Query total issued tokens |
| `get_offering() → OfferingMetadata` | none | Read offering details on-chain |

### CVM 88 Enforcement

`mint` and `transfer` both call `kyc-whitelist::is_ok(address)` before proceeding. If the recipient is not whitelisted, the transaction panics with:

```
Transfer rejected: recipient not KYC-approved (CVM 88)
```

This check is enforced at the contract level — no frontend bypass is possible.

### Events

| Event | Payload | Trigger |
|---|---|---|
| `mint` | `(to, amount)` | Tokens issued |
| `transfer` | `(from, to, amount)` | Tokens transferred |

### Invariants

- `total_supply` = sum of all `Balance` values at all times
- No balance can go negative (checked before deduction)
- Only KYC-approved addresses can hold tokens
- `metadata` is set at initialization and immutable thereafter

### Pending Completions (before mainnet)

- SEP-0041 full interface (`approve`, `allowance`, `burn`, `decimals`, `name`, `symbol`) — required for Stellar wallet compatibility
- `extend_ttl()` for balance entries and instance storage
- Admin / Operator split
- NAV calculation (`total_raise / total_supply`)

---

## Contract 3 — `distribution`

**Path:** `contracts/distribution/src/lib.rs`  
**Purpose:** Programmatic proportional yield distribution to all token holders  
**Tranche:** T3 (after mainnet launch)

### Design

Each distribution cycle:
1. Admin calls `distribute(token_contract, amount)` with a BRLA amount
2. Contract reads `total_supply` from `real-estate-token`
3. For each holder, computes `holder_share = amount × balance / total_supply`
4. Transfers BRLA from distribution reserve to each holder's Stellar address
5. Records distribution on-chain (count, amount, timestamp)

All operations are on-chain and auditable. Holders only need a KYC-approved Stellar address — no active claim transaction required.

### Why Soroban for Distributions

On EVM, distributing to 100 holders costs $50–200 in gas. On Soroban, the same operation costs under $0.10. This makes quarterly distributions economically viable at the retail scale BWB operates.

---

## Contract Interactions

```
Admin (Operator keypair)
  │
  ├── kyc-whitelist::add(investor_address, category)
  │
  └── real-estate-token::mint(investor_address, amount)
            │
            └── [internal] kyc-whitelist::is_ok(investor_address)
                     returns true → mint proceeds
                     returns false → transaction rejected
```

```
distribution::distribute(token_contract, brla_amount)
  │
  ├── real-estate-token::total_supply()
  ├── for each holder: real-estate-token::balance(address)
  └── BRLA::transfer(distribution_contract → holder)
```

---

## Security Considerations

- All admin functions require `require_auth()` — no function can be called without a valid Stellar signature
- Emergency pause (`Paused` flag) on `real-estate-token` blocks transfers without affecting balances
- KYC revocation is immediate — a removed investor cannot receive tokens in the same ledger
- Upgrade path: contract data keys are designed to be forward-compatible with Soroban upgrade patterns

Full security audit is planned before mainnet deployment (see [scf-deliverables.md](scf-deliverables.md) Tranche 3).
