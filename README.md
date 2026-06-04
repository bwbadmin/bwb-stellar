# BWB Real Estate on Stellar

> **CVM-regulated Brazilian real estate tokenization on Stellar — R$4M+ track record, institutional-grade compliance.**

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![SCF](https://img.shields.io/badge/SCF-Integration%20Track-brightgreen)](https://communityfund.stellar.org)
[![Regulation](https://img.shields.io/badge/Regulation-CVM%20Resolution%2088-orange)](https://bwbi.com.br)

---

## Overview

BWB Digital Assets is a **CVM Resolution 88-authorized** electronic investment platform that tokenizes Brazilian real estate for global investors. We are integrating **Stellar/Soroban** as the core settlement and token issuance layer, bringing regulated real estate securities to the Stellar ecosystem.

### Track Record

| Offering | Raised | IRR p.a. | Status |
|---|---|---|---|
| ARTP-HS | R$2.5M | 26.8% | Closed |
| HAUS-06 | R$1.18M | 20.5% | Closed |
| ARTP-DT | R$1.5M | 20.0% | Closed |

- **R$4M+** in transaction volume in 2025
- **Zero defaults**, zero restructurings
- **7+ institutional partners**
- **CVM Resolution 88** authorization — Brazil's primary regulation for tokenized securities

---

## Why Stellar is Core

Stellar is not a superficial addition to BWB. It addresses fundamental infrastructure limitations of our current EVM deployment:

1. **Cost efficiency** — Soroban smart contracts enable high-frequency micro-distributions to hundreds of token holders at a fraction of EVM gas costs, critical for quarterly yield distributions.

2. **Built-in compliance primitives** — Soroban's native account authorization framework maps directly to CVM 88's KYC-gated transfer requirements.

3. **Native stablecoin settlement** — BRLA (Transfero) exists natively on Stellar. EURC (Circle) arrived on Stellar in May 2026. No bridges required.

4. **Ecosystem alignment** — Stellar's focus on regulated financial services and underserved LATAM markets directly aligns with BWB's mission.

---

## Architecture

```
Investor (Brazil: PIX · Portugal/EU: EURC)
      │
      ▼
BWB Platform (app.bwbi.com.br)
  React + Privy Auth + Stellar Wallet Adapter
      │
      ▼
Convex Backend ──── Transfero BaaSic API (BRL → BRLA)
      │                    │
      │              PIX confirmed
      ▼                    ▼
BWB Stellar SDK (sdk/src/) ──── Soroban RPC
      │
      ├── real-estate-token  (one per offering)
      ├── kyc-whitelist      (shared CVM 88 registry)
      └── distribution       (programmatic BRLA yield)
```

See [docs/03-integration-architecture.md](docs/03-integration-architecture.md) for full system diagram and data flows.

---

## Repository Structure

```
bwb-stellar/
├── contracts/                  # Soroban smart contracts (Rust) — Apache 2.0
│   ├── real-estate-token/      # RWA token — CVM 88 compliant issuance
│   ├── kyc-whitelist/          # KYC-gated transfer restrictions
│   └── distribution/           # Programmatic yield distributions (BRLA)
├── sdk/                        # TypeScript — BWB Stellar SDK
├── scripts/                    # Deploy scripts (testnet + mainnet)
├── docs/                       # Technical documentation
└── audit/                      # Security audit reports
```

---

## Documentation

| Document | Description |
|---|---|
| [docs/01-protocol-overview.md](docs/01-protocol-overview.md) | What BWB does on Stellar — use cases BR + PT, compliance model |
| [docs/02-contracts.md](docs/02-contracts.md) | Contract specs — functions, storage, invariants, CVM 88 enforcement |
| [docs/03-integration-architecture.md](docs/03-integration-architecture.md) | Transfero + Operator + Soroban integration flows |
| [docs/scf-deliverables.md](docs/scf-deliverables.md) | SCF tranche structure and verification methods |
| [docs/kyc-flow.md](docs/kyc-flow.md) | KYC flow — CVM 88 on-chain |

---

## Open Source

All Soroban smart contracts in `contracts/` are licensed **Apache 2.0** and will remain permanently open source. This is a hard commitment — not contingent on SCF approval.

The BWB application (React frontend, Convex backend, EVM contracts) is proprietary and is not part of this repository.

---

## SCF Grant — Integration Track

This project is being submitted to the **Stellar Community Fund (SCF) Build Award — Integration Track**.

**Official integrations from the SCF Integration List:**
- [Privy](https://www.privy.io/) — auth + embedded wallets (already in production, extending to Stellar)
- [Stellar Wallets Kit](https://stellarwalletskit.dev/) — Freighter/Albedo adapter for the investor portal
- [Abroad](https://www.abroad.finance/) — BRL ↔ stablecoin ramp for Brazilian investors

| Tranche | Milestone | Timeline |
|---|---|---|
| T0 (10%) | SCF approval | On award |
| T1 (20%) | Soroban contracts + SDK + Privy/SWK integrations on testnet | Weeks 1–5 |
| T2 (30%) | Full PIX→token flow on testnet + distribution contract | Weeks 6–10 |
| T3 (40%) | Mainnet launch + first live offering + audit | Weeks 11–14 |

See [docs/scf-deliverables.md](docs/scf-deliverables.md) for detailed deliverables and verification methods.

---

## Market Opportunity

Brazil's tokenized securities market grew **55x in 3 years**:
- 2022: R$7M
- 2025: R$3.9B (861 active offerings)
- Total addressable market: **$128.6B** in Brazilian real estate

BWB is the **only CVM-authorized platform** actively integrating this institutional-grade pipeline into the Stellar ecosystem.

---

## Developer Setup

```bash
# Clone the repo
git clone https://github.com/bwbadmin/bwb-stellar.git
cd bwb-stellar

# Install git hooks (IP guard)
./scripts/install-hooks.sh

# Build Soroban contracts
cd contracts/kyc-whitelist && cargo test
cd contracts/real-estate-token && cargo test

# Install SDK dependencies
cd sdk && npm install && npm test
```

---

## License

Apache 2.0 — see [LICENSE](LICENSE)

---

## Contact

- **Website:** [bwbi.com.br](https://bwbi.com.br)
- **Platform:** [app.bwbi.com.br](https://app.bwbi.com.br)
- **Email:** contato@bwbi.com.br
- **Location:** Jaraguá do Sul, SC, Brazil
