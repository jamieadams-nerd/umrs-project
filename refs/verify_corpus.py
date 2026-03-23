#!/usr/bin/python3
"""Quality verification for French terminology TSV files."""
import csv
import sys

KEY_TERMS = [
    "encryption", "security posture", "hardening", "kernel",
    "mandatory access control", "audit trail", "vulnerability",
    "certificate", "digital signature", "public key", "hash function",
    "threat", "risk assessment", "compliance", "identity", "privilege",
    "authentication", "authorization", "access control list",
    "cryptography", "integrity", "confidentiality", "availability",
    "incident response", "patch management", "configuration management",
    "least privilege", "separation of duties",
    "security label", "security policy", "security audit",
    "denial of service", "malware", "intrusion detection",
    "firewall", "vpn", "tls", "multi-factor authentication",
    "vulnerability assessment", "penetration testing",
    "information security",
]


def check_terms(filepath, source_name):
    found = {}
    total = 0
    with open(filepath, encoding="utf-8") as f:
        reader = csv.DictReader(f, delimiter="\t")
        for row in reader:
            total += 1
            en = row.get("english_term", "").lower()
            for t in KEY_TERMS:
                if t == en and t not in found:
                    found[t] = row.get("fr_ca_term", "")
                elif t in en and t not in found:
                    found[t] = f"(partial match) {row.get('fr_ca_term', '')}"

    print(f"\n=== {source_name} ({total} entries) ===")
    hits = 0
    misses = []
    for t in sorted(KEY_TERMS):
        if t in found:
            hits += 1
            print(f"  OK  {t:<40} -> {found[t]}")
        else:
            misses.append(t)

    if misses:
        print(f"\n  MISSING ({len(misses)}):")
        for m in misses:
            print(f"       {m}")
    print(f"\n  Coverage: {hits}/{len(KEY_TERMS)} ({100*hits//len(KEY_TERMS)}%)")


if __name__ == "__main__":
    base = "/media/psf/repos/umrs-project/.claude/corpus"
    check_terms(f"{base}/termium-plus-fr_CA.tsv", "TERMIUM Plus (GoC)")
    check_terms(f"{base}/oqlf-gdt-fr_CA.tsv", "OQLF GDT (Québec)")
