#!/bin/bash
# Deploy BWB Soroban contracts to Stellar Testnet
set -e

NETWORK="testnet"
echo "Deploying BWB contracts to Stellar $NETWORK..."

# Fund deployer account via Friendbot (testnet only)
./scripts/fund-testnet-account.sh

# Deploy real-estate-token
echo "Deploying real-estate-token..."
RE_TOKEN_ADDRESS=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/real_estate_token.wasm \
  --network $NETWORK \
  --source "$DEPLOYER_SECRET_KEY")
echo "real-estate-token: $RE_TOKEN_ADDRESS"

# Deploy kyc-whitelist
echo "Deploying kyc-whitelist..."
KYC_ADDRESS=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kyc_whitelist.wasm \
  --network $NETWORK \
  --source "$DEPLOYER_SECRET_KEY")
echo "kyc-whitelist: $KYC_ADDRESS"

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
