#!/usr/bin/python3
"""Probe Termium Plus subject ZIP URLs - helper script for Librarian."""
import ssl
import urllib.request
import re
import zipfile
import io
import sys

ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE


def fetch(url):
    req = urllib.request.Request(
        url, headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) Firefox/120.0"}
    )
    with urllib.request.urlopen(req, context=ctx, timeout=30) as r:
        return r.read()


def head_size(url):
    req = urllib.request.Request(
        url,
        method="HEAD",
        headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) Firefox/120.0"},
    )
    try:
        with urllib.request.urlopen(req, context=ctx, timeout=10) as r:
            return r.getheader("Content-Length", "?"), r.status
    except Exception as e:
        return str(e), 0


if len(sys.argv) > 1 and sys.argv[1] == "probe":
    # Probe candidate URLs for Military/Security subject
    candidates = [
        "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-MilitaireSecurite-subject-MilitaryAndSecurity.zip",
        "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-MilitarySecurity-subject-MilitarySecurity.zip",
        "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-MiliArme-subject-MilAndArms.zip",
        "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-Securite-subject-Security.zip",
        "https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-Militaire-subject-Military.zip",
    ]
    for url in candidates:
        size, status = head_size(url)
        print(f"{status} {size:>12}  {url}")

elif len(sys.argv) > 1 and sys.argv[1] == "support":
    # List all ZIPs from support page
    data = fetch("https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/soutien-support-eng.html").decode("utf-8", "replace")
    zips = sorted(set(re.findall(r"href=\"([^\"]+\.zip)\"", data)))
    for z in zips:
        print(z)

elif len(sys.argv) > 1 and sys.argv[1] == "inspect":
    # Inspect a local ZIP
    path = sys.argv[2]
    z = zipfile.ZipFile(path)
    for name in z.namelist():
        print(name)
    with z.open(z.namelist()[0]) as f:
        sample = f.read(500).decode("utf-8-sig", "replace")
    print("=== SAMPLE ===")
    print(sample)

else:
    print("Usage: python3 probe_termium.py [probe|support|inspect <path>]")
