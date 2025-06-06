#!/bin/bash

openssl ecparam -name secp384r1 -genkey -noout | \
openssl pkcs8 -topk8 -nocrypt -out p384-private-key.pem

echo "Generated p384-private-key.pem"

openssl ecparam -name secp521r1 -genkey -noout | \
openssl pkcs8 -topk8 -nocrypt -out p521-private-key.pem

echo "Generated p521-private-key.pem"
