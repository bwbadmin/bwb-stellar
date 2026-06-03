# BRLA Anchor Integration — Stellar SEP

## Overview

BRLA is a BRL-pegged stablecoin (1:1) issued by **BRLA Digital Ltda.**, currently used by BWB for on-chain settlement on Base blockchain. This document describes the integration of BRLA as a Stellar anchor for the BWB platform.

---

## Why BRLA on Stellar

| Feature | Base (current) | Stellar + BRLA Anchor |
|---|---|---|
| Settlement currency | BRLA (ERC-20 on Base) | BRLA (Stellar anchor) |
| International investor access | Limited — requires EVM wallet | Native — SEP-31 cross-border |
| Settlement finality | ~12 seconds | ~5 seconds |
| Transaction cost | ~$0.10–2.00 gas | ~$0.0001 |
| Regulatory path | BRL only | BRL + USD/EUR via anchor |

---

## Protocol Options

### SEP-24 — Interactive Anchor
- User-facing flow with hosted UI (iframe)
- Investor deposits BRL via PIX → receives BRLA on Stellar
- Investor withdraws BRLA → receives BRL via PIX
- **Best for:** Brazilian retail investors using the BWB portal

### SEP-31 — Cross-Border Payments
- Direct API integration (no hosted UI)
- International investor sends USD/EUR → receives BRLA on Stellar
- Settlement via correspondent banking relationship
- **Best for:** International qualified/institutional investors

### Decision Required
> See GitHub Issue #4: [Decisao: BRLA anchor SEP-24 ou SEP-31?](https://github.com/bwbadmin/bwb-stellar/issues/4)

---

## Integration Architecture

```
Brazilian Investor              International Investor
       │                                  │
       │ PIX (BRL)                        │ Wire (USD/EUR)
       ▼                                  ▼
BRLA Digital Anchor ◄─────────────────────┘
  (SEP-24 / SEP-31)
       │
       │ BRLA on Stellar
       ▼
BWB real-estate-token contract
  (payment for token purchase)
       │
       │ Token
       ▼
Investor Stellar Wallet
  (Freighter / Albedo)
       │
       │ Quarterly yield (BRLA)
       ▼
distribution contract
  → BRLA back to investor wallet
```

---

## SDK Implementation Plan

```typescript
// sdk/src/anchor.ts

interface AnchorConfig {
  homeDomain: string;      // BRLA Digital anchor domain
  stellarNetwork: 'testnet' | 'mainnet';
  assetCode: 'BRLA';
  assetIssuer: string;     // BRLA issuer address on Stellar
}

// SEP-24: Interactive deposit flow
async function initiateDeposit(
  amount: number,           // BRL amount
  investorAddress: string,  // Stellar address
  pixKey?: string          // Optional PIX key for Brazilian investors
): Promise<DepositTransaction>

// SEP-31: Direct cross-border payment
async function initiateXBorderPayment(
  amountIn: number,         // USD/EUR amount
  currencyIn: 'USD' | 'EUR',
  investorAddress: string,  // Stellar address
  kycFields: KYCFields
): Promise<CrossBorderTransaction>

// Check transaction status
async function getTransactionStatus(
  transactionId: string
): Promise<TransactionStatus>
```

---

## BRLA Asset on Stellar Testnet

> Testnet BRLA asset details to be confirmed with BRLA Digital Ltda. before Tranche 2 deployment.

Expected format:
```
Asset Code: BRLA
Asset Issuer: G... (to be confirmed)
Anchor Home Domain: brla.digital (to be confirmed)
```

---

## Compliance Notes

- BRLA is a regulated stablecoin under Brazilian law (Banco Central do Brasil oversight)
- All BRLA deposits/withdrawals require KYC at the anchor level
- BWB KYC is separate from anchor KYC — investors must pass both
- CVM 88 settlement in BRL is maintained via BRLA 1:1 peg
- International investors must complete additional AML screening

---

## References

- [Stellar Anchor Network](https://resources.stellar.org/anchors)
- [SEP-24 Specification](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0024.md)
- [SEP-31 Specification](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0031.md)
- [BRLA Digital](https://brla.digital)
