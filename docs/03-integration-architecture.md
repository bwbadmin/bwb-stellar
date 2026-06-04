# Integration Architecture — BWB + Stellar

This document describes how BWB's existing platform connects to the Stellar network. The integration adds Stellar as a parallel settlement layer alongside the current Base (EVM) deployment.

---

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    INVESTOR                                  │
│           Brazil (PIX) · Portugal/EU (EURC)                  │
└──────────────────┬──────────────────────────────────────────┘
                   │ HTTPS
                   ▼
┌─────────────────────────────────────────────────────────────┐
│              BWB Platform — app.bwbi.com.br                  │
│         React 19 + Privy (auth) + Stellar Wallet Adapter     │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Convex Backend                             │
│           TypeScript serverless — real-time                  │
│     IRampProvider interface → TransferoRampProviderImpl      │
└──────────┬──────────────────────────────────────────────────┘
           │ BWB Stellar SDK (sdk/src/)
           ▼
┌─────────────────────────────────────────────────────────────┐
│              Soroban RPC + Horizon API                       │
│      Futurenet (dev) · Testnet · Mainnet                     │
└──────────┬────────────────┬───────────────────────────────┬─┘
           ▼                ▼                               ▼
  real-estate-token    kyc-whitelist              distribution
  (one per offering)   (shared registry)          (BRLA yield)
```

---

## Key Components

### 1. Privy Server Wallet — Operator Keypair

BWB's existing Privy integration already manages a Platform Server Wallet for EVM operations. For Stellar, the same Privy Server Wallet manages an Ed25519 keypair that acts as the **Operator**.

The Operator is the hot wallet that:
- Calls `kyc-whitelist::add` and `remove` when KYC events arrive
- Calls `real-estate-token::mint` after payment confirmation
- Triggers `distribution::distribute` on yield events

A separate cold wallet (hardware key) holds the Admin role, controlling contract upgrades and admin handover.

This mirrors the existing EVM architecture (`PlatformServerWallet` + relay) — no new custody model is required.

### 2. Transfero BaaSic API — BRL Ramp

Transfero replaces Avenia (which operates on Base/EVM only) as the fiat ramp for Stellar.

```
Investor → PIX payment
  │
  └── Transfero BaaSic API
        BRL → BRLA (Stellar, 1:1 peg)
        BRLA credited to BWB reserve address
          │
          └── Convex webhook handler
                payment confirmed → mint tokens
```

- **BRLA** is Transfero's BRL-pegged stablecoin, native on Stellar
- **BRZ** is also available on Stellar via Transfero (used for legacy flows)
- The Convex workpool handles Transfero webhook events with automatic retry

Transfero API access is via B2B contract — sandbox access is part of T2 deliverables.

### 3. Stellar DEX — EURC/BRLA for European Investors

```
EU Investor → EUR → EURC (Circle)
  │
  └── Stellar DEX atomic path payment
        EURC → BRLA (no custodian, on-chain price discovery)
          │
          └── Same mint flow as Brazilian investors
```

EURC (Circle) became natively available on Stellar in May 2026 via CCTP. The Stellar DEX provides EURC/BRLA liquidity for cross-border investment flows without any custodial bridge or off-chain settlement.

### 4. BWB Stellar SDK (`sdk/src/`)

The SDK is the TypeScript layer between the Convex backend and Soroban contracts.

| Module | Responsibility |
|---|---|
| `client.ts` | Soroban RPC connection with multi-endpoint fallback |
| `token.ts` | `mint`, `transfer`, `balance`, `total_supply`, `get_offering` |
| `kyc.ts` | `add`, `remove`, `is_ok`, `get_entry` |
| `anchor.ts` | BRLA/BRZ Transfero integration helpers |

The SDK produces unsigned XDRs for most operations — the Convex backend signs them with the Operator keypair via Privy Server Wallet.

---

## Data Flow — Token Issuance (Brazilian Investor)

```
1.  Investor completes KYC on BWB platform (Avenia — existing)
2.  KYC approved → Convex mutation fires
3.  SDK: kyc-whitelist::add(investor_stellar_address, category)
         signed by Operator keypair (Privy Server Wallet)
4.  Investor selects offering, confirms investment amount
5.  PIX payment initiated → Transfero BaaSic API
6.  Transfero webhook → Convex workpool
         payment confirmed (BRL received)
7.  SDK: real-estate-token::mint(investor_stellar_address, token_amount)
         [internal] kyc-whitelist::is_ok verified → proceeds
8.  Stellar event → Horizon indexer → frontend balance update
9.  Investor sees token balance in BWB portal
```

---

## Data Flow — Yield Distribution

```
1.  Offering matures / quarterly yield event
2.  Finance team approves distribution amount (BRLA)
3.  Convex scheduled action fires
4.  SDK: distribution::distribute(token_contract, brla_amount)
         contract reads total_supply
         iterates all holders
         sends proportional BRLA to each address
5.  On-chain distribution log (count, amount, timestamp)
6.  Investors see BRLA balance increase in Stellar wallet
7.  Optional: investors bridge BRLA → BRL via Transfero (PIX withdrawal)
```

---

## Environment Configuration

| Variable | Description |
|---|---|
| `STELLAR_NETWORK` | `testnet` or `mainnet` |
| `SOROBAN_RPC_URL` | Primary RPC endpoint (with fallback list) |
| `HORIZON_URL` | Horizon API URL |
| `KYC_CONTRACT_ID` | Deployed kyc-whitelist contract address |
| `OPERATOR_PUBLIC_KEY` | Operator Stellar public key (Ed25519) |

Private keys are never stored in this repository. The Operator private key is managed exclusively by Privy Server Wallet in the BWB private backend.

---

## Separation of Concerns

```
bwb-stellar (this repo — public)
  ├── contracts/   Apache 2.0 — permanent open source
  ├── sdk/         Apache 2.0 — permanent open source
  └── docs/        Apache 2.0 — permanent open source

bwb-tokenization (private repo)
  ├── frontend/    React app — proprietary
  ├── backend/     Convex backend — proprietary
  └── contracts/   EVM contracts — proprietary
```

The private backend consumes the public SDK as a dependency. The Soroban contracts have no dependency on the private backend — they can be used independently by any Stellar project.
