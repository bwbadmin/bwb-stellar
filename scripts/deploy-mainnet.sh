#!/bin/bash
# Deploy BWB Soroban contracts to Stellar Mainnet
# WARNING: This deploys to production. Requires audit completion.
set -e

NETWORK="mainnet"

echo "=============================================="
echo "  BWB MAINNET DEPLOYMENT"
echo "  Network: Stellar Mainnet"
echo "  WARNING: This is a production deployment"
echo "=============================================="
echo ""

# Safety confirmation
read -p "Have you completed the security audit? (yes/no): " AUDIT_CONFIRMED
if [ "$AUDIT_CONFIRMED" != "yes" ]; then
  echo "Deployment cancelled. Complete security audit before mainnet deploy."
  exit 1
fi

read -p "Type 'DEPLOY MAINNET' to confirm: " CONFIRM
if [ "$CONFIRM" != "DEPLOY MAINNET" ]; then
  echo "Deployment cancelled."
  exit 1
fi

echo ""
echo "Deploying BWB contracts to Stellar Mainnet..."

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

# Deploy distribution
echo "Deploying distribution..."
DIST_ADDRESS=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/distribution.wasm \
  --network $NETWORK \
  --source "$DEPLOYER_SECRET_KEY")
echo "distribution: $DIST_ADDRESS"

# Save addresses
cat > deployed-addresses-mainnet.json <<EOF
{
  "network": "mainnet",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
    "real_estate_token": "$RE_TOKEN_ADDRESS",
    "kyc_whitelist": "$KYC_ADDRESS",
    "distribution": "$DIST_ADDRESS"
  },
  "explorer": {
    "real_estate_token": "https://stellar.expert/explorer/public/contract/$RE_TOKEN_ADDRESS",
    "kyc_whitelist": "https://stellar.expert/explorer/public/contract/$KYC_ADDRESS",
    "distribution": "https://stellar.expert/explorer/public/contract/$DIST_ADDRESS"
  }
}
EOF

echo ""
echo "MAINNET DEPLOYMENT COMPLETE"
echo "Addresses saved to deployed-addresses-mainnet.json"
