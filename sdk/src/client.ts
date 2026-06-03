// BWB Stellar Client — Horizon API + Soroban RPC connection
// STATUS: Scaffold — Tranche 2 implementation

import { Horizon, SorobanRpc, Networks } from '@stellar/stellar-sdk';

export type StellarNetwork = 'testnet' | 'mainnet';

export interface BWBStellarConfig {
  network: StellarNetwork;
  horizonUrl?: string;
  sorobanRpcUrl?: string;
}

const DEFAULTS: Record<StellarNetwork, { horizon: string; soroban: string }> = {
  testnet: {
    horizon: 'https://horizon-testnet.stellar.org',
    soroban: 'https://soroban-testnet.stellar.org',
  },
  mainnet: {
    horizon: 'https://horizon.stellar.org',
    soroban: 'https://soroban-mainnet.stellar.org',  // Update when available
  },
};

export class BWBStellarClient {
  public readonly horizon: Horizon.Server;
  public readonly soroban: SorobanRpc.Server;
  public readonly network: StellarNetwork;
  public readonly networkPassphrase: string;

  constructor(config: BWBStellarConfig) {
    this.network = config.network;
    this.networkPassphrase =
      config.network === 'mainnet' ? Networks.PUBLIC : Networks.TESTNET;

    this.horizon = new Horizon.Server(
      config.horizonUrl ?? DEFAULTS[config.network].horizon
    );
    this.soroban = new SorobanRpc.Server(
      config.sorobanRpcUrl ?? DEFAULTS[config.network].soroban
    );
  }

  async getAccountBalance(publicKey: string): Promise<string> {
    const account = await this.horizon.loadAccount(publicKey);
    const xlm = account.balances.find((b) => b.asset_type === 'native');
    return xlm?.balance ?? '0';
  }

  async getTransactionStatus(hash: string): Promise<SorobanRpc.Api.GetTransactionResponse> {
    return this.soroban.getTransaction(hash);
  }
}
