#!/usr/bin/python3
"""
Parse the TERMIUM Plus Information Security Glossary from local HTML file.
Appends unique terms to termium-plus-fr_CA.tsv
"""
import csv
import re
import sys


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


def strip_tags(s: str) -> str:
    s = re.sub(r"<[^>]+>", " ", s)
    s = re.sub(r"\s+", " ", s)
    s = s.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&nbsp;", " ").replace("&#160;", " ")
    return s.strip()


def parse_html(path: str) -> list:
    with open(path, encoding="utf-8", errors="replace") as f:
        html = f.read()

    # Remove scripts/styles
    html = re.sub(r"<script[^>]*>.*?</script>", "", html, flags=re.DOTALL | re.IGNORECASE)
    html = re.sub(r"<style[^>]*>.*?</style>", "", html, flags=re.DOTALL | re.IGNORECASE)

    # Print a portion of the page to understand structure
    if "--debug" in sys.argv:
        # Find main content area
        main_idx = html.lower().find("content-main")
        if main_idx < 0:
            main_idx = html.lower().find("main-content")
        if main_idx < 0:
            main_idx = html.lower().find("<main")
        if main_idx >= 0:
            print("=== MAIN CONTENT SNIPPET ===", file=sys.stderr)
            snippet = html[main_idx:main_idx+5000]
            print(strip_tags(snippet[:3000]), file=sys.stderr)
        else:
            print("No main content found, showing body snippet:", file=sys.stderr)
            body_idx = html.lower().find("<body")
            if body_idx >= 0:
                print(strip_tags(html[body_idx:body_idx+3000]), file=sys.stderr)

    pairs = []

    # Strategy 1: Look for a table with EN/FR columns
    tables = re.findall(r"<table[^>]*>(.*?)</table>", html, re.DOTALL | re.IGNORECASE)
    print(f"Found {len(tables)} tables", file=sys.stderr)

    for ti, table in enumerate(tables):
        rows = re.findall(r"<tr[^>]*>(.*?)</tr>", table, re.DOTALL | re.IGNORECASE)
        if len(rows) < 3:
            continue
        # Check if first row is header with EN/FR
        header_cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", rows[0], re.DOTALL | re.IGNORECASE)
        header_text = " ".join(strip_tags(c).lower() for c in header_cells)
        if not ("english" in header_text or "anglais" in header_text or "english" in header_text or "term" in header_text):
            # try with first data row
            pass

        print(f"Table {ti}: {len(rows)} rows, header: {header_text[:80]}", file=sys.stderr)

        # Collect data rows
        for row in rows[1:]:
            cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", row, re.DOTALL | re.IGNORECASE)
            if len(cells) >= 2:
                c0 = strip_tags(cells[0])
                c1 = strip_tags(cells[1])
                c2 = strip_tags(cells[2]) if len(cells) > 2 else ""
                if c0 and c1 and c0.lower() not in ("english", "anglais", "term", "terme", ""):
                    pairs.append((c0, c1, c2))

    # Strategy 2: Look for definition lists
    if not pairs:
        dts = re.findall(r"<dt[^>]*>(.*?)</dt>", html, re.DOTALL | re.IGNORECASE)
        dds = re.findall(r"<dd[^>]*>(.*?)</dd>", html, re.DOTALL | re.IGNORECASE)
        print(f"DT/DD pairs: {len(dts)}/{len(dds)}", file=sys.stderr)
        for dt, dd in zip(dts, dds):
            en = strip_tags(dt)
            fr = strip_tags(dd)
            if en and fr:
                pairs.append((en, fr, ""))

    # Strategy 3: Look for a list where each item has EN: FR: pattern
    if not pairs:
        # Find all paragraph/li content
        items = re.findall(r"<(?:li|p)[^>]*>(.*?)</(?:li|p)>", html, re.DOTALL | re.IGNORECASE)
        for item in items:
            text = strip_tags(item)
            # Look for patterns like "English term / Terme français"
            m = re.match(r"^([A-Za-z][^/\|]{3,60}?)\s*[/|]\s*(.{3,80}?)$", text)
            if m:
                pairs.append((m.group(1).strip(), m.group(2).strip(), ""))

    print(f"Total pairs found: {len(pairs)}", file=sys.stderr)
    if pairs:
        print(f"Sample pairs:", file=sys.stderr)
        for p in pairs[:5]:
            print(f"  {p}", file=sys.stderr)

    return pairs


if __name__ == "__main__":
    html_path = "/tmp/termium-info-sec-glossary.html"
    output_path = "/media/psf/repos/umrs-project/.claude/corpus/termium-plus-fr_CA.tsv"

    seen = load_existing_seen(output_path)
    print(f"Existing entries: {len(seen)}", file=sys.stderr)

    pairs = parse_html(html_path)

    new_entries = []
    for (en, fr, notes) in pairs:
        if not en or not fr:
            continue
        en_key = en.lower()
        if en_key not in seen:
            seen.add(en_key)
            new_entries.append((en, fr, "information security", "Termium Plus — InfoSec Glossary", notes))

    print(f"New entries to add: {len(new_entries)}", file=sys.stderr)

    if new_entries:
        with open(output_path, "a", encoding="utf-8", newline="") as out:
            for row in sorted(new_entries, key=lambda r: r[0].lower()):
                sanitized = tuple(
                    str(f).replace("\t", " ").replace("\n", " ").replace("\r", "")
                    for f in row
                )
                out.write("\t".join(sanitized) + "\n")
        print(f"Appended {len(new_entries)} entries", file=sys.stderr)

    print("DONE")
