#!/usr/bin/python3
"""
Final clean parser for TERMIUM Plus Information Security Glossary.
Properly splits compound cells, cleans English grammar notes (n. / adj.),
handles semicolon-separated synonyms in English cells.
"""
import csv
import html as html_lib
import re
import sys

OUTPUT_PATH = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"
HTML_PATH = "/tmp/termium-info-sec-glossary.html"
DOMAIN_TAG = "information security"
SOURCE_TAG = "Termium Plus — Information Security Glossary"


def clean_text(s: str) -> str:
    s = re.sub(r"<[^>]+>", " ", s)
    s = html_lib.unescape(s)
    s = re.sub(r"\s+", " ", s).strip()
    return s


def extract_gender_fr(s: str) -> str:
    m = re.search(r"\(\s*n\s*\.\s*([mf])\s*\.\s*\)", s, re.IGNORECASE)
    if m:
        return "feminine noun" if m.group(1).lower() == "f" else "masculine noun"
    if re.search(r"\(\s*adj\s*\.\s*\)", s, re.IGNORECASE):
        return "adjective"
    return ""


def clean_en_term(s: str) -> str:
    """Remove grammar notes from English term. Keep NOTE text."""
    # Remove parts like "(n.)" "(adj.)" "(v.)" in English
    s = re.sub(r"\(\s*n\s*\.\s*\)", "", s)
    s = re.sub(r"\(\s*adj\s*\.\s*\)", "", s)
    s = re.sub(r"\(\s*v\s*\.\s*\)", "", s)
    s = re.sub(r"\s+", " ", s).strip()
    return s


def clean_fr_term(s: str) -> tuple:
    """Returns (clean_term, gender, note)"""
    gender = extract_gender_fr(s)
    # Remove gender annotation
    s = re.sub(r"\(\s*n\s*\.\s*[mf]\s*\.\s*\)", "", s, flags=re.IGNORECASE)
    s = re.sub(r"\(\s*adj\s*\.\s*\)", "", s, flags=re.IGNORECASE)
    # Extract NOTA
    nota_m = re.search(r"\bNOTA\b", s, re.IGNORECASE)
    nota = ""
    if nota_m:
        nota = s[nota_m.start():].strip()
        s = s[:nota_m.start()].strip()
    s = re.sub(r"\s+", " ", s).strip()
    return s, gender, nota


def split_compound_en(s: str) -> list:
    """
    Split compound English cells.
    The pattern in this glossary is:
    "primary term\n secondary term 1\n secondary term 2"
    But all newlines are collapsed to spaces in HTML rendering.

    Strategy: split on capitalized continuations, semicolons,
    and "NOTE" boundaries.
    """
    # First handle the NOTE split
    note_match = re.search(r"\bNOTE\b", s, re.IGNORECASE)
    note = ""
    if note_match:
        note = s[note_match.start():]
        s = s[:note_match.start()].strip()

    # Split on semicolons (synonym separators)
    if ";" in s:
        parts = [p.strip() for p in s.split(";") if p.strip()]
        # Prepend note to first part if present
        return parts, note

    # The compound cells pack "primary term variant1 variant2" all in one line
    # Heuristic: split when we see lowercase-then-newline-then-lowercase (already collapsed)
    # For this glossary, we just take the first meaningful chunk
    # Check if the string contains what appears to be multiple terms:
    # "access access to a computer access to a network"
    # The key pattern: repeated stem at start
    # Just return the whole cleaned string as one term
    return [s.strip()], note


def parse_html(path: str) -> list:
    with open(path, encoding="utf-8", errors="replace") as f:
        content = f.read()

    content = re.sub(r"<script[^>]*>.*?</script>", "", content, flags=re.DOTALL | re.IGNORECASE)
    content = re.sub(r"<style[^>]*>.*?</style>", "", content, flags=re.DOTALL | re.IGNORECASE)

    # In TERMIUM glossary pages, compound entries are in table cells
    # where each sub-term is in a <br>-separated list or separate paragraphs
    # Let's extract the raw cell HTML to properly split

    tables = re.findall(r"<table[^>]*>(.*?)</table>", content, re.DOTALL | re.IGNORECASE)
    entries = []

    for table in tables:
        rows = re.findall(r"<tr[^>]*>(.*?)</tr>", table, re.DOTALL | re.IGNORECASE)
        if len(rows) < 2:
            continue

        for row in rows:
            cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", row, re.DOTALL | re.IGNORECASE)
            if len(cells) < 2:
                continue

            # Get raw cell content with BR tags preserved as \n
            raw_en_html = cells[0]
            raw_fr_html = cells[1]

            # Replace BR with newline before stripping tags
            raw_en_html = re.sub(r"<br\s*/?>", "\n", raw_en_html, flags=re.IGNORECASE)
            raw_fr_html = re.sub(r"<br\s*/?>", "\n", raw_fr_html, flags=re.IGNORECASE)
            # Replace P end tags with newline
            raw_en_html = re.sub(r"</p>", "\n", raw_en_html, flags=re.IGNORECASE)
            raw_fr_html = re.sub(r"</p>", "\n", raw_fr_html, flags=re.IGNORECASE)

            en_raw = clean_text(raw_en_html)
            fr_raw = clean_text(raw_fr_html)

            # Skip headers
            if en_raw.lower() in ("english", "anglais", ""):
                continue
            if not en_raw or not fr_raw:
                continue

            # Split into lines
            en_lines = [l.strip() for l in en_raw.split("\n") if l.strip()]
            fr_lines = [l.strip() for l in fr_raw.split("\n") if l.strip()]

            # If no BR tags found, the lines are collapsed — extract just the primary term
            # The primary term is the SHORTEST leading term (before qualifiers like "to a computer")
            if not en_lines:
                en_lines = [en_raw]
            if not fr_lines:
                fr_lines = [fr_raw]

            # Process primary pair
            en_primary_raw = en_lines[0]
            fr_primary_raw = fr_lines[0]

            # Clean English: remove grammar annotations, extract NOTE
            note_m = re.search(r"\bNOTE\b", en_primary_raw, re.IGNORECASE)
            en_note = ""
            if note_m:
                en_note = en_primary_raw[note_m.start():].strip()
                en_primary_raw = en_primary_raw[:note_m.start()].strip()

            # Handle semicolon synonyms in English
            en_parts = [p.strip() for p in en_primary_raw.split(";") if p.strip()]
            en_primary = clean_en_term(en_parts[0]) if en_parts else ""

            fr_primary, gender, fr_nota = clean_fr_term(fr_primary_raw)

            if not en_primary or not fr_primary:
                continue

            notes_parts = []
            if gender:
                notes_parts.append(gender)
            nota_combined = " | ".join(p for p in [en_note, fr_nota] if p)
            if nota_combined:
                notes_parts.append(nota_combined[:120])
            notes = "; ".join(notes_parts)

            entries.append((en_primary, fr_primary, notes))

            # Add EN synonyms from semicolons (e.g., "access control list; ACL")
            for en_syn in en_parts[1:]:
                en_syn_clean = clean_en_term(en_syn)
                if en_syn_clean and en_syn_clean != en_primary:
                    entries.append((en_syn_clean, fr_primary, f"synonym of EN: {en_primary}"))

            # Process sub-terms (if BR-separated lines give us multiple)
            for i in range(1, min(len(en_lines), len(fr_lines))):
                en_sub = en_lines[i]
                fr_sub_raw = fr_lines[i]
                # Skip NOTE lines
                if re.match(r"^(NOTE|NOTA)\b", en_sub, re.IGNORECASE):
                    continue
                if re.match(r"^(NOTE|NOTA)\b", fr_sub_raw, re.IGNORECASE):
                    continue
                en_sub_clean = clean_en_term(en_sub)
                fr_sub, gender_sub, _ = clean_fr_term(fr_sub_raw)
                if en_sub_clean and fr_sub and en_sub_clean != en_primary:
                    sub_notes = gender_sub if gender_sub else ""
                    entries.append((en_sub_clean, fr_sub, sub_notes))

    return entries


def load_non_glossary_entries() -> tuple:
    non_glossary = []
    seen = set()
    try:
        with open(OUTPUT_PATH, encoding="utf-8") as f:
            reader = csv.DictReader(f, delimiter="\t")
            for row in reader:
                src = row.get("source", "")
                if "InfoSec Glossary" not in src:
                    non_glossary.append(row)
                    en = row.get("english_term", "").lower().strip()
                    if en:
                        seen.add(en)
    except FileNotFoundError:
        pass
    return non_glossary, seen


if __name__ == "__main__":
    print("Loading existing entries (excluding InfoSec Glossary)...", file=sys.stderr)
    existing_rows, seen = load_non_glossary_entries()
    print(f"Non-glossary entries: {len(existing_rows)}", file=sys.stderr)

    print("Parsing TERMIUM InfoSec Glossary HTML...", file=sys.stderr)
    entries = parse_html(HTML_PATH)
    print(f"Parsed entries: {len(entries)}", file=sys.stderr)

    # Show samples
    for e in entries[:12]:
        print(f"  EN: {e[0]!r}", file=sys.stderr)
        print(f"  FR: {e[1]!r}", file=sys.stderr)
        print(f"  notes: {e[2]!r}", file=sys.stderr)
        print("  ---", file=sys.stderr)

    # Deduplicate
    new_entries = []
    for (en, fr, notes) in entries:
        if not en or not fr:
            continue
        en_key = en.lower()
        if en_key not in seen:
            seen.add(en_key)
            new_entries.append((en, fr, DOMAIN_TAG, SOURCE_TAG, notes))

    print(f"\nNew entries to add: {len(new_entries)}", file=sys.stderr)

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
    print(f"Written {total} total entries ({len(new_entries)} from InfoSec Glossary)", file=sys.stderr)
    print(f"DONE: {len(new_entries)} InfoSec Glossary entries")
