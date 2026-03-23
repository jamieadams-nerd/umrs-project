#!/usr/bin/python3
"""
Extract UMRS-relevant terminology from Termium Plus CSV ZIPs.
Writes output to .claude/corpus/termium-plus-fr_CA.tsv
"""
import zipfile
import csv
import io
import sys
import os
import ssl
import urllib.request

# SSL context for GoC servers
CTX = ssl.create_default_context()
CTX.check_hostname = False
CTX.verify_mode = ssl.CERT_NONE

BASE_URL = "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/"

# Known subject ZIPs with their local names
SUBJECT_ZIPS = {
    "electronics-informatics": "domaine-ElectroniqueInformatique-subject-ElectronicsInformatics.zip",
    "administration": "domaine-subject-administration.zip",
}

# Try to discover Military/Security ZIP URL
MIL_CANDIDATES = [
    "domaine-MilitaireSecurite-subject-MilitaryAndSecurity.zip",
    "domaine-MilitaireArme-subject-MilitaryAndArms.zip",
    "domaine-SureteMilitaire-subject-MilitaryAndSecurity.zip",
    "domaine-subject-MilitaryAndSecurity.zip",
    "domaine-NationaleDefence-subject-NationalDefence.zip",
    "domaine-DefenceNationale-subject-NationalDefence.zip",
    "domaine-ArmeeMilitaire-subject-MilitaryArms.zip",
]

UMRS_KEYWORDS = [
    "secur", "encrypt", "crypt", "access", "auth", "audit",
    "policy", "compli", "integr", "label", "classif",
    "vulner", "threat", "risk", "monitor", "config",
    "harden", "certif", "hash", "signat", "key ", " key",
    "cipher", "firewall", "intrusion", "incident", "patch",
    "ident", "cred", "passw", "token", "algorithm",
    "assess", "remedi", "mitigat", "safeguard",
    "confidential", "availab", "repudiat", "protect",
    "cyber", "network", "operat", "software", "firmware",
    "user account", "role-bas", "session", "process",
    "permission", "privilege", "mandatory", "discretionary",
    "daemon", "port ", " port", "interface",
    "public key", "private key", "asymmetr", "symmetr",
    "integrity check", "data integ", "checksum", "digest",
    "malware", "ransomware", "exploit", "zero-day",
    "information security", "information system",
    "security posture", "security control", "security audit",
    "access control", "mandatory access", "discretionary access",
    "security event", "security label", "security context",
    "hardening", "baseline", "remediation", "countermeasure",
    "non-repudiation", "data protection", "privacy",
    "system admin", "root", "superuser", "least privilege",
    "separation of duties", "need to know", "clearance",
    "classification level", "sensitive information",
    "need-to-know",
]

# Exact terms that are always UMRS-relevant
EXACT_TERMS = {
    "security", "encryption", "decryption", "access control",
    "audit", "compliance", "integrity", "authentication",
    "authorization", "authorisation", "posture", "indicator",
    "kernel", "module", "enforcement", "policy", "label",
    "classification", "vulnerability", "threat", "risk",
    "monitoring", "configuration", "hardening", "certificate",
    "key", "hash", "digest", "digital signature", "signature",
    "firewall", "intrusion detection", "intrusion prevention",
    "logging", "event log", "alert", "incident", "response",
    "patch", "exploit", "malware", "ransomware", "spyware",
    "identity", "credential", "password", "token",
    "public key", "private key", "asymmetric encryption",
    "symmetric encryption", "algorithm", "standard",
    "framework", "assessment", "risk management", "baseline",
    "remediation", "mitigation", "countermeasure", "safeguard",
    "confidentiality", "availability", "non-repudiation",
    "data protection", "information security", "cybersecurity",
    "cyber security", "network security", "system security",
    "operating system", "firmware", "platform",
    "user", "role", "account", "session", "process",
    "file system", "filesystem", "permission", "ownership",
    "service", "daemon", "port", "interface", "protocol",
    "mandatory access control", "discretionary access control",
    "security label", "security context", "security policy",
    "security posture", "security control", "security audit",
    "security event", "security incident", "security alert",
    "data integrity", "data classification", "data handling",
    "need to know", "least privilege", "separation of duties",
    "clearance level", "sensitive information", "classified",
    "cryptography", "cryptographic", "key management",
    "certificate authority", "public key infrastructure",
    "digital certificate", "revocation", "crl", "ocsp",
    "tls", "ssl", "https", "vpn", "ipsec",
    "penetration testing", "vulnerability assessment",
    "security testing", "code review", "static analysis",
    "threat model", "attack surface", "threat vector",
    "intrusion", "breach", "disclosure", "exfiltration",
    "insider threat", "social engineering", "phishing",
    "denial of service", "distributed denial of service",
    "buffer overflow", "injection", "cross-site scripting",
    "privilege escalation", "lateral movement",
    "defense in depth", "zero trust", "security by design",
    "secure coding", "code signing", "software supply chain",
    "hash function", "message digest", "hmac", "mac",
    "block cipher", "stream cipher", "key derivation",
    "random number generator", "entropy", "seed",
    "audit trail", "audit log", "syslog", "siem",
    "patch management", "vulnerability management",
    "configuration management", "change management",
    "incident response", "business continuity",
    "disaster recovery", "backup", "recovery point",
    "multi-factor authentication", "two-factor authentication",
    "single sign-on", "federation", "identity provider",
    "attribute", "claim", "assertion", "principal",
    "subject", "object", "resource", "action",
    "rule", "policy enforcement", "enforcement point",
    "selinux", "apparmor", "mandatory integrity control",
}


def is_relevant(term_en: str) -> bool:
    if not term_en:
        return False
    lower = term_en.lower().strip()
    if lower in EXACT_TERMS:
        return True
    for kw in UMRS_KEYWORDS:
        if kw in lower:
            return True
    return False


def extract_from_zip(zip_path: str, domain_tag: str, results: list, seen: set):
    """Extract relevant terms from a Termium ZIP file."""
    try:
        z = zipfile.ZipFile(zip_path)
    except Exception as e:
        print(f"ERROR opening {zip_path}: {e}", file=sys.stderr)
        return 0

    count = 0
    for fname in z.namelist():
        if not fname.endswith(".csv"):
            continue
        with z.open(fname) as f:
            raw = f.read().decode("utf-8-sig", errors="replace")
        reader = csv.DictReader(io.StringIO(raw))
        for row in reader:
            term_en = (row.get("TERM_EN") or "").strip()
            term_fr = (row.get("TERME_FR") or "").strip()
            subject_en = (row.get("SUBJECT_EN") or "").strip()
            abbr_en = (row.get("ABBREVIATION_EN") or "").strip()
            abbr_fr = (row.get("ABBREVIATION_FR") or "").strip()
            syn_en = (row.get("SYNONYMS_EN") or "").strip()
            syn_fr = (row.get("SYNONYMES_FR") or "").strip()
            param_fr = (row.get("TERME_FR_PARAMETRE") or "").strip()

            if not term_en or not term_fr:
                continue
            if not is_relevant(term_en):
                continue

            key = term_en.lower()
            if key in seen:
                continue
            seen.add(key)

            # Build notes
            notes_parts = []
            if param_fr:
                # Extract just grammatical notes (gender, etc.)
                gram = "; ".join(
                    p.strip()
                    for p in param_fr.split(",")
                    if p.strip() and p.strip()
                    not in ("correct", "standardized", "normalized")
                )
                if gram:
                    notes_parts.append(gram)
            if abbr_fr:
                notes_parts.append(f"abrév: {abbr_fr}")

            notes = "; ".join(notes_parts) if notes_parts else ""
            results.append((term_en, term_fr, domain_tag, "Termium Plus", notes))
            count += 1

            # Add synonyms as separate entries
            if syn_en and syn_fr:
                syns_en = [s.strip() for s in syn_en.split(";")]
                syns_fr = [s.strip() for s in syn_fr.split(";")]
                for se, sf in zip(syns_en, syns_fr):
                    if se and sf:
                        se_key = se.lower()
                        if se_key not in seen and is_relevant(se):
                            seen.add(se_key)
                            results.append(
                                (se, sf, domain_tag, "Termium Plus", f"synonym of: {term_en}")
                            )
                            count += 1
    return count


def probe_mil_zip():
    """Try to find the Military/Security ZIP URL."""
    for candidate in MIL_CANDIDATES:
        url = BASE_URL + candidate
        req = urllib.request.Request(
            url, method="HEAD", headers={"User-Agent": "Mozilla/5.0"}
        )
        try:
            with urllib.request.urlopen(req, context=CTX, timeout=10) as r:
                size = r.getheader("Content-Length", "?")
                print(f"FOUND: {url} ({size} bytes)")
                return url, candidate
        except Exception:
            pass
    return None, None


if __name__ == "__main__":
    results = []
    seen = set()

    # Process already-downloaded ZIPs
    elec_zip = "/tmp/termium-elec-info.zip"
    admin_zip = "/tmp/termium-admin.zip"

    n = extract_from_zip(elec_zip, "information technology", results, seen)
    print(f"Electronics/Informatics: {n} terms extracted", file=sys.stderr)

    n = extract_from_zip(admin_zip, "public administration", results, seen)
    print(f"Administration: {n} terms extracted", file=sys.stderr)

    # Try Military/Security
    mil_url, mil_fname = probe_mil_zip()
    if mil_fname:
        mil_local = f"/tmp/termium-mil-sec-found.zip"
        req = urllib.request.Request(mil_url, headers={"User-Agent": "Mozilla/5.0"})
        with urllib.request.urlopen(req, context=CTX, timeout=90) as r:
            with open(mil_local, "wb") as out:
                out.write(r.read())
        n = extract_from_zip(mil_local, "national defence", results, seen)
        print(f"Military/Security: {n} terms extracted", file=sys.stderr)
    else:
        print("Military/Security ZIP URL not found", file=sys.stderr)

    # Sort by domain then term
    results.sort(key=lambda r: (r[2], r[0].lower()))

    # Write TSV
    output_path = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"
    with open(output_path, "w", encoding="utf-8", newline="") as out:
        out.write("english_term\tfr_ca_term\tdomain\tsource\tnotes\n")
        for row in results:
            # Sanitize tabs within fields
            sanitized = tuple(
                str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for f in row
            )
            out.write("\t".join(sanitized) + "\n")

    print(f"Written {len(results)} entries to {output_path}", file=sys.stderr)
    print(f"DONE: {len(results)} entries", file=sys.stdout)
