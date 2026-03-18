#!/usr/bin/env bash
set -euo pipefail

OUT="umrs-cpuaudit-linux.txt"

exec > >(tee "$OUT") 2>&1

echo "=== UMRS CPU AUDIT (LINUX) ==="
echo "Timestamp: $(date -Is)"
echo

echo "== OS Identity =="
uname -a
echo
cat /etc/os-release
echo

echo "== CPU Raw Info (/proc/cpuinfo) =="
cat /proc/cpuinfo
echo

echo "== CPU Summary (lscpu) =="
lscpu
echo

echo "== CPU Feature Subsets =="

echo "-- Crypto ISA --"
grep -oE '(aes|pmull|sha1|sha2|sha3|sha512|crc32|atomics)' /proc/cpuinfo | sort -u || true
echo

echo "-- SIMD / FP --"
grep -oE '(asimd|fphp|asimdhp|asimdrdm|asimddp|asimdfhm|fcma|jscvt)' /proc/cpuinfo | sort -u || true
echo

echo "-- Pointer Auth / Memory Ordering --"
grep -oE '(paca|pacg|uscat|lrcpc|ilrcpc|dcpop|dcpodp|flagm|flagm2)' /proc/cpuinfo | sort -u || true
echo

echo "== Core Topology =="
nproc || true
lscpu | grep -E 'CPU\(s\)|Thread|Core|Socket' || true
echo

echo "== Cache Hierarchy =="
lscpu | grep -E 'L1d|L1i|L2|L3' || echo "Cache info not reported by lscpu"
echo

echo "== Virtualization Surface =="
systemd-detect-virt || true
echo

echo "-- dmesg (hypervisor / rng) --"
if command -v sudo >/dev/null 2>&1; then
  sudo dmesg | grep -i hypervisor || true
  sudo dmesg | grep -i rng || true
else
  echo "sudo not available; skipping dmesg"
fi
echo

echo "== Entropy Surface =="
cat /proc/sys/kernel/random/entropy_avail || true
echo

echo "== Kernel Config Hints (crypto) =="
if [ -r /proc/config.gz ]; then
  zgrep -E 'CRYPTO_AES|CRYPTO_SHA|CRYPTO_SHA3|CRYPTO_SM3|CRYPTO_SM4|CRYPTO_USER_API' /proc/config.gz || true
else
  echo "/proc/config.gz not available"
fi
echo

echo "== ELF Auxv HWCAP Flags =="
if command -v getauxval >/dev/null 2>&1; then
  getauxval AT_HWCAP || true
  getauxval AT_HWCAP2 || true
else
  echo "getauxval not available"
fi
echo

echo "== End of UMRS CPU Audit =="

