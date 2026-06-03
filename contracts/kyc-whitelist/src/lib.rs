// BWB KYC Whitelist — Soroban Smart Contract
// CVM Resolution 88 on-chain compliance gate
// License: Apache 2.0
//
// STATUS: SCAFFOLD — implementation pending dev Soroban hire (see GitHub Issue #2)
// See docs/kyc-flow.md and docs/soroban-contracts.md for full specification

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, symbol_short};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum InvestorCategory {
    Retail,       // CVM 88: standard retail investor
    Qualified,    // CVM 88: R$1M+ financial assets
    Professional, // CVM 88: institutional / R$10M+
}

#[contracttype]
#[derive(Clone)]
pub struct WhitelistEntry {
    pub investor_category: InvestorCategory,
    pub approved_at: u64,
    pub approved_by: Address,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Entry(Address),
}

#[contract]
pub struct KYCWhitelist;

#[contractimpl]
impl KYCWhitelist {
    /// Initialize the KYC whitelist contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Add an investor to the KYC whitelist (admin only)
    pub fn add(env: Env, address: Address, category: InvestorCategory) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let entry = WhitelistEntry {
            investor_category: category,
            approved_at: env.ledger().timestamp(),
            approved_by: admin,
        };
        env.storage().persistent().set(&DataKey::Entry(address.clone()), &entry);

        env.events().publish((symbol_short!("kyc_add"),), address);
    }

    /// Remove an investor from the KYC whitelist (admin only)
    pub fn remove(env: Env, address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage().persistent().remove(&DataKey::Entry(address.clone()));
        env.events().publish((symbol_short!("kyc_rm"),), address);
    }

    /// Check if an address is KYC-approved (called by real-estate-token)
    pub fn is_ok(env: Env, address: Address) -> bool {
        env.storage().persistent().has(&DataKey::Entry(address))
    }

    /// Get whitelist entry details
    pub fn get_entry(env: Env, address: Address) -> Option<WhitelistEntry> {
        env.storage().persistent().get(&DataKey::Entry(address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::Env;

    #[test]
    fn test_add_and_verify() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, KYCWhitelist);
        let client = KYCWhitelistClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        assert!(!client.is_ok(&investor));

        client.add(&investor, &InvestorCategory::Retail);
        assert!(client.is_ok(&investor));
    }

    #[test]
    fn test_remove() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, KYCWhitelist);
        let client = KYCWhitelistClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&investor, &InvestorCategory::Qualified);
        assert!(client.is_ok(&investor));

        client.remove(&investor);
        assert!(!client.is_ok(&investor));
    }
}
