#!/bin/bash
# Fund a Stellar testnet account via Friendbot
set -e

NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
FRIENDBOT_URL="https://friendbot.stellar.org"

if [ -z "$STELLAR_TESTNET_PUBLIC_KEY" ]; then
  echo "Error: STELLAR_TESTNET_PUBLIC_KEY environment variable not set"
  exit 1
fi

echo "Funding testnet account: $STELLAR_TESTNET_PUBLIC_KEY"
curl -s "$FRIENDBOT_URL?addr=$STELLAR_TESTNET_PUBLIC_KEY" | python3 -m json.tool
echo "Account funded successfully."
