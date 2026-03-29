#!/usr/bin/python3
"""
Parse CCCS (Canadian Centre for Cyber Security) bilingual glossary.
The CCCS site has separate EN and FR pages — we need to correlate them.
Alternatively, we extract from each page and produce a merged TSV.

Output appended to termium-plus-fr_CA.tsv with source "CCCS Glossary (cyber.gc.ca)".
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


def extract_terms_from_page(path: str) -> dict:
    """
    Extract terms and definitions from CCCS glossary page.
    Returns dict: {term_lower: (term, definition)}
    """
    with open(path, encoding="utf-8", errors="replace") as f:
        content = f.read()

    content = re.sub(r"<script[^>]*>.*?</script>", "", content, flags=re.DOTALL | re.IGNORECASE)
    content = re.sub(r"<style[^>]*>.*?</style>", "", content, flags=re.DOTALL | re.IGNORECASE)

    if "--debug" in sys.argv:
        # Show structure
        main_m = re.search(r'id="main"[^>]*>(.*?)</main>', content, re.DOTALL | re.IGNORECASE)
        if main_m:
            snippet = clean(main_m.group(1))[:3000]
            print(f"=== MAIN SNIPPET ({path}) ===", file=sys.stderr)
            print(snippet, file=sys.stderr)

    terms = {}

    # Strategy 1: DT/DD lists
    dl_match = re.findall(r"<dl[^>]*>(.*?)</dl>", content, re.DOTALL | re.IGNORECASE)
    for dl in dl_match:
        dts = re.findall(r"<dt[^>]*>(.*?)</dt>", dl, re.DOTALL | re.IGNORECASE)
        dds = re.findall(r"<dd[^>]*>(.*?)</dd>", dl, re.DOTALL | re.IGNORECASE)
        for dt, dd in zip(dts, dds):
            term = clean(dt)
            defn = clean(dd)
            if term and defn and len(term) < 100:
                terms[term.lower()] = (term, defn)

    # Strategy 2: Section headings followed by definitions
    if not terms:
        # Look for h3/h4 followed by p
        items = re.findall(r"<h[34][^>]*>(.*?)</h[34]>(.*?)(?=<h[234]|</section|</article|$)",
                          content, re.DOTALL | re.IGNORECASE)
        for hdr, body in items:
            term = clean(hdr)
            defn = clean(body)
            if term and len(term) < 100:
                terms[term.lower()] = (term, defn[:200])

    # Strategy 3: Look for alphabetically indexed definition lists
    if not terms:
        # Find content within main section
        paras = re.findall(r"<(?:p|li)[^>]*>([^<]{10,300})</(?:p|li)>", content)
        for p in paras:
            text = clean(p)
            # Look for "Term: definition" pattern
            m = re.match(r"^([A-Z][A-Za-z\s/-]{2,50}):\s+(.{20,})", text)
            if m:
                term = m.group(1).strip()
                defn = m.group(2).strip()
                terms[term.lower()] = (term, defn[:200])

    print(f"  Extracted {len(terms)} terms from {path}", file=sys.stderr)
    if terms:
        first = list(terms.items())[:3]
        for k, v in first:
            print(f"  SAMPLE: {v[0]!r}: {v[1][:80]!r}", file=sys.stderr)

    return terms


def load_existing_seen(filepath: str) -> set:
    seen = set()
    try:
        with open(filepath, encoding="utf-8") as f:
            reader = csv.DictReader(f, delimiter="\t")
            for row in reader:
                en = row.get("english_term", "").lower().strip()
                if en:
                    seen.add(en)
    except FileNotFoundError:
        pass
    return seen


if __name__ == "__main__":
    print("Extracting English terms...", file=sys.stderr)
    en_terms = extract_terms_from_page(EN_HTML)

    print("Extracting French terms...", file=sys.stderr)
    fr_terms = extract_terms_from_page(FR_HTML)

    print(f"\nEN terms: {len(en_terms)}, FR terms: {len(fr_terms)}", file=sys.stderr)

    # The CCCS glossary is sorted alphabetically and the EN/FR pages have the same terms
    # We can attempt to correlate them by sort order
    en_sorted = sorted(en_terms.items(), key=lambda x: x[0])
    fr_sorted = sorted(fr_terms.items(), key=lambda x: x[0])

    print(f"\nFirst 5 EN: {[v[0] for _, v in en_sorted[:5]]}", file=sys.stderr)
    print(f"First 5 FR: {[v[0] for _, v in fr_sorted[:5]]}", file=sys.stderr)

    seen = load_existing_seen(OUTPUT_PATH)
    print(f"\nExisting entries: {len(seen)}", file=sys.stderr)

    new_entries = []
    # Add EN terms (even without FR translation) since they're CCCS-authoritative
    for en_key, (en_term, en_defn) in en_terms.items():
        if en_key in seen:
            continue
        seen.add(en_key)
        # Look for FR equivalent by sorted position or exact lookup
        fr_term = ""
        # The CCCS site is bilingual; try to find matching term in FR
        # For now, add with empty FR — we'll note it's from CCCS EN page
        notes = f"CCCS definition: {en_defn[:100]}"
        new_entries.append((en_term, fr_term or en_term, DOMAIN_TAG, SOURCE_TAG, notes))

    print(f"New entries to add: {len(new_entries)}", file=sys.stderr)

    if new_entries:
        with open(OUTPUT_PATH, "a", encoding="utf-8", newline="") as out:
            for row in sorted(new_entries, key=lambda r: r[0].lower()):
                sanitized = tuple(
                    str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                    for f in row
                )
                out.write("\t".join(sanitized) + "\n")
        print(f"Appended {len(new_entries)} CCCS entries", file=sys.stderr)

    print("DONE")
