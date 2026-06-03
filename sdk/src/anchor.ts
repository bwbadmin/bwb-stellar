// BWB BRLA Anchor Integration — SEP-24 / SEP-31
// STATUS: Scaffold — Tranche 3 implementation
// Decision pending: SEP-24 vs SEP-31 (GitHub Issue #4)

export interface AnchorConfig {
  homeDomain: string;       // e.g. 'brla.digital'
  assetCode: 'BRLA';
  assetIssuer: string;      // BRLA issuer address on Stellar
  network: 'testnet' | 'mainnet';
}

export interface DepositResult {
  id: string;
  status: string;
  how: string;              // Payment instructions (PIX key, wire details)
  eta?: number;
}

export interface WithdrawResult {
  id: string;
  status: string;
  accountId: string;        // Stellar account to send BRLA to
  memo?: string;
}

/**
 * SEP-24: Interactive Anchor (deposit/withdraw with hosted UI)
 * Best for: Brazilian retail investors using BWB portal
 */
export class BRLAAnchorSEP24 {
  constructor(private config: AnchorConfig) {}

  /**
   * Initiate BRL deposit (BRL → BRLA on Stellar)
   * Returns URL to hosted deposit UI
   */
  async initiateDeposit(
    investorStellarAddress: string,
    amountBRL: number,
    jwtToken: string
  ): Promise<string> {
    // TODO: Tranche 3 — SEP-24 interactive deposit flow
    // 1. GET anchor/sep24/info to verify BRLA supported
    // 2. POST anchor/sep24/transactions/deposit/interactive
    // 3. Return interactive URL for investor to complete deposit
    throw new Error('Not implemented — Tranche 3 deliverable');
  }

  /**
   * Initiate BRLA withdrawal (BRLA on Stellar → BRL via PIX)
   */
  async initiateWithdraw(
    investorStellarAddress: string,
    amountBRLA: number,
    pixKey: string,
    jwtToken: string
  ): Promise<string> {
    // TODO: Tranche 3 — SEP-24 interactive withdrawal flow
    throw new Error('Not implemented — Tranche 3 deliverable');
  }
}

/**
 * SEP-31: Cross-Border Payments (direct API, no hosted UI)
 * Best for: International institutional investors
 */
export class BRLAAnchorSEP31 {
  constructor(private config: AnchorConfig) {}

  /**
   * Send cross-border payment (USD/EUR → BRLA on Stellar)
   * Used for international investors
   */
  async initiatePayment(
    senderInfo: {
      firstName: string;
      lastName: string;
      email: string;
      country: string;
    },
    receiverStellarAddress: string,
    amountUSD: number
  ): Promise<string> {
    // TODO: Tranche 3 — SEP-31 cross-border payment
    // 1. GET anchor/sep31/info
    // 2. POST anchor/sep31/transactions
    // 3. Return transaction ID for status tracking
    throw new Error('Not implemented — Tranche 3 deliverable');
  }

  async getTransactionStatus(transactionId: string): Promise<string> {
    // TODO: Tranche 3 — GET anchor/sep31/transactions/:id
    throw new Error('Not implemented — Tranche 3 deliverable');
  }
}
