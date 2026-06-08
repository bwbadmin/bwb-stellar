// BWB Real Estate Token — Soroban Smart Contract
// CVM Resolution 88 compliant RWA token on Stellar/Soroban
// Implements SEP-0041 (Soroban token standard)
// License: Apache 2.0

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, String, symbol_short};

// TTL constants — Stellar mainnet at ~5 second ledger close time
const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30;
const INSTANCE_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 60;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30;
const PERSISTENT_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 120;

// Token precision: 7 decimal places (matches Stellar native asset standard)
pub const DECIMALS: u32 = 7;

// ─────────────────────────────────────────────────────────────────────────────
// Data types
// ─────────────────────────────────────────────────────────────────────────────

/// CVM Resolution 88 real estate offering metadata — stored immutably on-chain.
#[contracttype]
#[derive(Clone)]
pub struct OfferingMetadata {
    pub offering_id: String,        // BWB internal ID (e.g. "ARTP-HS")
    pub property_address: String,   // Brazilian property address
    pub total_raise: i128,          // Total raise in BRL cents
    pub max_supply: i128,           // SC-H05: max token units authorized by CVM-88
    pub target_irr_bps: u32,        // Target IRR in basis points (2080 = 20.80%)
    pub maturity_date: u64,         // Unix timestamp of expected maturity
    pub cvm_authorization: String,  // CVM Resolution 88 authorization code
}

/// Key for allowance temporary storage.
#[contracttype]
#[derive(Clone)]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

/// Allowance value with expiration.
#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    // Instance storage — governance and config
    Admin,
    PendingAdmin,
    Operator,
    KycContract,
    TotalSupply,
    Name,
    Symbol,
    Metadata,
    Paused,
    // Persistent storage — investor balances
    Balance(Address),
    // Temporary storage — allowances
    Allowance(AllowanceKey),
}

// ─────────────────────────────────────────────────────────────────────────────
// Contract
// ─────────────────────────────────────────────────────────────────────────────

#[contract]
pub struct RealEstateToken;

#[contractimpl]
impl RealEstateToken {

    // ── Initialization ────────────────────────────────────────────────────────

    /// Deploy the token contract.
    /// `name` and `symbol` are per-offering (e.g. "ARTP-HS Token", "ARTP-HS").
    /// `kyc_contract` is the deployed kyc-whitelist contract address.
    pub fn initialize(
        env: Env,
        admin: Address,
        operator: Address,
        kyc_contract: Address,
        name: String,
        symbol: String,
        metadata: OfferingMetadata,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Operator, &operator);
        env.storage().instance().set(&DataKey::KycContract, &kyc_contract);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Metadata, &metadata);
        env.storage().instance().set(&DataKey::Paused, &false);

        Self::bump_instance(&env);
    }

    // ── Admin governance ──────────────────────────────────────────────────────

    /// Step 1 of two-step admin handover.
    pub fn propose_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::PendingAdmin, &new_admin);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("adm_prop"),), new_admin); // SC-L01
    }

    /// Step 2 of two-step admin handover: pending admin accepts.
    pub fn accept_admin(env: Env) {
        let pending: Address = env.storage().instance()
            .get(&DataKey::PendingAdmin)
            .expect("No pending admin proposal");
        pending.require_auth();
        env.storage().instance().set(&DataKey::Admin, &pending);
        env.storage().instance().remove(&DataKey::PendingAdmin);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("adm_new"),), pending); // SC-L01
    }

    /// Set the operator (hot wallet for minting). Admin only.
    pub fn set_operator(env: Env, operator: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Operator, &operator);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("op_set"),), operator); // SC-L01
    }

    /// Update the KYC contract address. Admin only.
    /// SC-H04: allows compliance logic upgrades without redeploying the token contract.
    /// CAUTION: ensure the new contract is fully initialized with all existing investors
    /// before calling — any transfer or mint will immediately use the new KYC gate.
    pub fn set_kyc_contract(env: Env, new_kyc_contract: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        // SC-X04: probe the new contract implements is_ok before committing;
        // traps on wrong interface (wrong ABI / uninitialized contract).
        let _probe: bool = env.invoke_contract(
            &new_kyc_contract,
            &symbol_short!("is_ok"),
            soroban_sdk::vec![&env, admin.clone().into_val(&env)],
        );
        let old: Address = env.storage().instance().get(&DataKey::KycContract).unwrap();
        env.storage().instance().set(&DataKey::KycContract, &new_kyc_contract);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("kyc_upd"),), (old, new_kyc_contract));
    }

    /// Pause all transfers and burns. Admin only.
    pub fn pause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &true);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("paused"),), ());
    }

    /// Unpause transfers and burns. Admin only.
    pub fn unpause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &false);
        Self::bump_instance(&env);
        env.events().publish((symbol_short!("unpaused"),), ());
    }

    // ── Minting (admin or operator) ───────────────────────────────────────────

    /// Mint tokens to a KYC-approved investor. Caller must be admin or operator.
    /// KYC gate: recipient must be in the kyc-whitelist contract (CVM 88).
    pub fn mint(env: Env, caller: Address, to: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        Self::require_not_paused(&env); // SC-X03: pause must block minting too
        caller.require_auth();
        Self::require_admin_or_operator(&env, &caller);
        Self::require_kyc_approved(&env, &to);

        // SC-H05: enforce the CVM-88 authorized offering cap stored in metadata.
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        let metadata: OfferingMetadata = env.storage().instance().get(&DataKey::Metadata).unwrap();
        assert!(
            supply + amount <= metadata.max_supply,
            "Mint exceeds CVM-88 authorized offering cap"
        );

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        env.storage().persistent()
            .set(&DataKey::Balance(to.clone()), &(balance + amount));
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );

        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));

        Self::bump_instance(&env);
        env.events().publish((symbol_short!("mint"),), (to, amount));
    }

    // ── SEP-0041 — Core token interface ──────────────────────────────────────

    /// Return token balance for an address.
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    /// Transfer tokens from `from` to `to`.
    /// Requires auth from `from`. Both parties must be KYC-approved (CVM 88).
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        Self::require_not_paused(&env);
        from.require_auth();
        Self::require_kyc_approved(&env, &from); // SC-H01: sender KYC verified
        Self::require_kyc_approved(&env, &to);

        Self::do_transfer(&env, &from, &to, amount);
        env.events().publish((symbol_short!("transfer"),), (from, to, amount));
    }

    /// Transfer tokens on behalf of `from` using a pre-approved allowance.
    /// Spender must have sufficient allowance. Both `from` and `to` must be KYC-approved.
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        Self::require_not_paused(&env);
        spender.require_auth();
        Self::require_kyc_approved(&env, &from); // SC-H01: sender KYC verified
        Self::require_kyc_approved(&env, &to);

        Self::spend_allowance(&env, &from, &spender, amount);
        Self::do_transfer(&env, &from, &to, amount);
        env.events().publish((symbol_short!("xfer_from"),), (spender, from, to, amount));
    }

    /// Approve `spender` to transfer up to `amount` tokens from `from`.
    /// `expiration_ledger` is the absolute ledger number when the allowance expires.
    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        from.require_auth();
        Self::require_not_paused(&env); // SC-X07: freeze must also block new approvals

        assert!(
            expiration_ledger > env.ledger().sequence(), // SC-H03: strictly future (>= allows TTL=0)
            "Expiration ledger must be in the future"
        );
        assert!(amount >= 0, "Amount must be non-negative");

        let key = AllowanceKey { from: from.clone(), spender: spender.clone() };
        let val = AllowanceValue { amount, expiration_ledger };

        if amount == 0 {
            env.storage().temporary().remove(&DataKey::Allowance(key));
        } else {
            let ttl = expiration_ledger - env.ledger().sequence();
            env.storage().temporary().set(&DataKey::Allowance(key.clone()), &val);
            env.storage().temporary().extend_ttl(&DataKey::Allowance(key), ttl, ttl);
        }

        Self::bump_instance(&env);
        env.events().publish((symbol_short!("approve"),), (from, spender, amount, expiration_ledger));
    }

    /// Return the remaining allowance for `spender` to spend on behalf of `from`.
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = AllowanceKey { from, spender };
        match env.storage().temporary().get::<DataKey, AllowanceValue>(&DataKey::Allowance(key)) {
            Some(val) if val.expiration_ledger >= env.ledger().sequence() => val.amount,
            _ => 0,
        }
    }

    /// Burn (destroy) tokens from `from`. Requires auth from `from`.
    /// Paused state blocks burns. No KYC check — token holders can always exit.
    pub fn burn(env: Env, from: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        Self::require_not_paused(&env);
        from.require_auth();

        Self::do_burn(&env, &from, amount);
        env.events().publish((symbol_short!("burn"),), (from, amount));
    }

    /// Burn tokens from `from` using spender's allowance.
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        Self::require_not_paused(&env);
        spender.require_auth();

        Self::spend_allowance(&env, &from, &spender, amount);
        Self::do_burn(&env, &from, amount);
        env.events().publish((symbol_short!("burn_from"),), (spender, from, amount));
    }

    // ── SEP-0041 — Metadata ───────────────────────────────────────────────────

    /// Token decimal places (7, matching Stellar native asset standard).
    pub fn decimals(_env: Env) -> u32 {
        DECIMALS
    }

    /// Token name (e.g. "ARTP-HS Token").
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    /// Token symbol (e.g. "ARTP-HS").
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    // ── BWB-specific reads ────────────────────────────────────────────────────

    /// Total issued supply (in smallest token units).
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    /// CVM 88 offering metadata (property, authorization code, target IRR).
    pub fn get_offering(env: Env) -> OfferingMetadata {
        env.storage().instance().get(&DataKey::Metadata).unwrap()
    }

    /// Current admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    /// Current operator address.
    pub fn get_operator(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Operator).unwrap()
    }

    /// Paused state.
    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }

    /// NAV per whole token in BRL cents.
    /// Formula: total_raise_cents * 10^DECIMALS / total_supply
    /// Returns 0 if no tokens have been issued yet.
    pub fn nav(env: Env) -> i128 {
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        if supply == 0 {
            return 0;
        }
        let metadata: OfferingMetadata = env.storage().instance().get(&DataKey::Metadata).unwrap();
        let scalar: i128 = 10_i128.pow(DECIMALS);
        metadata.total_raise * scalar / supply
    }

    // ── TTL management ────────────────────────────────────────────────────────

    /// Extend instance storage TTL (heartbeat).
    pub fn extend_ttl(env: Env) {
        Self::bump_instance(&env);
    }

    /// Extend TTL for a specific investor balance entry.
    pub fn extend_balance_ttl(env: Env, investor: Address) {
        assert!(
            env.storage().persistent().has(&DataKey::Balance(investor.clone())),
            "No balance for address"
        );
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(investor),
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
        let operator: Address = env.storage().instance().get(&DataKey::Operator).unwrap();
        assert!(
            *caller == admin || *caller == operator,
            "Unauthorized: caller is not admin or operator"
        );
    }

    fn require_not_paused(env: &Env) {
        let paused: bool = env.storage().instance().get(&DataKey::Paused).unwrap_or(false);
        assert!(!paused, "Contract is paused");
    }

    fn require_kyc_approved(env: &Env, address: &Address) {
        let kyc_contract: Address = env.storage().instance()
            .get(&DataKey::KycContract)
            .unwrap();
        let approved: bool = env.invoke_contract(
            &kyc_contract,
            &symbol_short!("is_ok"),
            soroban_sdk::vec![env, address.clone().into_val(env)],
        );
        assert!(approved, "Transfer rejected: address not KYC-approved (CVM 88)");
    }

    fn do_transfer(env: &Env, from: &Address, to: &Address, amount: i128) {
        // SC-X01: self-transfer would read B, write (B-amount), then overwrite with (B+amount).
        if from == to {
            return;
        }
        let from_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        let to_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        let new_from_balance = from_balance - amount;
        env.storage().persistent()
            .set(&DataKey::Balance(from.clone()), &new_from_balance);
        env.storage().persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Extend TTL for recipient balance
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );

        // SC-H02: extend TTL for sender's remaining balance — without this the
        // sender's entry can expire and the remaining tokens are silently lost.
        if new_from_balance > 0 {
            env.storage().persistent().extend_ttl(
                &DataKey::Balance(from.clone()),
                PERSISTENT_LIFETIME_THRESHOLD,
                PERSISTENT_BUMP_AMOUNT,
            );
        }
    }

    fn do_burn(env: &Env, from: &Address, amount: i128) {
        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance");

        env.storage().persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply - amount));

        Self::bump_instance(env);
    }

    fn spend_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) {
        let key = AllowanceKey { from: from.clone(), spender: spender.clone() };
        let val: AllowanceValue = env.storage().temporary()
            .get(&DataKey::Allowance(key.clone()))
            .expect("No allowance");

        assert!(
            val.expiration_ledger >= env.ledger().sequence(),
            "Allowance expired"
        );
        assert!(val.amount >= amount, "Insufficient allowance");

        let new_amount = val.amount - amount;
        if new_amount == 0 {
            env.storage().temporary().remove(&DataKey::Allowance(key));
        } else {
            let new_val = AllowanceValue {
                amount: new_amount,
                expiration_ledger: val.expiration_ledger,
            };
            env.storage().temporary().set(&DataKey::Allowance(key), &new_val);
        }
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
    use kyc_whitelist::{KYCWhitelist, KYCWhitelistClient, InvestorCategory};

    // ── Test helpers ──────────────────────────────────────────────────────────

    fn make_metadata(env: &Env) -> OfferingMetadata {
        OfferingMetadata {
            offering_id: String::from_str(env, "ARTP-HS"),
            property_address: String::from_str(env, "Rua das Flores, 100, Jaraguá do Sul, SC"),
            total_raise: 250_000_000,  // R$ 2,500,000.00 in cents
            max_supply: 10_000_000_000, // SC-H05: 1 000 whole tokens at 7 decimals
            target_irr_bps: 2680,      // 26.80%
            maturity_date: 1_800_000_000,
            cvm_authorization: String::from_str(env, "CVM-88-2024-001"),
        }
    }

    /// Sets up both contracts. Returns (env, kyc_client, token_client, admin, operator).
    fn setup() -> (
        Env,
        KYCWhitelistClient<'static>,
        RealEstateTokenClient<'static>,
        Address,
        Address,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        // Register kyc-whitelist
        let kyc_id = env.register_contract(None, KYCWhitelist);
        let kyc_client = KYCWhitelistClient::new(&env, &kyc_id);

        // Register real-estate-token
        let token_id = env.register_contract(None, RealEstateToken);
        let token_client = RealEstateTokenClient::new(&env, &token_id);

        let admin = Address::generate(&env);
        let operator = Address::generate(&env);
        let metadata = make_metadata(&env);

        kyc_client.initialize(&admin);
        token_client.initialize(
            &admin,
            &operator,
            &kyc_id,
            &String::from_str(&env, "ARTP-HS Token"),
            &String::from_str(&env, "ARTP-HS"),
            &metadata,
        );

        (env, kyc_client, token_client, admin, operator)
    }

    // ── Initialization ────────────────────────────────────────────────────────

    #[test]
    fn test_initialize() {
        let (env, _kyc, token, admin, operator) = setup();
        assert_eq!(token.get_admin(), admin);
        assert_eq!(token.get_operator(), operator);
        assert_eq!(token.total_supply(), 0);
        assert_eq!(token.decimals(), 7);
        assert_eq!(token.name(), String::from_str(&env, "ARTP-HS Token"));
        assert_eq!(token.symbol(), String::from_str(&env, "ARTP-HS"));
        assert!(!token.is_paused());
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialize_panics() {
        let (env, kyc_client, token, admin, operator) = setup();
        let kyc_id = kyc_client.address.clone();
        let metadata = make_metadata(&env);
        token.initialize(
            &admin, &operator, &kyc_id,
            &String::from_str(&env, "X"), &String::from_str(&env, "X"),
            &metadata,
        );
    }

    // ── Mint ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_mint_to_kyc_approved_investor() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000); // 100 tokens (7 decimals)

        assert_eq!(token.balance(&investor), 1_000_000_000);
        assert_eq!(token.total_supply(), 1_000_000_000);
    }

    #[test]
    fn test_operator_can_mint() {
        let (env, kyc, token, admin, operator) = setup();
        let investor = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Qualified);
        token.mint(&operator, &investor, &500_000_000);

        assert_eq!(token.balance(&investor), 500_000_000);
    }

    #[test]
    #[should_panic(expected = "Transfer rejected: address not KYC-approved (CVM 88)")]
    fn test_mint_to_non_kyc_address_panics() {
        let (env, _kyc, token, admin, _op) = setup();
        let stranger = Address::generate(&env);
        token.mint(&admin, &stranger, &1_000_000_000);
    }

    #[test]
    #[should_panic(expected = "Unauthorized: caller is not admin or operator")]
    fn test_random_cannot_mint() {
        let (env, kyc, token, admin, _op) = setup();
        let attacker = Address::generate(&env);
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&attacker, &investor, &1_000_000_000);
    }

    #[test]
    #[should_panic(expected = "Amount must be positive")]
    fn test_mint_zero_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &0);
    }

    // ── Transfer ──────────────────────────────────────────────────────────────

    #[test]
    fn test_transfer_between_kyc_investors() {
        let (env, kyc, token, admin, _op) = setup();
        let investor_a = Address::generate(&env);
        let investor_b = Address::generate(&env);

        kyc.add(&admin, &investor_a, &InvestorCategory::Retail);
        kyc.add(&admin, &investor_b, &InvestorCategory::Retail);

        token.mint(&admin, &investor_a, &1_000_000_000);
        token.transfer(&investor_a, &investor_b, &300_000_000);

        assert_eq!(token.balance(&investor_a), 700_000_000);
        assert_eq!(token.balance(&investor_b), 300_000_000);
    }

    #[test]
    #[should_panic(expected = "Transfer rejected: address not KYC-approved (CVM 88)")]
    fn test_transfer_to_non_kyc_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        let stranger = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000);
        token.transfer(&investor, &stranger, &100_000_000);
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_transfer_exceeds_balance_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor_a = Address::generate(&env);
        let investor_b = Address::generate(&env);

        kyc.add(&admin, &investor_a, &InvestorCategory::Retail);
        kyc.add(&admin, &investor_b, &InvestorCategory::Retail);
        token.mint(&admin, &investor_a, &100_000_000);
        token.transfer(&investor_a, &investor_b, &200_000_000);
    }

    // ── Approve + transfer_from ───────────────────────────────────────────────

    #[test]
    fn test_approve_and_transfer_from() {
        let (env, kyc, token, admin, _op) = setup();
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);

        kyc.add(&admin, &owner, &InvestorCategory::Retail);
        kyc.add(&admin, &recipient, &InvestorCategory::Retail);
        token.mint(&admin, &owner, &1_000_000_000);

        let expiry = env.ledger().sequence() + 1000;
        token.approve(&owner, &spender, &500_000_000, &expiry);
        assert_eq!(token.allowance(&owner, &spender), 500_000_000);

        token.transfer_from(&spender, &owner, &recipient, &200_000_000);
        assert_eq!(token.balance(&owner), 800_000_000);
        assert_eq!(token.balance(&recipient), 200_000_000);
        assert_eq!(token.allowance(&owner, &spender), 300_000_000);
    }

    #[test]
    #[should_panic(expected = "Insufficient allowance")]
    fn test_transfer_from_exceeds_allowance_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);

        kyc.add(&admin, &owner, &InvestorCategory::Retail);
        kyc.add(&admin, &recipient, &InvestorCategory::Retail);
        token.mint(&admin, &owner, &1_000_000_000);

        let expiry = env.ledger().sequence() + 1000;
        token.approve(&owner, &spender, &100_000_000, &expiry);
        token.transfer_from(&spender, &owner, &recipient, &200_000_000);
    }

    // ── Burn ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_burn_reduces_supply() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000);
        token.burn(&investor, &400_000_000);

        assert_eq!(token.balance(&investor), 600_000_000);
        assert_eq!(token.total_supply(), 600_000_000);
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_burn_exceeds_balance_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &100_000_000);
        token.burn(&investor, &200_000_000);
    }

    // ── Pause ─────────────────────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "Contract is paused")]
    fn test_transfer_when_paused_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor_a = Address::generate(&env);
        let investor_b = Address::generate(&env);

        kyc.add(&admin, &investor_a, &InvestorCategory::Retail);
        kyc.add(&admin, &investor_b, &InvestorCategory::Retail);
        token.mint(&admin, &investor_a, &1_000_000_000);
        token.pause();
        token.transfer(&investor_a, &investor_b, &100_000_000);
    }

    #[test]
    fn test_unpause_restores_transfers() {
        let (env, kyc, token, admin, _op) = setup();
        let investor_a = Address::generate(&env);
        let investor_b = Address::generate(&env);

        kyc.add(&admin, &investor_a, &InvestorCategory::Retail);
        kyc.add(&admin, &investor_b, &InvestorCategory::Retail);
        token.mint(&admin, &investor_a, &1_000_000_000);
        token.pause();
        token.unpause();
        token.transfer(&investor_a, &investor_b, &100_000_000); // should succeed
        assert_eq!(token.balance(&investor_b), 100_000_000);
    }

    // ── NAV ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_nav_before_mint_is_zero() {
        let (_env, _kyc, token, _admin, _op) = setup();
        assert_eq!(token.nav(), 0);
    }

    #[test]
    fn test_nav_after_mint() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);

        // Mint 100 whole tokens = 100 * 10^7 = 1_000_000_000 units
        // total_raise = 250_000_000 cents = R$ 2,500,000
        // NAV = 250_000_000 * 10^7 / 1_000_000_000 = 2_500_000_000 (R$ 25,000 per token in cents*10^7 scale)
        token.mint(&admin, &investor, &1_000_000_000);
        assert!(token.nav() > 0);
    }

    // ── Admin handover ────────────────────────────────────────────────────────

    #[test]
    fn test_propose_and_accept_admin() {
        let (env, _kyc, token, admin, _op) = setup();
        let new_admin = Address::generate(&env);

        token.propose_admin(&new_admin);
        assert_eq!(token.get_admin(), admin); // not changed yet

        token.accept_admin();
        assert_eq!(token.get_admin(), new_admin);
    }

    // ── TTL ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_extend_ttl_does_not_panic() {
        let (_env, _kyc, token, _admin, _op) = setup();
        token.extend_ttl();
    }

    #[test]
    fn test_extend_balance_ttl() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000);
        token.extend_balance_ttl(&investor);
    }

    #[test]
    #[should_panic(expected = "No balance for address")]
    fn test_extend_balance_ttl_no_balance_panics() {
        let (env, _kyc, token, _admin, _op) = setup();
        let stranger = Address::generate(&env);
        token.extend_balance_ttl(&stranger);
    }

    // ── SC-H01: sender KYC check ──────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "Transfer rejected: address not KYC-approved (CVM 88)")]
    fn test_transfer_from_revoked_sender_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor_a = Address::generate(&env);
        let investor_b = Address::generate(&env);

        kyc.add(&admin, &investor_a, &InvestorCategory::Retail);
        kyc.add(&admin, &investor_b, &InvestorCategory::Retail);
        token.mint(&admin, &investor_a, &1_000_000_000);

        kyc.remove(&admin, &investor_a); // revoke A's KYC
        token.transfer(&investor_a, &investor_b, &100_000_000); // must panic
    }

    #[test]
    #[should_panic(expected = "Transfer rejected: address not KYC-approved (CVM 88)")]
    fn test_transfer_from_revoked_sender_via_allowance_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);

        kyc.add(&admin, &owner, &InvestorCategory::Retail);
        kyc.add(&admin, &recipient, &InvestorCategory::Retail);
        token.mint(&admin, &owner, &1_000_000_000);

        let expiry = env.ledger().sequence() + 1000;
        token.approve(&owner, &spender, &500_000_000, &expiry);

        kyc.remove(&admin, &owner); // revoke owner's KYC after approval
        token.transfer_from(&spender, &owner, &recipient, &200_000_000); // must panic
    }

    // ── SC-H03: approve expiration ledger strictly future ─────────────────────

    #[test]
    #[should_panic(expected = "Expiration ledger must be in the future")]
    fn test_approve_current_ledger_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);

        kyc.add(&admin, &owner, &InvestorCategory::Retail);
        token.mint(&admin, &owner, &1_000_000_000);

        let current = env.ledger().sequence();
        token.approve(&owner, &spender, &100_000_000, &current); // must panic
    }

    // ── SC-H04: KYC contract update ───────────────────────────────────────────

    #[test]
    fn test_set_kyc_contract() {
        let (env, _kyc, token, admin, _op) = setup();

        // Deploy a fresh KYC contract and point the token to it
        let new_kyc_id = env.register_contract(None, KYCWhitelist);
        let new_kyc_client = KYCWhitelistClient::new(&env, &new_kyc_id);
        new_kyc_client.initialize(&admin);

        token.set_kyc_contract(&new_kyc_id); // admin can update KYC contract

        // Minting to an investor only approved in the NEW contract must succeed
        let investor = Address::generate(&env);
        new_kyc_client.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000);
        assert_eq!(token.balance(&investor), 1_000_000_000);
    }

    // ── SC-X01: self-transfer must be a no-op ────────────────────────────────

    #[test]
    fn test_self_transfer_is_noop() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &1_000_000_000);
        token.transfer(&investor, &investor, &500_000_000); // must not inflate balance

        assert_eq!(token.balance(&investor), 1_000_000_000);
        assert_eq!(token.total_supply(), 1_000_000_000);
    }

    // ── SC-X03: pause must block minting ─────────────────────────────────────

    #[test]
    #[should_panic(expected = "Contract is paused")]
    fn test_mint_when_paused_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);

        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.pause();
        token.mint(&admin, &investor, &1_000_000_000);
    }

    // ── SC-X07: pause must block approve ─────────────────────────────────────

    #[test]
    #[should_panic(expected = "Contract is paused")]
    fn test_approve_when_paused_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);

        kyc.add(&admin, &owner, &InvestorCategory::Retail);
        token.mint(&admin, &owner, &1_000_000_000);
        token.pause();
        let expiry = env.ledger().sequence() + 1000;
        token.approve(&owner, &spender, &500_000_000, &expiry);
    }

    // ── SC-H05: CVM-88 authorized offering cap ────────────────────────────────

    #[test]
    #[should_panic(expected = "Mint exceeds CVM-88 authorized offering cap")]
    fn test_mint_exceeds_cap_panics() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        // max_supply = 10_000_000_000; try to mint one unit beyond the cap
        token.mint(&admin, &investor, &10_000_000_001);
    }

    #[test]
    fn test_mint_exactly_at_cap_succeeds() {
        let (env, kyc, token, admin, _op) = setup();
        let investor = Address::generate(&env);
        kyc.add(&admin, &investor, &InvestorCategory::Retail);
        token.mint(&admin, &investor, &10_000_000_000); // exactly at cap — must succeed
        assert_eq!(token.total_supply(), 10_000_000_000);
    }
}
