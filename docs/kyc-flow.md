# KYC Flow — CVM 88 Compliance On-Chain

## Overview

BWB operates under **CVM Resolution 88** — Brazil's regulatory framework for electronic investment platforms (equivalent to SEC/FCA oversight for tokenized securities). This document describes how CVM 88 KYC requirements are implemented on-chain via Soroban.

---

## CVM 88 Requirements Mapped to Soroban

| CVM 88 Requirement | On-Chain Implementation | Contract |
|---|---|---|
| Investor identity verification before investment | Whitelist check before any token transfer | `kyc-whitelist` |
| Qualified/retail investor classification | Whitelist includes investor category metadata | `kyc-whitelist` |
| Transfer restrictions to non-verified addresses | `transfer()` reverts if recipient not whitelisted | `real-estate-token` |
| Audit trail of investor transactions | All events emitted and indexed on Stellar | `real-estate-token` |
| Periodic distribution to verified holders only | Distribution iterates whitelist-verified addresses | `distribution` |

---

## KYC Flow Diagram

```
INVESTOR                    BWB PLATFORM              SOROBAN (ON-CHAIN)
    │                           │                           │
    │  1. Sign up + submit KYC  │                           │
    │ ─────────────────────────►│                           │
    │                           │                           │
    │                           │  2. Off-chain KYC check   │
    │                           │  (identity docs, CVM 88   │
    │                           │   suitability assessment) │
    │                           │                           │
    │                           │  3. KYC approved          │
    │                           │  (CVM 88 compliant)       │
    │                           │                           │
    │                           │  4. add_to_whitelist()    │
    │                           │ ─────────────────────────►│
    │                           │                           │ kyc-whitelist
    │                           │                           │ stores: address,
    │                           │                           │ category, timestamp
    │                           │                           │
    │  5. Invest in offering    │                           │
    │ ─────────────────────────►│                           │
    │                           │                           │
    │                           │  6. transfer(to=investor) │
    │                           │ ─────────────────────────►│
    │                           │                           │ real-estate-token
    │                           │                           │ checks whitelist →
    │                           │                           │ ✅ approved
    │                           │                           │ transfer executes
    │                           │                           │
    │  7. Token in wallet       │                           │
    │◄─────────────────────────────────────────────────────│
```

---

## Whitelist Contract — State Structure

```rust
// kyc-whitelist/src/lib.rs (planned)

pub struct WhitelistEntry {
    pub address: Address,
    pub investor_category: InvestorCategory,  // Retail | Qualified | Professional
    pub kyc_approved_at: u64,                  // Unix timestamp
    pub approved_by: Address,                  // BWB admin address
}

pub enum InvestorCategory {
    Retail,       // CVM 88: up to R$20K/year per platform
    Qualified,    // CVM 88: R$1M+ in financial investments
    Professional, // CVM 88: R$10M+ or institutional
}
```

---

## Transfer Restriction Logic

```rust
// real-estate-token/src/lib.rs (planned)

fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    from.require_auth();

    // CVM 88: verify recipient is KYC-approved
    let kyc_contract = env.storage().get::<Address>(&KYC_CONTRACT_KEY);
    let is_approved: bool = env.invoke_contract(
        &kyc_contract,
        &symbol_short!("is_ok"),
        vec![&env, to.clone().into_val(&env)]
    );

    if !is_approved {
        panic!("Transfer rejected: recipient not KYC-approved (CVM 88)");
    }

    // Execute transfer
    token::transfer(&env, &from, &to, &amount);
}
```

---

## CVM 88 vs SEC/FCA — Equivalence for International Evaluators

| Dimension | CVM 88 (Brazil) | SEC (USA) | FCA (UK) |
|---|---|---|---|
| Regulatory body | CVM (Comissão de Valores Mobiliários) | Securities and Exchange Commission | Financial Conduct Authority |
| Instrument type | Participação em negócios de impacto / FIPs | Regulation Crowdfunding (Reg CF) | FCA Crowdfunding Rules |
| Investor verification | Mandatory KYC + suitability | Mandatory KYC + accreditation | Mandatory KYC + appropriateness |
| Annual limits | R$15M per issuer | $5M per issuer (Reg CF) | £5M per issuer |
| Platform authorization | Required (CVM registration) | Required (FINRA registration) | Required (FCA authorization) |
| Transfer restrictions | Mandatory in secondary market | Mandatory (Rule 144) | Mandatory (restriction period) |
| **BWB status** | **Active since 2022** | N/A | N/A |

> BWB is the **only CVM Resolution 88-authorized platform** applying to the SCF ecosystem.

---

## Off-Chain → On-Chain KYC Sync

```
BWB Database (Convex)          Stellar Network
┌─────────────────────┐        ┌─────────────────────┐
│ investors table     │        │ kyc-whitelist        │
│ ─────────────────── │        │ contract             │
│ id: UUID            │        │ ─────────────────────│
│ stellar_address     │──────►│ address: Address     │
│ kyc_status: enum    │        │ category: enum       │
│ kyc_approved_at     │        │ approved_at: u64     │
│ investor_category   │        │                     │
│ cvm_suitability     │        │                     │
└─────────────────────┘        └─────────────────────┘
         │
         │ Triggered by: Convex mutation
         │ on KYC status change
         ▼
   backend/stellar/mutations.ts
   → sdk/src/kyc.ts
   → kyc-whitelist contract
```

---

## Compliance Monitoring

- All on-chain transfers emit events indexed by Stellar Horizon
- BWB compliance team monitors whitelist changes via Convex queries
- Quarterly CVM reports generated from on-chain event data
- Smart contract admin key held in HSM — multi-sig required for whitelist changes
