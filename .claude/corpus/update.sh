#https://translationproject.org/PO-files/fr/coreutils-9.9.280.fr.po
#wget -P .claude/corpus/gnu-fr/ https://translationproject.org/PO-files/fr/coreutils-9.9.280.fr.po


#!/bin/bash

# Define the destination
DEST="./"
mkdir -p "$DEST"

# List your URLs here
URLS=(
    "https://translationproject.org/PO-files/fr/coreutils-9.9.280.fr.po"
    "https://translationproject.org/PO-files/fr/bash-5.3-rc2.fr.po"
    "https://translationproject.org/PO-files/fr/cryptsetup-2.8.2-rc0.fr.po"
    "https://translationproject.org/PO-files/fr/findutils-4.9.0.fr.po"
    "https://translationproject.org/PO-files/fr/grep-3.11.68.fr.po"
    "https://translationproject.org/PO-files/fr/sed-4.8.44.fr.po"
    "https://translationproject.org/PO-files/fr/tar-1.35.90.fr.po"
)

for URL in "${URLS[@]}"; do
    # Get the filename from the URL (e.g., coreutils-9.5.fr.po)
    FILENAME=$(basename "$URL")
    
    echo "Downloading $FILENAME..."
    
    # Download and save to the specific directory
    wget -q "$URL" -O "$DEST/$FILENAME"
done

echo "Done! Corpus updated in $DEST"





