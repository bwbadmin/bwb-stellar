// BWB Real Estate Token — Soroban Smart Contract
// CVM Resolution 88 compliant RWA token on Stellar
// License: Apache 2.0
//
// STATUS: SCAFFOLD — implementation pending dev Soroban hire (see GitHub Issue #2)
// See docs/soroban-contracts.md for full interface specification

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct OfferingMetadata {
    pub offering_id: String,
    pub property_address: String,
    pub total_raise: i128,
    pub target_irr_bps: u32,  // basis points e.g. 2080 = 20.80%
    pub maturity_date: u64,
    pub cvm_authorization: String,
}

#[contracttype]
pub enum DataKey {
    Admin,
    KycContract,
    TotalSupply,
    Balance(Address),
    Metadata,
    Paused,
}

#[contract]
pub struct RealEstateToken;

#[contractimpl]
impl RealEstateToken {
    /// Initialize the token contract
    pub fn initialize(
        env: Env,
        admin: Address,
        kyc_contract: Address,
        metadata: OfferingMetadata,
    ) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::KycContract, &kyc_contract);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().set(&DataKey::Metadata, &metadata);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    /// Mint tokens to a KYC-verified investor
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // Verify recipient is KYC-approved (CVM 88)
        Self::require_kyc_approved(&env, &to);

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        env.storage().persistent()
            .set(&DataKey::Balance(to.clone()), &(balance + amount));

        let supply: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));

        env.events().publish((symbol_short!("mint"),), (to, amount));
    }

    /// Transfer tokens between KYC-verified addresses
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        // CVM 88 compliance: verify recipient is KYC-approved
        Self::require_kyc_approved(&env, &to);

        let from_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        let to_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage().persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        env.events().publish((symbol_short!("transfer"),), (from, to, amount));
    }

    /// Query token balance
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    /// Query total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Get offering metadata (CVM authorization, property details)
    pub fn get_offering(env: Env) -> OfferingMetadata {
        env.storage().instance().get(&DataKey::Metadata).unwrap()
    }

    // Internal: require KYC approval from whitelist contract
    fn require_kyc_approved(env: &Env, address: &Address) {
        let kyc_contract: Address = env.storage().instance()
            .get(&DataKey::KycContract)
            .unwrap();
        let approved: bool = env.invoke_contract(
            &kyc_contract,
            &symbol_short!("is_ok"),
            soroban_sdk::vec![env, address.clone().into()],
        );
        assert!(approved, "Transfer rejected: recipient not KYC-approved (CVM 88)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_initialize() {
        // TODO: implement after kyc-whitelist contract is ready
        // See GitHub Issue #6
    }
}
