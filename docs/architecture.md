# System Architecture — BWB + Stellar/Soroban

## Overview

BWB currently operates on Base (EVM L2) blockchain. This document describes the full architecture of the Stellar/Soroban integration, which will serve as the **core settlement and token issuance layer**.

---

## Current Architecture (Base/EVM)

```
┌─────────────────────────────────────────────────────────────┐
│                    INVESTOR (Brazil / Global)                │
│                  Retail / Qualified / Institutional          │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTPS
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              BWB Platform — app.bwbi.com.br                  │
│         React + TypeScript + Privy (auth) + Notus            │
│              ERC-7579 Modular Smart Wallets                  │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Convex Backend (BaaS)                      │
│          Mutations / Queries / Scheduled Actions             │
│              KYC/AML · CVM 88 Compliance Logic               │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│               Base Blockchain (EVM L2)                       │
│          ERC-20 Real Estate Tokens (current)                 │
│          BRLA Stablecoin Settlement (BRL 1:1)                │
└─────────────────────────────────────────────────────────────┘
```

---

## Target Architecture (Stellar/Soroban Integration)

```
┌─────────────────────────────────────────────────────────────┐
│                    INVESTOR (Brazil / Global)                │
│              + Freighter / Albedo Stellar Wallet             │
└──────────────────┬────────────────────────┬─────────────────┘
                   │ Brazilian investors     │ International investors
                   │ (BRL via PIX)           │ (USD/EUR via anchor)
                   ▼                         ▼
┌─────────────────────────────────────────────────────────────┐
│              BWB Platform — app.bwbi.com.br                  │
│      React + Privy + Stellar Wallet Adapter (Freighter)      │
│              frontend/src/components/StellarWallet/          │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Convex Backend (BaaS)                      │
│                backend/stellar/mutations.ts                  │
│                 backend/stellar/queries.ts                   │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              BWB Stellar SDK — sdk/src/                      │
│   client.ts · token.ts · kyc.ts · anchor.ts                 │
│         @stellar/stellar-sdk + Soroban RPC                  │
└──────────┬────────────────┬──────────────┬──────────────────┘
           │                │              │
           ▼                ▼              ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐
│   Soroban    │  │   Soroban    │  │   Soroban            │
│   Contract   │  │   Contract   │  │   Contract           │
│real-estate   │  │kyc-whitelist │  │  distribution        │
│   -token     │  │              │  │                      │
│              │  │ KYC-gated    │  │ Programmatic yield   │
│ Token mint   │  │ transfers    │  │ distributions to     │
│ + metadata   │  │ (CVM 88)     │  │ all holders          │
└──────────────┘  └──────────────┘  └──────────────────────┘
           │                │              │
           └────────────────┴──────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Stellar Network (Mainnet)                  │
│               Horizon API · Soroban RPC                     │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              BRLA Anchor — BRLA Digital Ltda.               │
│         SEP-31 (cross-border) + SEP-24 (interactive)        │
│              BRL ↔ BRLA on Stellar ↔ USD/EUR                │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Descriptions

### Soroban Contracts

| Contract | Path | Purpose | Tranche |
|---|---|---|---|
| `real-estate-token` | `contracts/real-estate-token/` | RWA token issuance + transfer | T1 |
| `kyc-whitelist` | `contracts/kyc-whitelist/` | KYC-gated transfer restrictions | T1 |
| `distribution` | `contracts/distribution/` | Programmatic yield distributions | T3 |

### SDK Modules

| Module | Path | Purpose |
|---|---|---|
| `client.ts` | `sdk/src/client.ts` | Stellar/Horizon connection + Soroban RPC |
| `token.ts` | `sdk/src/token.ts` | Token mint, transfer, balance queries |
| `kyc.ts` | `sdk/src/kyc.ts` | Whitelist management |
| `anchor.ts` | `sdk/src/anchor.ts` | BRLA anchor SEP-24/31 integration |

### Backend (Convex)

| Module | Path | Purpose |
|---|---|---|
| `mutations.ts` | `backend/stellar/mutations.ts` | Token issuance, KYC, distributions |
| `queries.ts` | `backend/stellar/queries.ts` | Status, balances, transaction history |

---

## Data Flow — Token Issuance

```
1. Offering approved by CVM 88
2. Convex mutation triggered → backend/stellar/mutations.ts
3. SDK calls real-estate-token contract → mint(amount, metadata)
4. KYC-whitelist contract verifies investor address
5. Token transferred to investor Stellar wallet
6. Event emitted → Horizon indexer → frontend balance update
7. BRLA anchor settles payment (BRL/USD → BRLA → contract)
```

## Data Flow — Yield Distribution

```
1. Yield event triggered (quarterly / on-demand)
2. Convex scheduled action → distribution contract
3. Contract iterates all holders (proportional calculation)
4. BRLA distributed to each holder wallet on Stellar
5. Distribution logged on-chain (auditable)
6. Investor sees balance update in portal
```

---

## Why Soroban vs EVM for Distributions

| Metric | EVM (current) | Soroban (target) |
|---|---|---|
| Gas per distribution (100 holders) | ~$50–200 | ~$0.01–0.10 |
| KYC-gated transfers | Custom logic required | Native account authorization |
| Settlement finality | ~12 seconds | ~5 seconds |
| BRLA anchor integration | Custom bridge required | Native SEP anchor |

---

## Security Considerations

- All contract upgrades require multi-sig admin authorization
- KYC whitelist is admin-only (BWB + CVM compliance team)
- Audit required before mainnet deployment (see `audit/`)
- Transfer restrictions enforced at contract level — cannot be bypassed by frontend
