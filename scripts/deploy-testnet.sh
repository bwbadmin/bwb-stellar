#!/bin/bash
# Deploy BWB Soroban contracts to Stellar Testnet
# SC-X02: each contract is deployed and initialized in a single operation
# (deploy ... -- initialize ...) to eliminate the front-run window between
# deploy and initialize.
set -euo pipefail

NETWORK="testnet"

# SEC-A3: use a named keystore identity — never expose the seed as an argv token.
# Import once: printf '%s\n' "$SECRET_SEED" | soroban keys add deployer --secret-key
: "${DEPLOYER_IDENTITY:?DEPLOYER_IDENTITY must be set (name of the soroban keys entry, e.g. 'deployer')}"
: "${ADMIN_G:?ADMIN_G (admin Stellar G-address) must be set}"
: "${OPERATOR_G:?OPERATOR_G (operator Stellar G-address) must be set}"
: "${TOKEN_NAME:?TOKEN_NAME must be set}"
: "${TOKEN_SYMBOL:?TOKEN_SYMBOL must be set}"
: "${METADATA_JSON:?METADATA_JSON (XDR or JSON blob) must be set}"

echo "Deploying BWB contracts to Stellar $NETWORK..."

# Fund deployer account via Friendbot (testnet only)
./scripts/fund-testnet-account.sh

# 1. Deploy KYC whitelist — atomic deploy+initialize (SC-X02)
echo "Deploying kyc-whitelist..."
KYC_ADDRESS=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kyc_whitelist.wasm \
  --network "$NETWORK" \
  --source "$DEPLOYER_IDENTITY" \
  -- initialize \
  --admin "$ADMIN_G")
echo "kyc-whitelist: $KYC_ADDRESS"

# 2. Real-estate-token — atomic deploy+initialize (SEC-A1 / SC-X02)
echo "Deploying real-estate-token..."
RE_TOKEN_ADDRESS=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/real_estate_token.wasm \
  --network "$NETWORK" \
  --source "$DEPLOYER_IDENTITY" \
  -- initialize \
  --admin "$ADMIN_G" \
  --operator "$OPERATOR_G" \
  --kyc_contract "$KYC_ADDRESS" \
  --name "$TOKEN_NAME" \
  --symbol "$TOKEN_SYMBOL" \
  --metadata "$METADATA_JSON")
echo "real-estate-token: $RE_TOKEN_ADDRESS"

# Save addresses to file for CI artifact
cat > deployed-addresses-testnet.json <<EOF
{
  "network": "testnet",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
    "real_estate_token": "$RE_TOKEN_ADDRESS",
    "kyc_whitelist": "$KYC_ADDRESS"
  },
  "explorer": {
    "real_estate_token": "https://testnet.stellar.expert/explorer/testnet/contract/$RE_TOKEN_ADDRESS",
    "kyc_whitelist": "https://testnet.stellar.expert/explorer/testnet/contract/$KYC_ADDRESS"
  }
}
EOF

echo ""
echo "Deployment complete!"
echo "Addresses saved to deployed-addresses-testnet.json"
echo ""
echo "View on Stellar Expert:"
echo "  real-estate-token: https://testnet.stellar.expert/explorer/testnet/contract/$RE_TOKEN_ADDRESS"
echo "  kyc-whitelist:     https://testnet.stellar.expert/explorer/testnet/contract/$KYC_ADDRESS"
