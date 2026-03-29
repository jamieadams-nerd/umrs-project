#!/usr/bin/python3
"""
Extract UMRS-relevant terminology from OQLF GDT CSV files.
Produces clean .claude/corpus/oqlf-gdt-fr_CA.tsv for Simone's french-lookup skill.

GDT CSV columns (fiches):  Termes_francais, Termes_anglais, Definition, Domaines, Date_fiche, URL_fiche
GDT CSV columns (termes):  Termes_Officialises, Type_Officialisation, Domaines, Date_fiche,
                            Date_officialisation, Definition, Equivalent_anglais, URL_fiche
"""
import csv
import io
import re
import sys

# IT and security domain patterns - only accept terms from these GDT subject areas
IT_SEC_DOMAINS = [
    "informati",     # informatique, information
    "sécuri",        # sécurité logique, sécurité informatique
    "cybersécuri",   # cybersécurité
    "télécommunic",  # telecommunications
    "réseaux",       # networks
    "logiciel",      # software
    "internet",      # internet
    "système",       # systems
    "donnée",        # data
    "traitement",    # data processing
    "intelligence artificielle",
    "cryptograph",   # cryptography
    "chiffr",        # encryption
    "administrat",   # public administration (for policy/compliance terms)
    "défense",       # national defence
]

# Anti-patterns: exclude if domain matches these (to avoid false positives from road safety, etc.)
EXCLUDE_DOMAINS = [
    "sécurité routière",
    "route > sécurité",
    "sécurité nucléaire",
    "sécurité au travail",
    "sécurité industrielle",
    "protection individuelle",
    "équipement de protection",
    "manutention",
    "sécurité incendie",
    "securite incendie",
    "aviation > sécurité",
    "sécurité alimentaire",
    "transport",
    "bâtiment",
    "physique > rayonnement",
    "médecine",
    "imagerie médicale",
    "écologie",
    "génie civil",
    "mécanique",
    "chimie",
    "biologie",
    "agriculture",
    "arts et loisirs",
    "musique",
    "sport",
    "vêtement",
    "véhicule automobile",
    "industrie automobile",
    "circulation routière",
    "accidentolog",
    "électricité",
    "énergie",
]

UMRS_TERMS_EN = {
    "security", "encryption", "decryption", "access control",
    "audit", "compliance", "integrity", "authentication",
    "authorization", "authorisation", "indicator", "kernel",
    "module", "enforcement", "policy", "label", "classification",
    "vulnerability", "threat", "risk", "monitoring", "configuration",
    "hardening", "certificate", "key", "hash", "digest",
    "digital signature", "signature", "firewall",
    "intrusion detection", "intrusion prevention",
    "logging", "event log", "alert", "incident", "patch",
    "exploit", "malware", "ransomware", "spyware",
    "identity", "credential", "password", "token",
    "public key", "private key", "asymmetric encryption",
    "symmetric encryption", "algorithm", "standard",
    "framework", "assessment", "risk management", "baseline",
    "remediation", "mitigation", "countermeasure", "safeguard",
    "confidentiality", "availability", "non-repudiation",
    "data protection", "information security", "cybersecurity",
    "cyber security", "network security", "system security",
    "mandatory access control", "discretionary access control",
    "security label", "security context", "security policy",
    "security posture", "security control", "security audit",
    "security event", "security incident", "security alert",
    "data integrity", "data classification", "data handling",
    "need to know", "least privilege", "separation of duties",
    "clearance", "sensitive information", "classified",
    "cryptography", "key management", "certificate authority",
    "public key infrastructure", "digital certificate",
    "revocation", "tls", "ssl", "https", "vpn", "ipsec",
    "penetration testing", "vulnerability assessment",
    "security testing", "threat model", "attack surface",
    "intrusion", "breach", "exfiltration", "insider threat",
    "denial of service", "buffer overflow", "injection",
    "privilege escalation", "defense in depth", "zero trust",
    "secure coding", "code signing",
    "hash function", "message digest", "hmac",
    "block cipher", "stream cipher", "key derivation",
    "random number generator", "entropy",
    "audit trail", "syslog", "siem",
    "patch management", "vulnerability management",
    "configuration management", "incident response",
    "multi-factor authentication", "two-factor authentication",
    "single sign-on", "federation", "identity provider",
    "access right", "access privilege", "permission",
    "user account", "role", "session", "process",
    "file system", "daemon", "interface", "protocol",
    "open source software", "free software", "open source",
    "software license", "license",
    "computer security", "information system security",
    "network access", "remote access", "secure access",
    "trusted computing", "trusted platform",
    "security audit", "security review", "penetration test",
    "risk assessment", "threat assessment", "security assessment",
    "information assurance", "data security",
    "access log", "security log", "event log",
    "security policy", "security requirement",
    "security architecture", "security design",
    "security testing", "security validation",
    "malicious code", "malicious software", "spyware",
    "adware", "trojan", "worm", "virus",
    "botnet", "command and control",
    "encryption key", "decryption key", "session key",
    "master key", "root of trust", "hardware security module",
    "smart card", "hardware token",
    "biometric", "fingerprint", "iris scan",
    "privilege", "root access", "administrator",
    "sudo", "setuid", "capability", "mandatory",
}

UMRS_KEYWORDS_EN = [
    "secur", "encrypt", "crypt", "access control", "auth",
    "audit", "compli", "integr", "label", "classif",
    "vulner", "threat", "risk", "monitor", "config",
    "harden", "certif", "hash", "signat",
    "cipher", "firewall", "intrusion", "incident",
    "ident", "cred", "passw", "token", "algorithm",
    "assess", "remedi", "mitigat", "safeguard",
    "confidential", "availab", "repudiat", "protect",
    "cyber", "network", "software", "firmware",
    "privilege", "mandatory", "discretionary",
    "information security", "security posture",
    "computer security", "data protection",
    "malware", "exploit", "breach", "threat",
    "denial of service", "buffer overflow",
    "open source", "free software",
    "key management", "public key", "private key",
    "digital certificate", "certification authority",
    "root of trust", "hardware security",
]

UMRS_KEYWORDS_FR = [
    "sécurité informati", "sécurité logique", "sécurité physique",
    "sécurité du réseau", "sécurité des données",
    "sécurité des informations", "sécurité de l'inform",
    "cybersécuri", "chiffr", "crypt",
    "contrôle d'accès", "contrôle de l'accès",
    "authentifi", "autorisa", "audit",
    "confidentialit", "intégrit", "disponibilit",
    "vulnérab", "menace", "risque informati", "risque de sécuri",
    "logiciel malveillant", "maliciel", "programme malveillant",
    "pare-feu", "coupe-feu", "détection d'intrusion",
    "prévention d'intrusion", "incident de sécuri",
    "correctif", "mise à jour de sécuri",
    "clé de chiffr", "clé publique", "clé privée",
    "certificat numéri", "certificat électroni",
    "infrastructure à clés", "autorité de certifi",
    "signature numéri", "signature électroni",
    "hachage", "fonction de hachage",
    "logiciel libre", "code source ouvert", "logiciel ouvert",
    "droit d'accès", "privilege d'accès", "permission",
    "compte d'utilisateur", "mot de passe",
    "accès à distance", "accès sécuri",
    "identité numéri", "gestion des identit",
    "journalisation", "journal d'audit", "journal d'événement",
    "gestion des risques", "évaluation des risques",
    "politique de sécuri", "exigences de sécuri",
    "architecture de sécuri", "conception sécuri",
    "essai d'intrusion", "test de pénétr",
    "analyse de vulnérabil", "évaluation de sécuri",
    "zone démilitaris", "DMZ",
    "réseau privé virtuel", "RPV",
    "protocole sécuri", "tunneli",
    "chiffrement à clé", "chiffrement symétri", "chiffrement asymétri",
    "module de sécuri matérielle",
    "carte à puce", "jeton matériel",
    "biométri",
    "séparation des tâches", "principe du moindre privilège",
    "nécessité d'en connaître", "cloisonnement",
]


def is_domain_relevant(domain: str) -> bool:
    if not domain:
        return False
    d = domain.lower()
    # Check exclusions first
    for excl in EXCLUDE_DOMAINS:
        if excl in d:
            return False
    # Check inclusions
    for inc in IT_SEC_DOMAINS:
        if inc in d:
            return True
    return False


def is_term_relevant(term_en: str, term_fr: str) -> bool:
    if term_en:
        lower_en = term_en.lower()
        if lower_en in UMRS_TERMS_EN:
            return True
        if any(kw in lower_en for kw in UMRS_KEYWORDS_EN):
            return True
    if term_fr:
        lower_fr = term_fr.lower()
        if any(kw in lower_fr for kw in UMRS_KEYWORDS_FR):
            return True
    return False


def split_semicolons(s: str) -> list:
    """Split semicolon-separated synonyms, stripping whitespace."""
    if not s:
        return []
    return [p.strip() for p in s.split(";") if p.strip()]


def clean_term(t: str) -> str:
    """Remove grammatical annotations like (n. f.) (n. m.) (adj.) from GDT terms."""
    # Remove parenthetical annotations at end
    t = re.sub(r"\s*\(n\.\s*[mf]\.\s*(?:ou\s*[mf]\.\s*)?\)", "", t)
    t = re.sub(r"\s*\(adj\.\)", "", t)
    t = re.sub(r"\s*\(v\.\)", "", t)
    t = re.sub(r"\s*\([^)]{1,20}\)", "", t)
    return t.strip()


def extract_gender(t: str) -> str:
    """Extract grammatical gender from GDT term annotation."""
    m = re.search(r"\((n\.\s*[mf](?:\.\s*ou\s*[mf])?\.?)\)", t)
    if m:
        g = m.group(1).replace(" ", "")
        if "m.ouf" in g or "ouf" in g:
            return "masculine or feminine noun"
        elif "n.m" in g:
            return "masculine noun"
        elif "n.f" in g:
            return "feminine noun"
    if "(adj.)" in t:
        return "adjective"
    return ""


def process_fiches_csv(filepath: str, results: list, seen: set) -> int:
    """Process fiches terminologiques CSV (larger, 54K entries)."""
    with open(filepath, encoding="utf-8-sig", errors="replace") as f:
        reader = csv.DictReader(f)
        count = 0
        for row in reader:
            terms_fr_raw = (row.get("Termes_francais") or "").strip()
            terms_en_raw = (row.get("Termes_anglais") or "").strip()
            domain = (row.get("Domaines") or "").strip()
            definition = (row.get("Definition") or "").strip()

            if not terms_fr_raw:
                continue

            # Check domain and term relevance
            domain_ok = is_domain_relevant(domain)
            terms_fr_list = split_semicolons(terms_fr_raw)
            terms_en_list = split_semicolons(terms_en_raw)

            # Primary term
            primary_fr = clean_term(terms_fr_list[0]) if terms_fr_list else ""
            primary_en = clean_term(terms_en_list[0]) if terms_en_list else ""
            gender = extract_gender(terms_fr_list[0]) if terms_fr_list else ""

            if not domain_ok and not is_term_relevant(primary_en, primary_fr):
                continue

            # Determine domain tag
            domain_tag = "information technology"
            d_lower = domain.lower()
            if "sécuri" in d_lower and "logique" in d_lower:
                domain_tag = "information security"
            elif "cybersécuri" in d_lower:
                domain_tag = "cybersecurity"
            elif "défense" in d_lower:
                domain_tag = "national defence"
            elif "administrat" in d_lower:
                domain_tag = "public administration"
            elif "informati" in d_lower:
                domain_tag = "information technology"

            key = primary_fr.lower()
            if key not in seen:
                seen.add(key)
                notes = gender
                if domain:
                    notes = (notes + "; " if notes else "") + domain
                results.append((primary_en or primary_fr, primary_fr, domain_tag, "OQLF GDT", notes))
                count += 1

            # Add synonyms
            for i in range(1, max(len(terms_fr_list), len(terms_en_list))):
                fr = clean_term(terms_fr_list[i]) if i < len(terms_fr_list) else ""
                en = clean_term(terms_en_list[i]) if i < len(terms_en_list) else ""
                if not fr:
                    continue
                fr_key = fr.lower()
                if fr_key not in seen:
                    seen.add(fr_key)
                    gender_syn = extract_gender(terms_fr_list[i]) if i < len(terms_fr_list) else ""
                    notes = gender_syn
                    if primary_fr:
                        notes = (notes + "; " if notes else "") + f"synonym of: {primary_fr}"
                    results.append((en or fr, fr, domain_tag, "OQLF GDT", notes))
                    count += 1

        return count


def process_terms_csv(filepath: str, results: list, seen: set) -> int:
    """Process termes officialisés CSV (smaller, 1471 entries, officialised terms)."""
    with open(filepath, encoding="utf-8-sig", errors="replace") as f:
        reader = csv.DictReader(f)
        count = 0
        for row in reader:
            term_fr_raw = (row.get("Termes_Officialises") or "").strip()
            term_en = clean_term((row.get("Equivalent_anglais") or "").strip())
            domain = (row.get("Domaines") or "").strip()
            officialisation_type = (row.get("Type_Officialisation") or "").strip()

            if not term_fr_raw:
                continue

            term_fr = clean_term(term_fr_raw)
            gender = extract_gender(term_fr_raw)

            if not is_domain_relevant(domain) and not is_term_relevant(term_en, term_fr):
                continue

            domain_tag = "information technology"
            d_lower = domain.lower()
            if "sécuri" in d_lower:
                domain_tag = "information security"
            elif "administrat" in d_lower:
                domain_tag = "public administration"

            key = term_fr.lower()
            if key not in seen:
                seen.add(key)
                notes_parts = []
                if gender:
                    notes_parts.append(gender)
                if officialisation_type:
                    notes_parts.append(f"officialisation: {officialisation_type}")
                notes = "; ".join(notes_parts) if notes_parts else ""
                results.append((term_en or term_fr, term_fr, domain_tag, "OQLF GDT officialisé", notes))
                count += 1

        return count


if __name__ == "__main__":
    results = []
    seen = set()

    fiches_path = "/tmp/gdt-fiches.csv"
    terms_path = "/tmp/gdt-terms.csv"

    if len(sys.argv) > 1 and sys.argv[1] == "fiches":
        n = process_fiches_csv(fiches_path, results, seen)
        print(f"Fiches: {n} entries extracted", file=sys.stderr)
    elif len(sys.argv) > 1 and sys.argv[1] == "terms":
        n = process_terms_csv(terms_path, results, seen)
        print(f"Terms: {n} entries extracted", file=sys.stderr)
    else:
        # Process both
        n1 = process_fiches_csv(fiches_path, results, seen)
        print(f"Fiches terminologiques: {n1} entries extracted", file=sys.stderr)
        n2 = process_terms_csv(terms_path, results, seen)
        print(f"Termes officialisés: {n2} entries extracted", file=sys.stderr)

    # Sort by domain then term
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

    print(f"Written {len(results)} entries to {output_path}", file=sys.stderr)
    print(f"DONE: {len(results)} entries", file=sys.stdout)
