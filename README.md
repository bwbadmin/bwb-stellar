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
- **CVM Resolution 88** authorization — Brazil's SEC/FCA equivalent

---

## Why Stellar is Core

Stellar is not a superficial addition to BWB. It addresses fundamental infrastructure limitations of our current EVM deployment:

1. **Cost efficiency** — Soroban smart contracts enable high-frequency micro-distributions to hundreds of token holders at a fraction of EVM gas costs, critical for quarterly yield distributions.

2. **Built-in compliance primitives** — Soroban's native account authorization framework maps directly to CVM 88's KYC-gated transfer requirements.

3. **Institutional stablecoin settlement** — Stellar's established anchor infrastructure enables BRLA-to-USD/EUR settlement paths for international investors.

4. **Ecosystem alignment** — Stellar's focus on regulated financial services and underserved LATAM markets directly aligns with BWB's mission.

---

## Architecture

```
Investor (global)
      │
      ▼
BWB Platform (app.bwbi.com.br)
  React + Privy + Notus ERC-7579
      │
      ▼
Convex Backend ──────────────────────────────────────────┐
      │                                                   │
      ▼                                                   ▼
Base Blockchain (current)              Stellar Network (integration)
  ERC-20 tokens                          Soroban Smart Contracts
  BRLA settlement                          real-estate-token
                                           kyc-whitelist
                                           distribution
                                              │
                                              ▼
                                        BRLA Anchor (SEP-31)
                                        BRL ↔ BRLA ↔ USD/EUR
```

See [docs/architecture.md](docs/architecture.md) for full system diagram.

---

## Repository Structure

```
bwb-stellar/
├── contracts/                  # Soroban smart contracts (Rust)
│   ├── real-estate-token/      # RWA token — main issuance contract
│   ├── kyc-whitelist/          # KYC-gated transfer restrictions (CVM 88)
│   └── distribution/           # Programmatic yield distributions
├── sdk/                        # TypeScript — Stellar SDK integration
├── backend/                    # Convex backend → Stellar bridge
├── frontend/                   # Investor portal — Stellar wallet adapter
├── scripts/                    # Deploy scripts (testnet + mainnet)
├── tests/                      # Unit, integration and compliance tests
├── docs/                       # Technical documentation (EN)
└── audit/                      # Security audit reports
```

---

## SCF Grant — Integration Track

This project is being developed under the **Stellar Community Fund (SCF) Build Award — Integration Track**.

| Tranche | Milestone | Timeline | Value |
|---|---|---|---|
| T1 (10%) | Soroban Contract Dev | Weeks 1–4 | $15,000 |
| T2 (20%) | Testnet Integration | Weeks 5–8 | $30,000 |
| T3 (30%) | Mainnet Launch | Weeks 9–14 | $45,000 |
| T4 (40%) | Traction + Report | Weeks 15–20 | $60,000 |

See [docs/scf-deliverables.md](docs/scf-deliverables.md) for detailed deliverables.

---

## Market Opportunity

Brazil's tokenized securities market grew **55x in 3 years**:
- 2022: R$7M
- 2025: R$3.9B (861 active offerings)
- Total addressable market: **$128.6B** in Brazilian real estate

BWB is the **only CVM-authorized platform** bringing this institutional-grade pipeline to the Stellar ecosystem.

---

## License

Apache 2.0 — see [LICENSE](LICENSE)

---

## Contact

- **Website:** [bwbi.com.br](https://bwbi.com.br)
- **Platform:** [app.bwbi.com.br](https://app.bwbi.com.br)
- **Email:** contato@bwbi.com.br
- **Location:** Jaraguá do Sul, SC, Brazil
