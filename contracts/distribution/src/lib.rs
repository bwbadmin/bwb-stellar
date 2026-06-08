// BWB Distribution — Soroban Smart Contract
// Programmatic yield distributions to real estate token holders
// License: Apache 2.0
//
// STATUS: SCAFFOLD — Tranche 3 deliverable
// See docs/soroban-contracts.md for full specification

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// TTL constants — Stellar mainnet at ~5 second ledger close time
const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30;
const INSTANCE_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 60;

// ─────────────────────────────────────────────────────────────────────────────
// Data types
// ─────────────────────────────────────────────────────────────────────────────

/// Structured event payload emitted once per completed distribution.
/// Tranche 3: populate and emit this only AFTER all BRLA transfers succeed.
#[contracttype]
#[derive(Clone)]
pub struct DistributionEvent {
    pub distribution_id: u64,
    pub total_amount: i128,
    pub per_token_amount: i128,
    pub holder_count: u32,
    pub executed_at: u64,
    pub asset: Address, // always the registered BrlaContract — never caller-supplied
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenContract,
    KycContract,
    BrlaContract,      // SC-C03: registered BRLA asset — fixed at initialization
    DistributionCount,
    Distribution(u64),
}

// ─────────────────────────────────────────────────────────────────────────────
// Contract
// ─────────────────────────────────────────────────────────────────────────────

#[contract]
pub struct Distribution;

#[contractimpl]
impl Distribution {

    /// Initialize the distribution contract.
    /// `brla_contract` is the Transfero BRLA token address on Stellar — the only
    /// asset that may ever be distributed. Fixed at initialization (SC-C03).
    pub fn initialize(
        env: Env,
        admin: Address,
        token_contract: Address,
        kyc_contract: Address,
        brla_contract: Address,
    ) {
        // SC-C01: guard against re-initialization — any caller can overwrite admin
        // without this check because require_auth() only validates the supplied address.
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
        env.storage().instance().set(&DataKey::KycContract, &kyc_contract);
        env.storage().instance().set(&DataKey::BrlaContract, &brla_contract);
        env.storage().instance().set(&DataKey::DistributionCount, &0_u64);
        Self::bump_instance(&env);
    }

    /// Trigger a yield distribution to all token holders.
    /// Called by BWB backend (Convex) on quarterly schedule.
    ///
    /// SC-C02: scaffold guard — panics until Tranche 3 implementation is complete.
    /// This prevents false distribution events from being emitted on-chain before
    /// the actual BRLA transfer logic exists.
    ///
    /// SC-C03: the `asset` parameter was removed. The distribution asset is always
    /// `DataKey::BrlaContract` registered at initialization — never caller-supplied.
    #[allow(unused_variables)]
    pub fn trigger_distribution(env: Env, total_amount: i128) {
        panic!("trigger_distribution: not yet implemented — Tranche 3 deliverable");
        // ── Tranche 3 implementation outline (do not remove) ──────────────────
        // assert!(total_amount > 0, "Amount must be positive");
        //
        // let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        // admin.require_auth();
        //
        // // asset is always the registered BRLA — never from caller input (SC-C03)
        // let brla: Address = env.storage().instance().get(&DataKey::BrlaContract).unwrap();
        //
        // // 1. Enumerate KYC-approved holders from kyc_contract
        // // 2. Get each holder's balance from token_contract
        // // 3. Calculate per-token amount (total_amount * balance / total_supply)
        // // 4. Transfer BRLA from this contract to each holder
        // // 5. ONLY after all transfers succeed, emit event and increment counter:
        //
        // let dist_id: u64 = env.storage().instance()
        //     .get(&DataKey::DistributionCount)
        //     .unwrap_or(0);
        //
        // env.events().publish(
        //     (soroban_sdk::symbol_short!("dist"),),
        //     DistributionEvent {
        //         distribution_id: dist_id,
        //         total_amount,
        //         per_token_amount: 0, // fill from calculation
        //         holder_count: 0,     // fill from enumeration
        //         executed_at: env.ledger().timestamp(),
        //         asset: brla,
        //     }
        // );
        // env.storage().instance().set(&DataKey::DistributionCount, &(dist_id + 1));
        // Self::bump_instance(&env);
        // ──────────────────────────────────────────────────────────────────────
    }

    /// Get distribution event count.
    pub fn distribution_count(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::DistributionCount)
            .unwrap_or(0)
    }

    /// Return the current admin address, or None if not yet initialized.
    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    /// Return the registered BRLA contract address, or None if not yet initialized.
    pub fn get_brla_contract(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::BrlaContract)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    fn setup() -> (Env, DistributionClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, Distribution);
        let client = DistributionClient::new(&env, &contract_id);
        (env, client)
    }

    // ── Initialization ────────────────────────────────────────────────────────

    #[test]
    fn test_initialize() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let kyc = Address::generate(&env);
        let brla = Address::generate(&env);

        client.initialize(&admin, &token, &kyc, &brla);

        assert_eq!(client.get_admin(), Some(admin));
        assert_eq!(client.get_brla_contract(), Some(brla));
        assert_eq!(client.distribution_count(), 0);
    }

    // SC-C01: re-initialization guard
    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialize_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let kyc = Address::generate(&env);
        let brla = Address::generate(&env);

        client.initialize(&admin, &token, &kyc, &brla);
        client.initialize(&admin, &token, &kyc, &brla); // must panic
    }

    // SC-C02: scaffold guard prevents false distribution events
    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_trigger_distribution_scaffold_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let kyc = Address::generate(&env);
        let brla = Address::generate(&env);

        client.initialize(&admin, &token, &kyc, &brla);
        client.trigger_distribution(&1_000_000); // must panic until Tranche 3
    }
}
