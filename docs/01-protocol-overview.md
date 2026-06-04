# BWB on Stellar — Protocol Overview

## What BWB Is

BWB Digital Assets is a **CVM Resolution 88-authorized** electronic investment platform that tokenizes Brazilian real estate for retail, qualified, and institutional investors. Operating since 2024, BWB has facilitated over R$4M in real estate fundraises with zero defaults across three closed offerings.

BWB is integrating **Stellar/Soroban** as the settlement and token issuance layer for its next generation of offerings — replacing the current Base (EVM L2) infrastructure.

---

## Why Stellar

Stellar is not a superficial addition. It resolves specific limitations of the current EVM deployment:

| Problem (EVM) | Solution (Stellar) |
|---|---|
| Gas costs make micro-distributions expensive (~$50–200 per batch of 100 holders) | Soroban transactions cost fractions of a cent — viable for quarterly yield distributions at scale |
| KYC-gated transfers require custom on-chain logic | Soroban's native account authorization framework maps directly to CVM 88's transfer restrictions |
| BRL stablecoin settlement requires custom bridges | BRLA exists natively on Stellar via Transfero — no bridge required |
| No native DEX for cross-border flows | Stellar DEX enables EURC → BRLA settlement for European investors |

---

## Use Cases

### Brazil — Domestic Retail and Qualified Investors

```
Investor (Brazil)
  │
  ├── PIX payment → Transfero BaaSic API
  │     BRL → BRLA (on Stellar, 1:1 BRL peg)
  │
  └── Convex Backend → BWB Stellar SDK
        mint() → real-estate-token contract
        investor receives tokens in Stellar wallet
```

1. Investor completes KYC on BWB platform (Avenia — existing)
2. Operator adds investor address to `kyc-whitelist` contract
3. Investor pays via PIX → Transfero converts BRL to BRLA on Stellar
4. Backend mints real estate tokens to investor's Stellar address
5. Quarterly distributions: `distribution` contract sends BRLA proportionally to all holders

### Portugal / Europe — Cross-Border Investment

```
Investor (Portugal / EU)
  │
  ├── EUR → EURC (Circle, native on Stellar since May 2026)
  │     Stellar DEX: EURC → BRLA (atomic swap, no custodian)
  │
  └── Same flow as Brazil from step 3 onward
```

EURC arrived on Stellar in May 2026 via Circle CCTP. The Stellar DEX provides native EURC/BRLA liquidity, enabling European investors to acquire Brazilian real estate tokens without any custodial bridge.

---

## Track Record

| Offering | Raised | Target IRR | Status |
|---|---|---|---|
| ARTP-HS | R$2.5M | 26.8% p.a. | Closed — 100% distributed |
| HAUS-06 | R$1.18M | 20.5% p.a. | Closed — 100% distributed |
| ARTP-DT | R$1.5M | 20.0% p.a. | Closed — active |

- R$4M+ total transaction volume (2025)
- Zero defaults, zero restructurings
- 7+ institutional partners (developers, originators)
- CVM Resolution 88 authorization — Brazil's primary capital markets regulation for tokenized securities

---

## Compliance Model — CVM Resolution 88

CVM Resolution 88 (2023) regulates the issuance and distribution of tokenized securities in Brazil. Key requirements that BWB enforces on-chain:

1. **KYC mandatory** — transfers may only occur between CVM-verified investors
2. **Investor categories** — Retail / Qualified (R$1M+) / Professional (R$10M+) — different investment limits per category
3. **Transfer restrictions** — tokens cannot be sent to unverified addresses
4. **Offering authorization** — each offering requires a CVM authorization code, stored in token metadata on-chain

These constraints are enforced by the `kyc-whitelist` and `real-estate-token` Soroban contracts — not by the frontend. A compromised frontend cannot bypass them.

---

## Open Source Commitment

All Soroban smart contracts in this repository (`contracts/`) are released under the **Apache 2.0 license** and will remain permanently open source. The BWB application code (React frontend, Convex backend, EVM contracts) is proprietary and is not part of this repository.

This separation ensures:
- The Stellar community can audit, fork, and extend the contracts
- Other Brazilian RWA projects can adapt the CVM 88 compliance pattern for their own Stellar deployments
- The open source contracts serve as a public reference implementation for regulated securities on Soroban

---

## SCF Integration Track — Building Blocks

BWB's Stellar deployment integrates the following building blocks from the SCF Integration List:

| Building Block | Role |
|---|---|
| [Privy](https://www.privy.io/) | Auth + Stellar Ed25519 keypair management (Operator + investor wallets) |
| [Stellar Wallets Kit](https://stellarwalletskit.dev/) | Freighter/Albedo wallet adapter in the investor portal |
| [Abroad](https://www.abroad.finance/) | BRL ↔ stablecoin on/off-ramp for Brazilian investors via PIX |

The Soroban smart contracts are the connective tissue that enforces CVM 88 compliance across these integrations.

---

## Roadmap

| Phase | Timeline | Milestone |
|---|---|---|
| T1 | Weeks 1–5 | Soroban contracts + SDK + Privy/SWK integrations on testnet |
| T2 | Weeks 6–10 | Full PIX→token flow on testnet + distribution contract |
| T3 | Weeks 11–14 | Mainnet launch + first live offering + security audit |

See [scf-deliverables.md](scf-deliverables.md) for detailed deliverables per tranche.
