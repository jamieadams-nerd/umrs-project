#!/usr/bin/python3
"""
Fetch TERMIUM Plus specialized publication: Information Security Glossary
URL: https://www.btb.termiumplus.gc.ca/publications/securite-information-security-eng.html
Also fetches: Access to Information Glossary (Fascicle 1)

The page lists ~400 EN/FR term pairs from the Information Management glossaries.
Appends unique terms to termium-plus-fr_CA.tsv with source "Termium Plus Glossary".
"""
import csv
import io
import re
import ssl
import sys
import urllib.request

CTX = ssl.create_default_context()
CTX.check_hostname = False
CTX.verify_mode = ssl.CERT_NONE

GLOSSARY_URLS = [
    (
        "https://www.btb.termiumplus.gc.ca/publications/securite-information-security-eng.html",
        "Information Security Glossary",
        "information security",
    ),
]


def fetch(url, timeout=30):
    req = urllib.request.Request(
        url, headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) Firefox/120.0"}
    )
    with urllib.request.urlopen(req, context=CTX, timeout=timeout) as r:
        return r.read().decode("utf-8", "replace")


def parse_glossary_page(html: str) -> list:
    """
    Parse the TERMIUM Plus glossary HTML page.
    The page typically contains a table or definition list with EN/FR pairs.
    """
    # Remove scripts and styles
    html = re.sub(r"<script[^>]*>.*?</script>", "", html, flags=re.DOTALL | re.IGNORECASE)
    html = re.sub(r"<style[^>]*>.*?</style>", "", html, flags=re.DOTALL | re.IGNORECASE)

    # Find tables — TERMIUM glossaries are typically HTML tables
    tables = re.findall(r"<table[^>]*>(.*?)</table>", html, re.DOTALL | re.IGNORECASE)

    pairs = []

    for table in tables:
        rows = re.findall(r"<tr[^>]*>(.*?)</tr>", table, re.DOTALL | re.IGNORECASE)
        for row in rows:
            cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", row, re.DOTALL | re.IGNORECASE)
            if len(cells) >= 2:
                # Strip HTML tags from cells
                cleaned = []
                for c in cells[:3]:
                    text = re.sub(r"<[^>]+>", " ", c)
                    text = re.sub(r"\s+", " ", text).strip()
                    text = text.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&nbsp;", " ")
                    cleaned.append(text)
                if cleaned[0] and cleaned[1] and cleaned[0].lower() not in ("english", "anglais", "term", "terme", "en", "fr"):
                    pairs.append(cleaned)

    # Also try definition list pattern
    dts = re.findall(r"<dt[^>]*>(.*?)</dt>", html, re.DOTALL | re.IGNORECASE)
    dds = re.findall(r"<dd[^>]*>(.*?)</dd>", html, re.DOTALL | re.IGNORECASE)
    for dt, dd in zip(dts, dds):
        en = re.sub(r"<[^>]+>", " ", dt)
        en = re.sub(r"\s+", " ", en).strip()
        fr = re.sub(r"<[^>]+>", " ", dd)
        fr = re.sub(r"\s+", " ", fr).strip()
        if en and fr:
            pairs.append([en, fr])

    return pairs


def load_existing_seen(filepath: str) -> set:
    """Load already-seen english terms from existing TSV."""
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
    output_path = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"
    seen = load_existing_seen(output_path)
    print(f"Existing entries: {len(seen)}", file=sys.stderr)

    new_entries = []

    for url, glossary_name, domain_tag in GLOSSARY_URLS:
        print(f"\nFetching: {glossary_name}", file=sys.stderr)
        print(f"  URL: {url}", file=sys.stderr)
        try:
            html = fetch(url)
            print(f"  Downloaded: {len(html)} chars", file=sys.stderr)

            # Save raw HTML for inspection
            with open(f"/tmp/termium-glossary-{domain_tag.replace(' ', '-')}.html", "w") as f:
                f.write(html)

            pairs = parse_glossary_page(html)
            print(f"  Parsed pairs: {len(pairs)}", file=sys.stderr)

            # Show first few
            for p in pairs[:5]:
                print(f"  SAMPLE: {p}", file=sys.stderr)

            added = 0
            for pair in pairs:
                if len(pair) < 2:
                    continue
                en = pair[0].strip()
                fr = pair[1].strip()
                notes = pair[2].strip() if len(pair) > 2 else ""
                if not en or not fr:
                    continue
                en_key = en.lower()
                if en_key not in seen:
                    seen.add(en_key)
                    new_entries.append((en, fr, domain_tag, f"Termium Plus — {glossary_name}", notes))
                    added += 1
            print(f"  Added: {added} new entries", file=sys.stderr)

        except Exception as e:
            print(f"  ERROR: {e}", file=sys.stderr)

    # Append to TSV
    if new_entries:
        with open(output_path, "a", encoding="utf-8", newline="") as out:
            for row in sorted(new_entries, key=lambda r: (r[2], r[0].lower())):
                sanitized = tuple(
                    str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                    for f in row
                )
                out.write("\t".join(sanitized) + "\n")
        print(f"\nAppended {len(new_entries)} entries to {output_path}", file=sys.stderr)

    # Print raw HTML snippet for manual inspection
    print("DONE", file=sys.stdout)
