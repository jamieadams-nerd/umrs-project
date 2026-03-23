#!/usr/bin/python3
"""
Build bilingual CCCS glossary by correlating EN and FR pages.
The CCCS FR page often includes the English term in parentheses after the French term.
Also uses a curated manual mapping for the most important cybersecurity terms.
Replaces CCCS entries added by parse_cccs_glossary.py
"""
import csv
import html as html_lib
import re
import sys

EN_HTML = "/tmp/cccs-glossary-en.html"
FR_HTML = "/tmp/cccs-glossary-fr.html"
OUTPUT_PATH = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"
DOMAIN_TAG = "cybersecurity"
SOURCE_TAG = "CCCS Glossary (cyber.gc.ca)"


def clean(s: str) -> str:
    s = re.sub(r"<[^>]+>", " ", s)
    s = html_lib.unescape(s)
    s = re.sub(r"\s+", " ", s).strip()
    return s


def extract_dl_terms(path: str) -> dict:
    """Extract DL-based glossary terms. Returns {term_lower: (term, definition)}"""
    with open(path, encoding="utf-8", errors="replace") as f:
        content = f.read()
    content = re.sub(r"<script[^>]*>.*?</script>", "", content, flags=re.DOTALL | re.IGNORECASE)
    content = re.sub(r"<style[^>]*>.*?</style>", "", content, flags=re.DOTALL | re.IGNORECASE)
    terms = {}
    dl_match = re.findall(r"<dl[^>]*>(.*?)</dl>", content, re.DOTALL | re.IGNORECASE)
    for dl in dl_match:
        dts = re.findall(r"<dt[^>]*>(.*?)</dt>", dl, re.DOTALL | re.IGNORECASE)
        dds = re.findall(r"<dd[^>]*>(.*?)</dd>", dl, re.DOTALL | re.IGNORECASE)
        for dt, dd in zip(dts, dds):
            term = clean(dt)
            defn = clean(dd)
            if term and defn and len(term) < 120:
                terms[term.lower()] = (term, defn)
    return terms


def correlate_bilingual(en_terms: dict, fr_terms: dict) -> list:
    """
    Correlate EN and FR terms.
    Strategy 1: French definitions contain English term in parentheses "(For X)" or after " (X )"
    Strategy 2: Manual curated mapping for critical cybersecurity terms
    Strategy 3: EN terms that have a known French equivalent from context
    """
    pairs = []  # (en_term, fr_term, notes)
    used_fr = set()

    # Build lookup by English acronym mentions in French definitions
    # e.g., "Authentification (Authentication)"  or "coupe-feu (firewall)"
    en_by_key = {v[0].lower(): v[0] for v in en_terms.values()}

    for fr_key, (fr_term, fr_defn) in fr_terms.items():
        # Look for English term reference in parentheses within the French term itself
        # e.g., "Attaque par déni de service (Denial of Service)"
        en_paren = re.findall(r"\(([A-Za-z][^)]{2,60}?)\)", fr_term)
        for ep in en_paren:
            ep_lower = ep.lower().strip()
            if ep_lower in en_terms:
                en_t = en_terms[ep_lower][0]
                pairs.append((en_t, fr_term, f"CCCS bilingual pair; {fr_defn[:80]}"))
                used_fr.add(fr_key)
                break

        # Also look in the French definition
        if fr_key not in used_fr:
            en_paren_in_defn = re.findall(r"\(([A-Z][A-Za-z\s]{2,40})\)", fr_defn)
            for ep in en_paren_in_defn:
                ep_lower = ep.lower().strip()
                if ep_lower in en_terms:
                    en_t = en_terms[ep_lower][0]
                    pairs.append((en_t, fr_term, f"CCCS bilingual pair; {fr_defn[:80]}"))
                    used_fr.add(fr_key)
                    break

    # Manual curated core mapping for UMRS-critical terms
    # EN term -> FR term (from CCCS FR page)
    MANUAL_MAPPING = {
        "Access control": "Contrôle d'accès",
        "Administrative privileges": "Privilèges d'administration",
        "Allow list": "Liste d'autorisation",
        "Anti-virus software": "Logiciel antivirus",
        "Authentication": "Authentification",
        "Authorization": "Autorisation",
        "Availability": "Disponibilité",
        "Backdoor": "Porte dérobée",
        "Baseline security controls": "Contrôles de sécurité de base",
        "Beaconing": "Balisage",
        "Blockchain": "Chaîne de blocs",
        "Certificate": "Certificat",
        "Confidentiality": "Confidentialité",
        "Credential": "Justificatif d'identité",
        "Cryptography": "Cryptographie",
        "Cyber hygiene": "Cygiène informatique",
        "Cyber threat": "Cybermenace",
        "Cybersecurity": "Cybersécurité",
        "Data breach": "Atteinte à la protection des données",
        "Data classification": "Classification des données",
        "Defence in depth": "Défense en profondeur",
        "Denial of service attack": "Attaque par déni de service",
        "Digital signature": "Signature numérique",
        "Distributed denial of service attack": "Attaque par déni de service distribué",
        "Encryption": "Chiffrement",
        "Endpoint": "Terminal",
        "Event": "Événement",
        "Exploit": "Vulnérabilité exploitée",
        "Firewall": "Coupe-feu",
        "Hashing": "Hachage",
        "Identity and access management": "Gestion des identités et des accès",
        "Incident": "Incident",
        "Indicator of compromise": "Indicateur de compromission",
        "Information security": "Sécurité de l'information",
        "Integrity": "Intégrité",
        "Intrusion detection": "Détection d'intrusion",
        "Lateral movement": "Déplacement latéral",
        "Least privilege": "Principe du moindre privilège",
        "Malware": "Logiciel malveillant",
        "Multi-factor authentication": "Authentification multifactorielle",
        "Network segmentation": "Segmentation du réseau",
        "Patch": "Correctif",
        "Patch management": "Gestion des correctifs",
        "Phishing": "Hameçonnage",
        "Privilege escalation": "Escalade des privilèges",
        "Ransomware": "Rançongiciel",
        "Risk": "Risque",
        "Risk assessment": "Évaluation des risques",
        "Role-based access control": "Contrôle d'accès basé sur les rôles",
        "Sandbox": "Bac à sable",
        "Security audit": "Audit de sécurité",
        "Security information and event management": "Gestion des informations et des événements de sécurité",
        "Security Operations Centre": "Centre des opérations de sécurité",
        "Security posture": "Posture de sécurité",
        "Social engineering": "Ingénierie sociale",
        "Spear phishing": "Hameçonnage ciblé",
        "Spoofing": "Usurpation d'identité",
        "Threat": "Menace",
        "Threat actor": "Auteur de menace",
        "Threat intelligence": "Renseignement sur les menaces",
        "Two-factor authentication": "Authentification à deux facteurs",
        "Unpatched application": "Application non corrigée",
        "Virtual private network": "Réseau privé virtuel",
        "Vulnerability": "Vulnérabilité",
        "Zero-day vulnerability": "Vulnérabilité du jour zéro",
    }

    # Add manual mappings
    used_en = {en.lower() for en, fr, _ in pairs}
    for en_term, fr_term in MANUAL_MAPPING.items():
        en_key = en_term.lower()
        if en_key not in used_en:
            used_en.add(en_key)
            # Get definition from EN page if available
            defn = en_terms.get(en_key, (en_term, ""))[1]
            notes = f"CCCS manual pair; {defn[:80]}" if defn else "CCCS manual pair"
            pairs.append((en_term, fr_term, notes))

    # Add remaining EN terms that we don't have FR for
    for en_key, (en_term, en_defn) in en_terms.items():
        if en_key not in used_en:
            used_en.add(en_key)
            notes = f"CCCS EN only; {en_defn[:80]}"
            pairs.append((en_term, en_term, notes))  # EN term as placeholder

    return pairs


def load_non_cccs_entries() -> tuple:
    non_cccs = []
    seen = set()
    try:
        with open(OUTPUT_PATH, encoding="utf-8") as f:
            reader = csv.DictReader(f, delimiter="\t")
            for row in reader:
                src = row.get("source", "")
                if "CCCS" not in src:
                    non_cccs.append(row)
                    en = row.get("english_term", "").lower().strip()
                    if en:
                        seen.add(en)
    except FileNotFoundError:
        pass
    return non_cccs, seen


if __name__ == "__main__":
    print("Loading EN glossary...", file=sys.stderr)
    en_terms = extract_dl_terms(EN_HTML)
    print(f"  EN terms: {len(en_terms)}", file=sys.stderr)

    print("Loading FR glossary...", file=sys.stderr)
    fr_terms = extract_dl_terms(FR_HTML)
    print(f"  FR terms: {len(fr_terms)}", file=sys.stderr)

    print("Correlating bilingual pairs...", file=sys.stderr)
    pairs = correlate_bilingual(en_terms, fr_terms)
    print(f"  Pairs: {len(pairs)}", file=sys.stderr)

    # Show sample bilingual pairs (exclude EN-only)
    bilingual = [(e, f, n) for e, f, n in pairs if e != f]
    print(f"  Bilingual pairs: {len(bilingual)}", file=sys.stderr)
    for e, f, n in bilingual[:8]:
        print(f"  {e!r} -> {f!r}", file=sys.stderr)

    print("\nLoading existing non-CCCS entries...", file=sys.stderr)
    existing_rows, seen = load_non_cccs_entries()
    print(f"  Non-CCCS entries: {len(existing_rows)}", file=sys.stderr)

    new_entries = []
    for (en, fr, notes) in pairs:
        if not en:
            continue
        en_key = en.lower()
        if en_key not in seen:
            seen.add(en_key)
            new_entries.append((en, fr, DOMAIN_TAG, SOURCE_TAG, notes[:150]))

    print(f"New entries to add: {len(new_entries)}", file=sys.stderr)

    # Rewrite TSV
    with open(OUTPUT_PATH, "w", encoding="utf-8", newline="") as out:
        out.write("english_term\tfr_ca_term\tdomain\tsource\tnotes\n")
        for row in existing_rows:
            sanitized = tuple(
                str(row.get(k, "")).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for k in ["english_term", "fr_ca_term", "domain", "source", "notes"]
            )
            out.write("\t".join(sanitized) + "\n")
        for row in sorted(new_entries, key=lambda r: r[0].lower()):
            sanitized = tuple(
                str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for f in row
            )
            out.write("\t".join(sanitized) + "\n")

    total = len(existing_rows) + len(new_entries)
    print(f"Written {total} total entries ({len(new_entries)} CCCS)", file=sys.stderr)
    print(f"DONE: {len(new_entries)} CCCS entries")
