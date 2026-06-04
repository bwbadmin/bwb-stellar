// BWB KYC Whitelist — Soroban Smart Contract
// CVM Resolution 88 on-chain compliance gate
// License: Apache 2.0

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, symbol_short};

// TTL constants — Stellar mainnet at ~5 second ledger close time
// 1 day ≈ 17,280 ledgers
const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30;  // bump when < 30 days remain
const INSTANCE_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 60;         // extend to 60 days
const PERSISTENT_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30; // bump when < 30 days remain
const PERSISTENT_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 120;      // extend to 120 days (investor data is long-lived)

// ─────────────────────────────────────────────────────────────────────────────
// Data types
// ─────────────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum InvestorCategory {
    Retail,       // CVM 88: standard retail investor
    Qualified,    // CVM 88: R$1M+ financial assets
    Professional, // CVM 88: institutional / R$10M+
}

#[contracttype]
#[derive(Clone)]
pub struct WhitelistEntry {
    pub investor_category: InvestorCategory,
    pub approved_at: u64,    // ledger timestamp at approval
    pub approved_by: Address, // admin or operator that approved
}

#[contracttype]
pub enum DataKey {
    Admin,           // instance — cold wallet, governance only
    PendingAdmin,    // instance — two-step handover in progress
    Operator,        // instance — hot wallet, daily KYC operations
    Entry(Address),  // persistent — per-investor KYC record
}

// ─────────────────────────────────────────────────────────────────────────────
// Contract
// ─────────────────────────────────────────────────────────────────────────────

#[contract]
pub struct KYCWhitelist;

#[contractimpl]
impl KYCWhitelist {

    // ── Initialization ────────────────────────────────────────────────────────

    /// Deploy and set the admin (cold wallet).
    /// No operator is set at initialization — use set_operator() after deploy.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Self::bump_instance(&env);
    }

    // ── Admin governance ──────────────────────────────────────────────────────

    /// Step 1 of two-step admin handover: current admin proposes a successor.
    /// The new admin must call accept_admin() to complete the transfer.
    pub fn propose_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::PendingAdmin, &new_admin);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("adm_prop"),), new_admin);
    }

    /// Step 2 of two-step admin handover: pending admin accepts and becomes admin.
    /// Until this is called the previous admin remains in control.
    pub fn accept_admin(env: Env) {
        let pending: Address = env.storage().instance()
            .get(&DataKey::PendingAdmin)
            .expect("No pending admin proposal");
        pending.require_auth();
        env.storage().instance().set(&DataKey::Admin, &pending);
        env.storage().instance().remove(&DataKey::PendingAdmin);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("adm_new"),), pending);
    }

    /// Set or replace the operator (hot wallet for daily KYC operations). Admin only.
    pub fn set_operator(env: Env, operator: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Operator, &operator);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("op_set"),), operator);
    }

    /// Remove the operator. After this, only admin can perform KYC operations.
    pub fn remove_operator(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().remove(&DataKey::Operator);
        Self::bump_instance(&env);
    }

    // ── KYC operations (admin or operator) ───────────────────────────────────

    /// Add an investor to the KYC whitelist.
    /// `caller` must be either admin or operator, and must sign the transaction.
    pub fn add(env: Env, caller: Address, address: Address, category: InvestorCategory) {
        caller.require_auth();
        Self::require_admin_or_operator(&env, &caller);

        let entry = WhitelistEntry {
            investor_category: category,
            approved_at: env.ledger().timestamp(),
            approved_by: caller,
        };
        env.storage().persistent().set(&DataKey::Entry(address.clone()), &entry);
        env.storage().persistent().extend_ttl(
            &DataKey::Entry(address.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );

        Self::bump_instance(&env);
        env.events().publish((symbol_short!("kyc_add"),), address);
    }

    /// Remove an investor from the KYC whitelist.
    /// `caller` must be either admin or operator, and must sign the transaction.
    pub fn remove(env: Env, caller: Address, address: Address) {
        caller.require_auth();
        Self::require_admin_or_operator(&env, &caller);

        assert!(
            env.storage().persistent().has(&DataKey::Entry(address.clone())),
            "Address not in whitelist"
        );

        env.storage().persistent().remove(&DataKey::Entry(address.clone()));
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("kyc_rm"),), address);
    }

    // ── Read functions (no auth) ──────────────────────────────────────────────

    /// Check if an address is KYC-approved.
    /// Called by real-estate-token on every mint and transfer.
    pub fn is_ok(env: Env, address: Address) -> bool {
        env.storage().persistent().has(&DataKey::Entry(address))
    }

    /// Return the full whitelist entry for an address, or None if not approved.
    pub fn get_entry(env: Env, address: Address) -> Option<WhitelistEntry> {
        env.storage().persistent().get(&DataKey::Entry(address))
    }

    /// Return the current admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    /// Return the current operator address, or None if no operator is set.
    pub fn get_operator(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Operator)
    }

    // ── TTL management ────────────────────────────────────────────────────────

    /// Extend instance storage TTL.
    /// Call periodically (heartbeat) to keep contract governance data alive on-chain.
    pub fn extend_ttl(env: Env) {
        Self::bump_instance(&env);
    }

    /// Extend TTL for a specific investor entry.
    /// Call when an entry approaches its expiration threshold.
    pub fn extend_entry_ttl(env: Env, address: Address) {
        assert!(
            env.storage().persistent().has(&DataKey::Entry(address.clone())),
            "Address not in whitelist"
        );
        env.storage().persistent().extend_ttl(
            &DataKey::Entry(address),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    fn require_admin_or_operator(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        let operator: Option<Address> = env.storage().instance().get(&DataKey::Operator);

        let is_admin = *caller == admin;
        let is_operator = operator.map_or(false, |op| op == *caller);

        assert!(is_admin || is_operator, "Unauthorized: caller is not admin or operator");
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

    fn setup() -> (Env, KYCWhitelistClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, KYCWhitelist);
        let client = KYCWhitelistClient::new(&env, &contract_id);
        (env, client)
    }

    // ── Happy path ────────────────────────────────────────────────────────────

    #[test]
    fn test_add_and_verify() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        assert!(!client.is_ok(&investor));

        client.add(&admin, &investor, &InvestorCategory::Retail);
        assert!(client.is_ok(&investor));
    }

    #[test]
    fn test_remove() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&admin, &investor, &InvestorCategory::Qualified);
        assert!(client.is_ok(&investor));

        client.remove(&admin, &investor);
        assert!(!client.is_ok(&investor));
    }

    #[test]
    fn test_get_entry_returns_category_and_approver() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&admin, &investor, &InvestorCategory::Professional);

        let entry = client.get_entry(&investor).unwrap();
        assert_eq!(entry.investor_category, InvestorCategory::Professional);
        assert_eq!(entry.approved_by, admin);
    }

    // ── Operator ──────────────────────────────────────────────────────────────

    #[test]
    fn test_operator_can_add_and_remove() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let operator = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.set_operator(&operator);

        // operator can add
        client.add(&operator, &investor, &InvestorCategory::Retail);
        assert!(client.is_ok(&investor));

        // operator can remove
        client.remove(&operator, &investor);
        assert!(!client.is_ok(&investor));
    }

    #[test]
    fn test_set_operator_and_get_operator() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let operator = Address::generate(&env);

        client.initialize(&admin);
        assert!(client.get_operator().is_none());

        client.set_operator(&operator);
        assert_eq!(client.get_operator().unwrap(), operator);
    }

    #[test]
    fn test_remove_operator() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let operator = Address::generate(&env);

        client.initialize(&admin);
        client.set_operator(&operator);
        assert!(client.get_operator().is_some());

        client.remove_operator();
        assert!(client.get_operator().is_none());
    }

    // ── Two-step admin handover ───────────────────────────────────────────────

    #[test]
    fn test_propose_and_accept_admin() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let new_admin = Address::generate(&env);

        client.initialize(&admin);
        assert_eq!(client.get_admin(), admin);

        client.propose_admin(&new_admin);
        // admin is still the current admin — handover not complete
        assert_eq!(client.get_admin(), admin);

        client.accept_admin();
        assert_eq!(client.get_admin(), new_admin);
    }

    #[test]
    fn test_new_admin_can_operate_after_handover() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let new_admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.propose_admin(&new_admin);
        client.accept_admin();

        // new admin can now approve investors
        client.add(&new_admin, &investor, &InvestorCategory::Retail);
        assert!(client.is_ok(&investor));
    }

    // ── TTL ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_extend_ttl_does_not_panic() {
        let (env, client) = setup();
        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.extend_ttl(); // should not panic
    }

    #[test]
    fn test_extend_entry_ttl() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&admin, &investor, &InvestorCategory::Retail);
        client.extend_entry_ttl(&investor); // should not panic
    }

    // ── Error cases ───────────────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialize_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin); // must panic
    }

    #[test]
    #[should_panic(expected = "Unauthorized: caller is not admin or operator")]
    fn test_random_address_cannot_add() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let attacker = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&attacker, &investor, &InvestorCategory::Retail); // must panic
    }

    #[test]
    #[should_panic(expected = "Unauthorized: caller is not admin or operator")]
    fn test_random_address_cannot_remove() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let attacker = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.add(&admin, &investor, &InvestorCategory::Retail);
        client.remove(&attacker, &investor); // must panic
    }

    #[test]
    #[should_panic(expected = "Address not in whitelist")]
    fn test_remove_nonexistent_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.remove(&admin, &investor); // must panic — never added
    }

    #[test]
    #[should_panic(expected = "No pending admin proposal")]
    fn test_accept_admin_without_proposal_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.accept_admin(); // must panic — no proposal pending
    }

    #[test]
    #[should_panic(expected = "Address not in whitelist")]
    fn test_extend_entry_ttl_nonexistent_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let investor = Address::generate(&env);

        client.initialize(&admin);
        client.extend_entry_ttl(&investor); // must panic — not in whitelist
    }
}
