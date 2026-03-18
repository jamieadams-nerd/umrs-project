#!/usr/bin/bash
#
#

echo
echo "OpenSSL Version:" 
openssl version -a

echo
echo "OpenSSL Providers:"
openssl list -providers

openssl list -providers -verbose

echo 
echo "OpenSSL algorithm surface inventory:"
openssl list -cipher-algorithms
openssl list -digest-algorithms
openssl list -mac-algorithms

echo 
echo "Public-key + PQ readiness surface:"
echo "Public key:"
openssl list -public-key-algorithms
echo
echo "Key Managers:"
openssl list -key-managers
echo
echo "Signature algorithms"
openssl list -signature-algorithms


echo 
echo "OpenSSL Engine / hardware offload surface"
openssl engine -t -c

echo 
echo "ARM Crypto acceleration linkage test:"
openssl speed -evp aes-128-gcm
echo
openssl speed -evp sha256
echo
openssl speed -evp sha3-256

echo
echo "Kernel crypto API surface:"
grep -E 'aes|sha|gcm|chacha|poly1305' /proc/crypto



