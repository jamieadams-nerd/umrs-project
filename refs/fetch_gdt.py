#!/usr/bin/python3
"""
Fetch OQLF Grand dictionnaire terminologique open data from Données Québec.
Extracts UMRS-relevant terms and writes to .claude/corpus/oqlf-gdt-fr_CA.tsv
"""
import csv
import io
import json
import os
import re
import ssl
import sys
import urllib.request

CTX = ssl.create_default_context()
CTX.check_hostname = False
CTX.verify_mode = ssl.CERT_NONE


def fetch(url, timeout=30):
    req = urllib.request.Request(
        url, headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) Firefox/120.0"}
    )
    with urllib.request.urlopen(req, context=CTX, timeout=timeout) as r:
        return r.read()


def get_resource_url(resource_id):
    """Get download URL from Données Québec CKAN API."""
    api_url = f"https://www.donneesquebec.ca/recherche/api/action/resource_show?id={resource_id}"
    try:
        data = json.loads(fetch(api_url))
        result = data.get("result", {})
        return result.get("url"), result.get("name"), result.get("format")
    except Exception as e:
        print(f"CKAN API error for {resource_id}: {e}", file=sys.stderr)
        return None, None, None


# Known resource IDs from Données Québec
RESOURCES = {
    "terms": {
        "id": "882453c2-93c3-4204-b5ff-6d6297082ad9",
        "desc": "Termes officialisés (GDT)",
    },
    "fiches": {
        "id": "c3ce0af4-7c0f-4dd2-b53a-6dc7fb3ea5ef",
        "desc": "Fiches terminologiques (GDT)",
    },
}

# UMRS-relevant keywords for French term matching (subset focused on IT/security)
UMRS_KEYWORDS_FR = [
    "sécurité", "sécuri", "chiffr", "crypt", "accès", "auth",
    "audit", "conformi", "intégri", "étiquett", "classifi",
    "vulnér", "menace", "risque", "monitor", "configur",
    "durciss", "certif", "hacha", "hachage", "signature",
    "pare-feu", "intrusion", "incident", "correctif",
    "identit", "accréditat", "mot de passe", "jeton",
    "algorithme", "norme", "cadre", "évaluation",
    "remédiati", "atténuati", "contre-mesure",
    "confidentialit", "disponibilit", "répudiati",
    "protecti", "informatique", "logiciel", "réseau",
    "système exploit", "micrologiciel",
    "utilisateur", "rôle", "compte", "session", "processus",
    "permission", "privilège", "obligatoire", "discrétionnaire",
    "clé public", "clé privé", "asymétr", "symétri",
    "autorité de certif", "infrastructure à clés public",
    "contrôle d'accès", "accès obligatoire",
    "posture de sécurité", "contrôle de sécurité",
    "événement de sécurité", "incident de sécurité",
    "intégrité des données", "classification des données",
    "cybers", "sécurité de l'information",
    "gestion des risques", "gestion des correctifs",
    "gestion de la configuration", "gestion des incidents",
]

UMRS_KEYWORDS_EN = [
    "secur", "encrypt", "crypt", "access control", "auth",
    "audit", "compli", "integr", "label", "classif",
    "vulner", "threat", "risk", "monitor", "config",
    "harden", "certif", "hash", "signat", "key",
    "cipher", "firewall", "intrusion", "incident",
    "ident", "cred", "passw", "token", "algorithm",
    "assess", "remedi", "mitigat", "safeguard",
    "confidential", "availab", "repudiat", "protect",
    "cyber", "network", "operat", "software", "firmware",
    "permission", "privilege", "mandatory", "discretionary",
    "information security", "security posture",
]


def is_relevant(term_en="", term_fr="", domain=""):
    """Check if a term is UMRS-relevant."""
    if term_en:
        lower_en = term_en.lower()
        if any(kw in lower_en for kw in UMRS_KEYWORDS_EN):
            return True
    if term_fr:
        lower_fr = term_fr.lower()
        if any(kw in lower_fr for kw in UMRS_KEYWORDS_FR):
            return True
    if domain:
        lower_d = domain.lower()
        if any(kw in lower_d for kw in ["informati", "sécuri", "défense", "administr", "cyber"]):
            return True
    return False


def parse_terms_csv(raw_bytes):
    """
    Parse the 'Termes officialisés' CSV.
    We need to inspect the actual column names first.
    """
    raw = raw_bytes.decode("utf-8-sig", errors="replace")
    reader = csv.DictReader(io.StringIO(raw))
    print("Columns:", reader.fieldnames, file=sys.stderr)
    rows = list(reader)
    print(f"Total rows: {len(rows)}", file=sys.stderr)
    if rows:
        print("Sample row:", dict(list(rows[0].items())[:8]), file=sys.stderr)
    return rows


def parse_fiches_csv(raw_bytes):
    """Parse the 'Fiches terminologiques' CSV."""
    raw = raw_bytes.decode("utf-8-sig", errors="replace")
    reader = csv.DictReader(io.StringIO(raw))
    print("Columns:", reader.fieldnames, file=sys.stderr)
    rows = list(reader)
    print(f"Total rows: {len(rows)}", file=sys.stderr)
    if rows:
        print("Sample row:", dict(list(rows[0].items())[:8]), file=sys.stderr)
    return rows


if __name__ == "__main__":
    results = []
    seen = set()

    for key, info in RESOURCES.items():
        print(f"\nFetching resource: {info['desc']} ({info['id']})", file=sys.stderr)
        download_url, name, fmt = get_resource_url(info["id"])
        print(f"  URL: {download_url}", file=sys.stderr)
        print(f"  Name: {name}, Format: {fmt}", file=sys.stderr)

        if not download_url:
            print("  No URL found, skipping", file=sys.stderr)
            continue

        # Download the CSV
        try:
            raw = fetch(download_url, timeout=120)
            print(f"  Downloaded: {len(raw)} bytes", file=sys.stderr)
        except Exception as e:
            print(f"  Download error: {e}", file=sys.stderr)
            continue

        # Save locally for inspection
        local = f"/tmp/gdt-{key}.csv"
        with open(local, "wb") as f:
            f.write(raw)

        # Parse and inspect
        if key == "terms":
            rows = parse_terms_csv(raw)
        else:
            rows = parse_fiches_csv(raw)

        # Extract relevant rows
        for row in rows:
            # Try to find English/French term columns
            # Column names vary — try common patterns
            term_en = ""
            term_fr = ""
            domain = ""
            notes = ""

            # Try various column name patterns
            for col in row:
                col_lower = col.lower()
                if "anglai" in col_lower or "_en" in col_lower or "english" in col_lower:
                    if not term_en:
                        term_en = (row[col] or "").strip()
                elif "français" in col_lower or "franç" in col_lower or "_fr" in col_lower or "french" in col_lower or "terme" in col_lower:
                    if not term_fr:
                        term_fr = (row[col] or "").strip()
                elif "domain" in col_lower or "domaine" in col_lower or "subject" in col_lower or "sujet" in col_lower:
                    if not domain:
                        domain = (row[col] or "").strip()

            if not term_fr:
                continue

            if not is_relevant(term_en, term_fr, domain):
                continue

            key_dedup = term_fr.lower()
            if key_dedup in seen:
                continue
            seen.add(key_dedup)

            # Map domain to standardized tag
            domain_tag = "information technology"
            if domain:
                d_lower = domain.lower()
                if "défense" in d_lower or "military" in d_lower or "arm" in d_lower:
                    domain_tag = "national defence"
                elif "administr" in d_lower:
                    domain_tag = "public administration"
                elif "sécuri" in d_lower or "security" in d_lower:
                    domain_tag = "information security"
                elif "informati" in d_lower or "informatique" in d_lower:
                    domain_tag = "information technology"

            results.append((term_en or term_fr, term_fr, domain_tag, "OQLF GDT", domain or ""))

    # Sort
    results.sort(key=lambda r: (r[2], r[0].lower()))

    output_path = "/media/psf/repos/umrs-project/.claude/corpus/oqlf-gdt-fr_CA.tsv"
    with open(output_path, "w", encoding="utf-8", newline="") as out:
        out.write("english_term\tfr_ca_term\tdomain\tsource\tnotes\n")
        for row in results:
            sanitized = tuple(
                str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for f in row
            )
            out.write("\t".join(sanitized) + "\n")

    print(f"\nWritten {len(results)} entries to {output_path}", file=sys.stderr)
    print(f"DONE: {len(results)} entries", file=sys.stdout)
