#!/bin/bash
# IMA/EVM Key Generation for Ubuntu (No Passphrase)
set -e

KEY_DIR="/etc/keys"
sudo mkdir -p "$KEY_DIR"
sudo chmod 700 "$KEY_DIR"

echo "[1/3] Generating Master Keys (KMK & EVM-Key)..."
# Create a 32-byte random hex string for the Kernel Master Key
sudo dd if=/dev/urandom bs=1 count=32 2>/dev/null | xxd -p -c 32 | sudo tee "$KEY_DIR/kmk" > /dev/null
# Create the encrypted EVM key
sudo dd if=/dev/urandom bs=1 count=32 2>/dev/null | xxd -p -c 32 | sudo tee "$KEY_DIR/evm-key" > /dev/null

echo "[2/3] Generating RSA Keypair (Private & Public X509)..."
# Generate private key with NO password (-nodes)
sudo openssl genrsa -out "$KEY_DIR/privkey_evm.pem" 2048

# Generate the X509 certificate in DER format for the kernel
sudo openssl req -new -nodes -utf8 -sha256 -days 3650 \
    -batch -x509 \
    -key "$KEY_DIR/privkey_evm.pem" \
    -out "$KEY_DIR/x509_evm.der" \
    -outform DER \
    -subj "/CN=IMA-EVM Root CA/"

# Copy for scripts that specifically look for the 'ima' filename
sudo cp "$KEY_DIR/x509_evm.der" "$KEY_DIR/x509_ima.der"

echo "[3/3] Securing Key Directory..."
sudo chmod 400 "$KEY_DIR/kmk" "$KEY_DIR/evm-key" "$KEY_DIR/privkey_evm.pem"
sudo chmod 444 "$KEY_DIR/x509_evm.der" "$KEY_DIR/x509_ima.der"

echo "--------------------------------------------------"
echo "Keys created in $KEY_DIR"
sudo ls -l "$KEY_DIR"


