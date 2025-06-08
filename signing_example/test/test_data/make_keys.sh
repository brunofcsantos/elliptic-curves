#!/bin/bash

# Ensure the script is sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script must be sourced: use 'source ${0}' or '. ${0}'"
  return 1
fi

# Change directory to the directory containing this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || { echo "Failed to cd into script directory"; return 1; }

# Generate keys
openssl ecparam -name secp256r1 -genkey -noout | \
openssl pkcs8 -topk8 -nocrypt -out p256-private-key.pem

openssl ecparam -name secp384r1 -genkey -noout | \
openssl pkcs8 -topk8 -nocrypt -out p384-private-key.pem

echo "Generated p384-private-key.pem"

openssl ecparam -name secp521r1 -genkey -noout | \
openssl pkcs8 -topk8 -nocrypt -out p521-private-key.pem

echo "Generated p521-private-key.pem"
