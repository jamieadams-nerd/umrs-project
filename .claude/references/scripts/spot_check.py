#!/usr/bin/python3
"""Spot-check specific UMRS terms in both corpus TSVs."""
import csv

TARGETS = [
    'encryption', 'mandatory access control', 'security posture',
    'hardening', 'audit trail', 'least privilege', 'firewall',
    'digital signature', 'authentication', 'multi-factor authentication',
    'ransomware', 'malware', 'denial of service attack', 'patch management',
    'separation of duties', 'access control list', 'vulnerability',
    'certificate', 'cryptography', 'hash function', 'integrity',
    'incident response', 'penetration testing',
]
targets = set(TARGETS)

for path, label in [
    ('/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv', 'TERMIUM Plus'),
    ('/media/psf/repos/umrs-project/.claude/corpus/oqlf-gdt-fr_CA.tsv', 'OQLF GDT'),
]:
    print(f'\n=== {label} — UMRS key terms ===')
    found = {}
    with open(path, encoding='utf-8') as f:
        reader = csv.DictReader(f, delimiter='\t')
        for row in reader:
            en = row.get('english_term','').strip().lower()
            fr = row.get('fr_ca_term','').strip()
            src = row.get('source','').strip()
            notes = row.get('notes','').strip()
            if en in targets and en not in found:
                found[en] = (fr, src[:30])
    for t in sorted(TARGETS):
        if t in found:
            fr, src = found[t]
            print(f'  OK  {t:<35} -> {fr:<40} [{src}]')
        else:
            print(f'  --  {t}')
