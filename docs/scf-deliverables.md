# SCF Deliverables — BWB Real Estate on Stellar

**Grant:** SCF Build Award — Integration Track  
**Total:** USD 150,000 in XLM  
**Timeline:** 20 weeks (~5 months)  
**Payment:** 4 tranches (10% → 20% → 30% → 40%)

---

## Tranche 1 — Soroban Contract Development
**Value:** $15,000 (10%) | **Timeline:** Weeks 1–4

### Deliverables

| # | Deliverable | Verification Method |
|---|---|---|
| 1.1 | `real-estate-token` Soroban contract — mint, transfer, metadata | GitHub repo + `cargo test` output |
| 1.2 | `kyc-whitelist` Soroban contract — add/remove, verify | GitHub repo + `cargo test` output |
| 1.3 | Unit tests with ≥80% coverage for both contracts | `cargo tarpaulin` coverage report |
| 1.4 | CI pipeline — GitHub Actions build + test + lint | GitHub Actions green badge |
| 1.5 | Architecture documentation in English | `docs/architecture.md` on GitHub |

### Verification
All deliverables verifiable via public GitHub repository `bwbadmin/bwb-stellar` without additional follow-up from BWB team.

---

## Tranche 2 — Testnet Integration
**Value:** $30,000 (20%) | **Timeline:** Weeks 5–8

### Deliverables

| # | Deliverable | Verification Method |
|---|---|---|
| 2.1 | Both contracts deployed on Stellar **testnet** | Contract addresses on Stellar Expert (testnet) |
| 2.2 | TypeScript SDK — `client.ts`, `token.ts`, `kyc.ts` | GitHub repo + `npm test` output |
| 2.3 | Convex backend integration — mutations and queries | Integration test suite passing |
| 2.4 | Compliance test suite — KYC gate, whitelist enforcement | `tests/compliance/` results on GitHub |
| 2.5 | Full technical documentation — all modules | `docs/` folder complete on GitHub |
| 2.6 | KYC flow documentation (CVM 88 on-chain) | `docs/kyc-flow.md` on GitHub |

### Verification
Testnet contract addresses provided. Any SCF evaluator can query them directly via Stellar Expert or Horizon API.

---

## Tranche 3 — Mainnet Launch
**Value:** $45,000 (30%) | **Timeline:** Weeks 9–14

### Deliverables

| # | Deliverable | Verification Method |
|---|---|---|
| 3.1 | Security audit completed (SCF Audit Bank or equivalent) | Audit report in `audit/` on GitHub |
| 3.2 | `distribution` contract — programmatic yield distributions | GitHub repo + mainnet address |
| 3.3 | BRLA anchor integration — SEP-24 and/or SEP-31 | Live transaction on Stellar mainnet |
| 3.4 | Stellar wallet adapter live on investor portal | app.bwbi.com.br — public demo |
| 3.5 | All contracts deployed on Stellar **mainnet** | Contract addresses on Stellar Expert (mainnet) |
| 3.6 | First real estate offering live on Stellar | Offering page on app.bwbi.com.br + on-chain evidence |

### Verification
Mainnet contract addresses on Stellar Expert. First offering visible on BWB platform with on-chain token evidence.

---

## Tranche 4 — Traction + Impact Report
**Value:** $60,000 (40%) | **Timeline:** Weeks 15–20

### Deliverables

| # | Deliverable | Verification Method |
|---|---|---|
| 4.1 | First fundraise via Stellar completed | On-chain transactions + BRLA settlement evidence |
| 4.2 | On-chain metrics dashboard (public) | Public URL with live Stellar data |
| 4.3 | SCF impact report published (English) | Public document / GitHub `docs/scf-impact-report.md` |
| 4.4 | Open-source release — Apache 2.0 license, complete docs | GitHub repo — public, licensed, documented |

### Key Metrics (targets)
- Minimum 1 completed fundraise via Stellar mainnet
- On-chain verifiable: number of token holders, BRLA volume, transaction count
- IRR and offering details published in impact report

### Verification
All metrics verifiable on-chain via Stellar Expert or Horizon API. Impact report publicly accessible.

---

## Budget Breakdown

| Category | Description | Amount (USD) | % |
|---|---|---|---|
| Smart Contract Development | 2 Rust/Soroban devs × 4 months | $60,000 | 40% |
| Backend Integration | BWB-Stellar TypeScript SDK | $25,000 | 17% |
| Frontend/UX | Investor portal + Stellar wallet adapter | $20,000 | 13% |
| Security Audit | Smart contract audit (SCF Audit Bank) | $20,000 | 13% |
| Testing & QA | Testnet, stress tests, compliance tests | $10,000 | 7% |
| Legal/Compliance | Stellar integration vs CVM 88 analysis | $10,000 | 7% |
| Project Management | Coordination and SCF reporting | $5,000 | 3% |
| **TOTAL** | | **$150,000** | **100%** |

> Note: SCF grant covers only Stellar integration costs — not BWB's general operational expenses.

---

## Progress Tracking

| Tranche | Status | Completion |
|---|---|---|
| T1 — Soroban Contract Dev | Not started | 0% |
| T2 — Testnet Integration | Not started | 0% |
| T3 — Mainnet Launch | Not started | 0% |
| T4 — Traction + Report | Not started | 0% |

*This document is updated at the completion of each tranche milestone.*
