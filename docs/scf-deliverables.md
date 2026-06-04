# SCF Deliverables — BWB Real Estate on Stellar

**Grant:** SCF Build Award — Integration Track  
**Total:** Up to USD 150,000 in XLM  
**Timeline:** ~14 weeks  
**Payment structure:** T0 (10% on approval) → T1 (20%) → T2 (30%) → T3 (40%)

---

## Official Integrations (Integration Track requirement)

BWB is integrating the following building blocks from the SCF Integration List:

| Building Block | Category | Role in BWB |
|---|---|---|
| [Privy](https://www.privy.io/) | Wallet Connection | Auth + embedded wallets (already in production on BWB — extending to Stellar) |
| [Stellar Wallets Kit](https://stellarwalletskit.dev/) | Wallet Connection | Freighter/Albedo adapter for investor portal |
| [Abroad](https://www.abroad.finance/) | On/Off-Ramp (LATAM) | BRL ↔ stablecoin ramp for Brazilian investors |

These integrations are the core of BWB's Stellar deployment. The Soroban smart contracts are the "connective tissue" that enforces CVM 88 compliance across these building blocks.

---

## Tranche 0 — Initial Payment (10%)
**Trigger:** Approval + community vote passing  
**Value:** ~$15,000

No deliverables required. Released upon SCF award.

---

## Tranche 1 — Contracts + SDK (20%)
**Value:** ~$30,000 | **Timeline:** Weeks 1–5

| # | Deliverable | Verification |
|---|---|---|
| 1.1 | `kyc-whitelist` Soroban contract — deployed on testnet | Contract address on Stellar Expert (testnet) |
| 1.2 | `real-estate-token` Soroban contract — SEP-0041 compliant, deployed on testnet | Contract address on Stellar Expert (testnet) |
| 1.3 | Unit tests — `cargo test` passing for both contracts | GitHub Actions CI green |
| 1.4 | TypeScript SDK (`client.ts`, `token.ts`, `kyc.ts`) — npm package published | `@bwb-tech/stellar` on npm + `npm test` output |
| 1.5 | Privy integration — Stellar Ed25519 keypair via Privy Server Wallet | Integration test in repo |
| 1.6 | Stellar Wallets Kit integration — Freighter/Albedo connect in investor portal | Live on staging environment |

---

## Tranche 2 — Testnet Integration (30%)
**Value:** ~$45,000 | **Timeline:** Weeks 6–10

| # | Deliverable | Verification |
|---|---|---|
| 2.1 | Full PIX → stablecoin → token flow on testnet (Abroad sandbox) | End-to-end demo recording |
| 2.2 | `distribution` Soroban contract — deployed on testnet | Contract address on Stellar Expert (testnet) |
| 2.3 | KYC flow — CVM 88 categories enforced on-chain (Retail/Qualified/Professional) | Compliance test suite in `tests/compliance/` |
| 2.4 | Convex backend integration — mutations and queries for Stellar | Integration tests passing in CI |
| 2.5 | Complete technical documentation | `docs/` folder on GitHub |

---

## Tranche 3 — Mainnet Launch (40%)
**Value:** ~$60,000 | **Timeline:** Weeks 11–14

| # | Deliverable | Verification |
|---|---|---|
| 3.1 | Security audit completed (SCF Audit Bank or equivalent) | Audit report in `audit/` on GitHub |
| 3.2 | All contracts deployed on Stellar **mainnet** | Contract addresses on Stellar Expert (mainnet) |
| 3.3 | First real estate offering live via Stellar | Offering page on app.bwbi.com.br + on-chain mint evidence |
| 3.4 | Abroad integration live — Brazilian investors funding via BRL on mainnet | Live transaction on Stellar mainnet |
| 3.5 | SCF impact report published | `docs/scf-impact-report.md` on GitHub |

---

## Budget Breakdown

| Category | Description | Amount (USD) | % |
|---|---|---|---|
| Smart Contract Development | Soroban contracts (Rust) — kyc-whitelist, real-estate-token, distribution | $50,000 | 33% |
| Integrations | Privy (Stellar), Stellar Wallets Kit, Abroad ramp | $35,000 | 23% |
| Backend Integration | Convex → BWB Stellar SDK → Soroban | $25,000 | 17% |
| Security Audit | Smart contract audit (SCF Audit Bank) | $20,000 | 13% |
| Testing & QA | Testnet, compliance tests, end-to-end | $10,000 | 7% |
| Project Management | SCF coordination and reporting | $10,000 | 7% |
| **TOTAL** | | **$150,000** | **100%** |

> Budget covers only Stellar integration costs — not BWB's existing operational expenses.

---

## Progress Tracking

| Tranche | Status | Completion |
|---|---|---|
| T0 — Approval | Pending submission | — |
| T1 — Contracts + SDK | Not started | 0% |
| T2 — Testnet Integration | Not started | 0% |
| T3 — Mainnet Launch | Not started | 0% |

*Updated at completion of each tranche milestone.*
