#!/usr/bin/python3
"""
Clean parser for TERMIUM Plus Information Security Glossary.
Handles HTML entities, splits compound entries, and produces clean TSV rows.

This replaces the entries added by parse_termium_glossary.py —
run this after removing old InfoSec Glossary entries first.
"""
import csv
import html
import re
import sys

OUTPUT_PATH = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"
HTML_PATH = "/tmp/termium-info-sec-glossary.html"
DOMAIN_TAG = "information security"
SOURCE_TAG = "Termium Plus — Information Security Glossary"


def clean_html_text(s: str) -> str:
    """Strip tags, decode HTML entities, normalize whitespace."""
    # Remove tags
    s = re.sub(r"<[^>]+>", " ", s)
    # Decode HTML entities
    s = html.unescape(s)
    # Normalize whitespace
    s = re.sub(r"\s+", " ", s).strip()
    return s


def extract_gender_fr(s: str) -> str:
    """Extract gender from French term annotation like (n.f.) (n.m.)"""
    m = re.search(r"\(\s*n\s*\.\s*([mf])\s*\.\s*\)", s, re.IGNORECASE)
    if m:
        g = m.group(1).lower()
        return "feminine noun" if g == "f" else "masculine noun"
    return ""


def clean_fr_term(s: str) -> tuple:
    """
    Clean a French term cell.
    Returns (primary_term, gender, notes)
    """
    # Extract gender before cleaning
    gender = extract_gender_fr(s)

    # Remove grammar annotations
    cleaned = re.sub(r"\(\s*n\s*\.\s*[mf]\s*\.\s*\)", "", s, flags=re.IGNORECASE)
    # Remove NOTE/NOTA lines
    nota_match = re.search(r"\b(NOTE|NOTA)\b", cleaned, re.IGNORECASE)
    nota = ""
    if nota_match:
        nota = cleaned[nota_match.start():].strip()
        cleaned = cleaned[:nota_match.start()].strip()

    return cleaned.strip(), gender, nota


def split_compound_cell(s: str) -> list:
    """
    Split a compound cell that contains multiple terms on separate lines.
    E.g., "access\naccess to a computer\naccess to a network" -> ["access", "access to a computer", ...]
    """
    # The compound cells in the glossary pack multiple terms into one cell
    # separated by whitespace/newlines within the HTML rendering
    # We look for the pattern: primary term followed by more specific terms
    # For simplicity, keep only the first (primary) term
    lines = [l.strip() for l in s.split("\n") if l.strip()]
    if lines:
        return [lines[0]]
    return [s.strip()] if s.strip() else []


def parse_html(path: str) -> list:
    with open(path, encoding="utf-8", errors="replace") as f:
        html_content = f.read()

    # Remove scripts/styles
    html_content = re.sub(r"<script[^>]*>.*?</script>", "", html_content, flags=re.DOTALL | re.IGNORECASE)
    html_content = re.sub(r"<style[^>]*>.*?</style>", "", html_content, flags=re.DOTALL | re.IGNORECASE)

    # Find all EN/FR table pairs
    tables = re.findall(r"<table[^>]*>(.*?)</table>", html_content, re.DOTALL | re.IGNORECASE)

    entries = []  # (en_term, fr_term, gender, notes)

    for table in tables:
        rows = re.findall(r"<tr[^>]*>(.*?)</tr>", table, re.DOTALL | re.IGNORECASE)
        if len(rows) < 2:
            continue

        for row in rows:
            cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", row, re.DOTALL | re.IGNORECASE)
            if len(cells) < 2:
                continue

            raw_en = clean_html_text(cells[0])
            raw_fr = clean_html_text(cells[1])

            # Skip header rows
            if raw_en.lower() in ("english", "anglais", "term", "terme", "en", "fr", ""):
                continue
            if not raw_en or not raw_fr:
                continue

            # For compound cells, we want to extract primary term + its variants
            # Pattern: cells contain "primary term\n variant 1\n variant 2"
            en_lines = [l.strip() for l in raw_en.split("\n") if l.strip() and len(l.strip()) > 1]
            fr_lines = [l.strip() for l in raw_fr.split("\n") if l.strip() and len(l.strip()) > 1]

            # Primary term = first non-empty line
            en_primary = en_lines[0] if en_lines else ""
            fr_primary_raw = fr_lines[0] if fr_lines else ""

            if not en_primary or not fr_primary_raw:
                continue

            # Extract NOTE from FR primary
            fr_primary_clean, gender, nota = clean_fr_term(fr_primary_raw)
            fr_primary_clean = fr_primary_clean.strip()

            if not fr_primary_clean:
                continue

            notes_parts = []
            if gender:
                notes_parts.append(gender)
            if nota:
                notes_parts.append(nota[:100])
            notes = "; ".join(notes_parts)

            entries.append((en_primary, fr_primary_clean, notes))

            # Also add secondary terms if they are UMRS-relevant
            for i in range(1, min(len(en_lines), len(fr_lines))):
                en_sec = en_lines[i]
                fr_sec_raw = fr_lines[i]
                # Skip NOTE/NOTA continuations
                if re.match(r"^(NOTE|NOTA)\b", fr_sec_raw, re.IGNORECASE):
                    continue
                fr_sec_clean, gender_sec, _ = clean_fr_term(fr_sec_raw)
                if en_sec and fr_sec_clean and en_sec != en_primary:
                    syn_notes = gender_sec if gender_sec else ""
                    entries.append((en_sec, fr_sec_clean, syn_notes))

    return entries


def load_non_glossary_entries() -> tuple:
    """Load existing TSV entries that are NOT from the InfoSec Glossary."""
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
    print("Loading existing entries (excluding old InfoSec Glossary entries)...", file=sys.stderr)
    existing_rows, seen = load_non_glossary_entries()
    print(f"Non-glossary entries: {len(existing_rows)}", file=sys.stderr)

    print("Parsing TERMIUM InfoSec Glossary HTML...", file=sys.stderr)
    entries = parse_html(HTML_PATH)
    print(f"Parsed entries: {len(entries)}", file=sys.stderr)

    # Show samples
    for e in entries[:8]:
        print(f"  {e}", file=sys.stderr)

    # Deduplicate against existing
    new_entries = []
    for (en, fr, notes) in entries:
        if not en or not fr:
            continue
        en_key = en.lower()
        if en_key not in seen:
            seen.add(en_key)
            new_entries.append((en, fr, DOMAIN_TAG, SOURCE_TAG, notes))

    print(f"New entries to add: {len(new_entries)}", file=sys.stderr)

    # Rewrite TSV: existing non-glossary entries + new glossary entries
    all_new = sorted(new_entries, key=lambda r: (r[2], r[0].lower()))

    with open(OUTPUT_PATH, "w", encoding="utf-8", newline="") as out:
        out.write("english_term\tfr_ca_term\tdomain\tsource\tnotes\n")
        # Write existing non-glossary entries
        for row in existing_rows:
            sanitized = tuple(
                str(row.get(k, "")).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for k in ["english_term", "fr_ca_term", "domain", "source", "notes"]
            )
            out.write("\t".join(sanitized) + "\n")
        # Append new glossary entries
        for row in all_new:
            sanitized = tuple(
                str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                for f in row
            )
            out.write("\t".join(sanitized) + "\n")

    total = len(existing_rows) + len(new_entries)
    print(f"Written {total} total entries to {OUTPUT_PATH}", file=sys.stderr)
    print(f"DONE: {len(new_entries)} new InfoSec Glossary entries added")
