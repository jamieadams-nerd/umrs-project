#!/bin/bash
# IMA/EVM Recursive Re-signer
# Usage: sudo ./ima-refresh.sh /path/to/monitor/

TARGET=$1
KEY="/etc/keys/privkey_evm.pem"

if [ -z "$TARGET" ]; then
    echo "Usage: $0 <file_or_directory>"
    exit 1
fi

echo "Re-signing $TARGET and subdirectories..."

# Find all regular files and sign them
find "$TARGET" -type f | while read -r FILE; do
    echo "Signing: $FILE"
    evmctl sign --imasig --key "$KEY" "$FILE"
done

echo "Done. Verifying one sample..."
evmctl verify "$TARGET" 2>/dev/null || evmctl ima_verify --key /etc/keys/x509_evm.der "$TARGET"

