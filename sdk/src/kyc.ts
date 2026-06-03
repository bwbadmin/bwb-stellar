// BWB KYC Whitelist SDK — Soroban contract interactions
// STATUS: Scaffold — Tranche 2 implementation

import { Contract, TransactionBuilder, BASE_FEE } from '@stellar/stellar-sdk';
import { BWBStellarClient } from './client';

export type InvestorCategory = 'Retail' | 'Qualified' | 'Professional';

export interface WhitelistEntry {
  investorCategory: InvestorCategory;
  approvedAt: number;
  approvedBy: string;
}

export class KYCWhitelistClient {
  private contract: Contract;
  private stellar: BWBStellarClient;

  constructor(stellar: BWBStellarClient, contractAddress: string) {
    this.stellar = stellar;
    this.contract = new Contract(contractAddress);
  }

  /**
   * Check if an address is KYC-approved
   * Called before any token transfer
   */
  async isApproved(investorAddress: string): Promise<boolean> {
    // TODO: Tranche 2 — invoke kyc-whitelist contract is_ok() function
    throw new Error('Not implemented — Tranche 2 deliverable');
  }

  /**
   * Add investor to KYC whitelist (admin only)
   * Triggered by Convex backend on KYC approval
   */
  async addToWhitelist(
    adminKeypair: any,
    investorAddress: string,
    category: InvestorCategory
  ): Promise<string> {
    // TODO: Tranche 2 — invoke kyc-whitelist contract add() function
    throw new Error('Not implemented — Tranche 2 deliverable');
  }

  /**
   * Remove investor from KYC whitelist (admin only)
   * Called on KYC revocation or compliance action
   */
  async removeFromWhitelist(
    adminKeypair: any,
    investorAddress: string
  ): Promise<string> {
    // TODO: Tranche 2 — invoke kyc-whitelist contract remove() function
    throw new Error('Not implemented — Tranche 2 deliverable');
  }
}
