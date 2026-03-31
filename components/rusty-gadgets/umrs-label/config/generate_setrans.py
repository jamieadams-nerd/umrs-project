#!/usr/bin/env python3
"""
Generate TARGETED-setrans.conf and MLS-setrans.conf from UMRS label catalogs.

Source files:
  us/US-CUI-LABELS.json    — United States CUI categories and dissemination controls
  ca/CANADIAN-PROTECTED.json — Canadian Protected A/B/C tiers
  LEVELS.json               — Shared sensitivity level definitions

Output files:
  TARGETED-setrans.conf     — All markings at s0 (targeted policy, no MLS enforcement)
  MLS-setrans.conf          — US CUI at s1, Canadian PA/PB/PC at s1/s2/s3

Usage:
  python3 generate_setrans.py
"""

import json
import sys
from collections import defaultdict, OrderedDict
from datetime import date
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent

# NARA index group ordering (matches the 18 active groups in the UMRS catalog)
INDEX_GROUP_ORDER = [
    "Critical Infrastructure",
    "Defense",
    "Export Control",
    "Financial",
    "Immigration",
    "Intelligence",
    "International Agreements",
    "Law Enforcement",
    "Legal",
    "Natural and Cultural Resources",
    "Nuclear",
    "Patent",
    "Privacy",
    "Procurement and Acquisition",
    "Proprietary Business Information",
    "Statistical",
    "Tax",
    "Transportation",
]

# Short names for section headers
GROUP_ABBREV = {
    "Critical Infrastructure": "CRIT",
    "Defense": "DEF",
    "Export Control": "EXPT",
    "Financial": "FNC",
    "Immigration": "IMMG",
    "Intelligence": "INTEL",
    "International Agreements": "INTL",
    "Law Enforcement": "LEI",
    "Legal": "LGL",
    "Natural and Cultural Resources": "NCR",
    "Nuclear": "NUC",
    "Patent": "PAT",
    "Privacy": "PRIV",
    "Procurement and Acquisition": "PROCURE",
    "Proprietary Business Information": "PROPIN",
    "Statistical": "STAT",
    "Tax": "TAX",
    "Transportation": "TRANS",
}

# MCS base category number for each index group.
# Designed with headroom for future additions within each group.
GROUP_BASE = {
    "Critical Infrastructure": 1,
    "Defense": 15,
    "Export Control": 25,
    "Financial": 30,
    "Immigration": 45,
    "Intelligence": 55,
    "International Agreements": 68,
    "Law Enforcement": 70,
    "Legal": 95,
    "Natural and Cultural Resources": 115,
    "Nuclear": 120,
    "Patent": 130,
    "Privacy": 135,
    "Procurement and Acquisition": 155,
    "Proprietary Business Information": 160,
    "Statistical": 170,
    "Tax": 178,
    "Transportation": 185,
}

# LDC (Limited Dissemination Control) MCS allocation — c250-c265
# Sorted alphabetically by banner_marking for setrans.conf output.
LDC_MCS = {
    "Attorney-Client": 250,
    "Attorney-WP": 251,
    "DISPLAY ONLY": 252,
    "DL ONLY": 253,
    "FED ONLY": 254,
    "FEDCON": 255,
    "NOCON": 256,
    "NOFORN": 257,
    "REL TO": 258,
    "RELIDO": 259,
}

# DoD Distribution Statements — c266-c270
# These are LDC-equivalent tags for DoD technical data (SP-CTI).
# They do NOT appear in the CUI banner; they appear in the
# CUI Designation Indicator Block on the document.
# Tracked here for MCS category completeness.
DIST_STMT_MCS = {
    "DIST-B": (266, "Distribution Statement B — Limited to US Govt + contractors"),
    "DIST-C": (267, "Distribution Statement C — Limited to US Govt + contractors w/ need"),
    "DIST-D": (268, "Distribution Statement D — Limited to DoD + US DoD contractors"),
    "DIST-E": (269, "Distribution Statement E — Limited to DoD"),
    "DIST-F": (270, "Distribution Statement F — Further dissemination only as directed"),
}

# Canadian MCS numbers (from CANADIAN-PROTECTED.json category_base fields)
CA_MCS = {
    "PROTECTED-A": 300,
    "PROTECTED-B": 301,
    "PROTECTED-C": 302,
}


def sort_key(marking_key: str) -> tuple:
    """Sort by abbreviation ignoring SP- prefix. Basic (0) before specified (1)."""
    if marking_key.startswith("CUI//SP-"):
        return (marking_key[8:], 1)
    elif marking_key.startswith("CUI//"):
        return (marking_key[5:], 0)
    return (marking_key, 0)


def load_json(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def build_allocation(markings: dict) -> tuple:
    """
    Group markings by index_group, sort within each group, assign MCS numbers.
    Returns (allocation dict {marking_key: mcs_number}, grouped list for output).
    """
    groups = defaultdict(list)
    for key, entry in markings.items():
        if key == "CUI":
            continue  # handled separately as c0
        ig = entry.get("index_group")
        if ig:
            groups[ig].append((key, entry))

    for ig in groups:
        groups[ig].sort(key=lambda x: sort_key(x[0]))

    allocation = OrderedDict()
    allocation["CUI"] = 0
    group_ranges = {}

    for ig in INDEX_GROUP_ORDER:
        if ig not in groups:
            continue
        base = GROUP_BASE[ig]
        cat_num = base
        for key, entry in groups[ig]:
            allocation[key] = cat_num
            cat_num += 1
        group_ranges[ig] = (base, cat_num - 1)

    return allocation, groups, group_ranges


def build_block_map(group_ranges: dict) -> str:
    """Build the block allocation comment block for the file header."""
    lines = []
    lines.append("#   c0        US CUI base umbrella")
    for ig in INDEX_GROUP_ORDER:
        if ig not in group_ranges:
            continue
        lo, hi = group_ranges[ig]
        abbrev = GROUP_ABBREV[ig]
        count = hi - lo + 1
        if lo == hi:
            rng = f"c{lo}"
            lines.append(f"#   {rng:<10s}  {abbrev} — {ig} ({count})")
        else:
            rng = f"c{lo}-c{hi}"
            lines.append(f"#   {rng:<10s}  {abbrev} — {ig} ({count})")

    lines.append("#   c190-c249 RESERVED (future US index groups)")
    lines.append("#   c250-c259 LDCs — Limited Dissemination Controls (10)")
    lines.append("#   c266-c270 DoD Distribution Statements B-F (5)")
    lines.append("#   c271-c279 RESERVED (future LDCs / distribution)")
    lines.append("#   c280-c299 RESERVED")
    lines.append("#   c300-c399 CANADIAN PROTECTED (c303-c399 reserved for departmental use)")
    lines.append("#   c400-c499 UK OFFICIAL (reserved — pending catalog)")
    lines.append("#   c500-c599 AUSTRALIAN PSPF (reserved — pending catalog)")
    lines.append("#   c600-c1023 Unallocated")
    return "\n".join(lines)


def make_header(policy_type: str, block_map: str) -> str:
    """Generate the file header block."""
    today = date.today().isoformat()

    if policy_type == "TARGETED":
        return f"""\
#-------------------------------------------------------------------------
# SETRANS.CONF // TARGETED Policy
#
# MCS Category-to-Label translation for systems running targeted policy.
# In targeted policy, only s0 is available — there is no MLS enforcement.
# Categories still distinguish marking types, but sensitivity levels
# do not provide Bell-LaPadula dominance. Only type enforcement applies.
#
# Generated: {today} from US-CUI-LABELS.json + CANADIAN-PROTECTED.json
#
#-------------------------------------------------------------------------
# USAGE EXAMPLES
#
# Assign the base CUI umbrella to a file:
#   chcon -l s0:c0 /some/path/file
#
# Assign CUI with Investigation marking:
#   chcon -l s0:c81 /some/path/file
#
# Using chcat:
#   chcat -L             List all categories
#   chcat -d file        Remove all categories
#   chcat <cat> file     Set categories
#
#-------------------------------------------------------------------------
# BLOCK ALLOCATION MAP — Five Eyes MCS Category Ranges
#
{block_map}
#
#-------------------------------------------------------------------------
"""
    else:
        return f"""\
#-------------------------------------------------------------------------
# SETRANS.CONF // MLS Policy
#
# MCS Category-to-Label translation for systems running MLS policy.
# Sensitivity levels s1-s3 provide Bell-LaPadula dominance enforcement.
#
# Generated: {today} from US-CUI-LABELS.json + CANADIAN-PROTECTED.json
#
#-------------------------------------------------------------------------
# USAGE EXAMPLES
#
# Assign the base CUI umbrella to a file:
#   chcon -l s1:c0 /some/path/file
#
# Assign CUI with Investigation marking:
#   chcon -l s1:c81 /some/path/file
#
# Assign Canadian Protected B:
#   chcon -l s2:c301 /some/path/file
#
# Using chcat:
#   chcat -L             List all categories
#   chcat -d file        Remove all categories
#   chcat <cat> file     Set categories
#
#-------------------------------------------------------------------------
# BLOCK ALLOCATION MAP — Five Eyes MCS Category Ranges
#
{block_map}
#
#-------------------------------------------------------------------------
# SENSITIVITY LEVELS
#
# s0  SystemLow / General (lowest — binaries, libraries)
#     The sensitivity level alone does NOT imply controlled status.
#     Control is determined by category (e.g., c0 = CUI umbrella).
#
# s1  Unclassified L1 — US CUI, Canadian PA, UK OFFICIAL, AU OFFICIAL, NZ OFFICIAL
# s2  Unclassified L2 — Canadian PB, UK OFFICIAL-SENSITIVE, AU OFFICIAL:Sensitive
# s3  Unclassified L3 — Canadian PC, AU PROTECTED
#
#-------------------------------------------------------------------------
# Default setrans.conf
s0=
s0-s0:c0.c1023=SystemLow-SystemHigh
s0:c0.c1023=SystemHigh

# Bare sensitivity level with no category (fallback display)
s1=Generic Unclass L1
s2=Generic Unclass L2
s3=Generic Unclass L3
"""


def format_entry(sensitivity: str, mcs_num: int, label: str, name: str,
                 designation: str, lhs_width: int = 14,
                 label_width: int = 28) -> str:
    """Format one setrans.conf translation line."""
    lhs = f"{sensitivity}:c{mcs_num}"
    sp_tag = " [SP]" if designation == "specified" else ""
    return f"{lhs:<{lhs_width}}= {label:<{label_width}}# {name}{sp_tag}"


def generate_us_body(sensitivity: str, allocation: dict, markings: dict,
                     groups: dict) -> str:
    """Generate the US CUI section body."""
    lines = []

    # CUI umbrella
    entry = markings["CUI"]
    lines.append("")
    lines.append("#-------------------------------------------------------------------------")
    lines.append("# BASE CUI UMBRELLA")
    lines.append(format_entry(sensitivity, 0, "CUI", entry["name"], entry["designation"]))
    lines.append("")

    for ig in INDEX_GROUP_ORDER:
        if ig not in groups:
            continue
        abbrev = GROUP_ABBREV[ig]
        lines.append("")
        lines.append("#-------------------------------------------------------------------------")
        lines.append(f"# {ig.upper()} ({abbrev})")

        for key, entry in groups[ig]:
            mcs = allocation[key]
            lines.append(format_entry(
                sensitivity, mcs, key,
                entry["name"], entry.get("designation", ""),
            ))

    return "\n".join(lines)


def generate_ldc_section(sensitivity: str, ldc_data: dict) -> str:
    """Generate the LDC and Distribution Statement section."""
    lines = []
    lines.append("")
    lines.append("")
    lines.append("#-------------------------------------------------------------------------")
    lines.append("# LIMITED DISSEMINATION CONTROLS (LDCs)")
    lines.append("#  LDCs restrict who can see the data. They appear at the end of the")
    lines.append("#  CUI banner after a double slash: CUI//CATEGORY//LDC")
    lines.append("#  Parameterized LDCs (REL TO, DISPLAY ONLY) require country/org lists.")

    for ldc_key, mcs in LDC_MCS.items():
        entry = ldc_data.get(ldc_key, {})
        name = entry.get("name", ldc_key)
        banner = entry.get("banner_marking", ldc_key)
        restriction = entry.get("category_restriction")
        note = f" (restricted to {restriction})" if restriction else ""
        lhs = f"{sensitivity}:c{mcs}"
        lines.append(f"{lhs:<14}= {banner:<28}# {name}{note}")

    lines.append("")
    lines.append("#-------------------------------------------------------------------------")
    lines.append("# DoD DISTRIBUTION STATEMENTS (B-F)")
    lines.append("#  These do NOT appear in the CUI banner. They appear in the")
    lines.append("#  CUI Designation Indicator Block on the document first page.")
    lines.append("#  Tracked here as MCS-equivalent tags for access control.")

    for key, (mcs, desc) in DIST_STMT_MCS.items():
        lhs = f"{sensitivity}:c{mcs}"
        lines.append(f"{lhs:<14}= {key:<28}# {desc}")

    return "\n".join(lines)


def generate_ca_section_targeted(ca_markings: dict) -> str:
    """Generate Canadian section for TARGETED policy (all at s0)."""
    lines = []
    lines.append("")
    lines.append("")
    lines.append("#-------------------------------------------------------------------------")
    lines.append("# CANADIAN PROTECTED A/B/C (TBS Directive on Security Management)")
    lines.append("#  In targeted policy, all three tiers collapse to s0.")
    lines.append("#  Categories still distinguish them for display and audit purposes.")
    lines.append("#  In MLS policy: PA = s1:c300, PB = s2:c301, PC = s3:c302")
    lines.append("")

    for ca_key, mcs in CA_MCS.items():
        entry = ca_markings[ca_key]
        banner_en = entry["marking_banner_en"]
        banner_fr = entry["marking_banner_fr"]
        name = entry["name"]
        desc = entry.get("description", "")
        # Use French banner as the setrans display label
        lines.append(
            f"{'s0:c' + str(mcs):<14}= {banner_fr:<28}# {name}"
        )

    return "\n".join(lines)


def generate_ca_section_mls(ca_markings: dict) -> str:
    """Generate Canadian section for MLS policy (PA=s1, PB=s2, PC=s3)."""
    lines = []
    lines.append("")
    lines.append("")
    lines.append("#-------------------------------------------------------------------------")
    lines.append("# CANADIAN PROTECTED A/B/C (TBS Directive on Security Management)")
    lines.append("#  PA = s1:c300  Limited/moderate injury outside national interest")
    lines.append("#  PB = s2:c301  Serious injury outside national interest")
    lines.append("#  PC = s3:c302  Extremely grave injury outside national interest")
    lines.append("")

    level_map = {"PROTECTED-A": "s1", "PROTECTED-B": "s2", "PROTECTED-C": "s3"}
    for ca_key, mcs in CA_MCS.items():
        entry = ca_markings[ca_key]
        level = level_map[ca_key]
        banner_fr = entry["marking_banner_fr"]
        name = entry["name"]
        lines.append(
            f"{level + ':c' + str(mcs):<14}= {banner_fr:<28}# {name}"
        )

    return "\n".join(lines)


def main():
    us_data = load_json(SCRIPT_DIR / "us" / "US-CUI-LABELS.json")
    ca_data = load_json(SCRIPT_DIR / "ca" / "CANADIAN-PROTECTED.json")

    us_markings = us_data["markings"]
    us_ldcs = us_data.get("dissemination_controls", {})
    ca_markings = ca_data["markings"]

    allocation, groups, group_ranges = build_allocation(us_markings)
    block_map = build_block_map(group_ranges)

    # --- TARGETED ---
    targeted_header = make_header("TARGETED", block_map)
    targeted_level_line = "\ns0=SystemLow\n"
    targeted_us = generate_us_body("s0", allocation, us_markings, groups)
    targeted_ldc = generate_ldc_section("s0", us_ldcs)
    targeted_ca = generate_ca_section_targeted(ca_markings)

    targeted_path = SCRIPT_DIR / "TARGETED-setrans.conf"
    with open(targeted_path, "w") as f:
        f.write(targeted_header)
        f.write(targeted_level_line)
        f.write(targeted_us)
        f.write(targeted_ldc)
        f.write(targeted_ca)
        f.write("\n")

    # --- MLS ---
    mls_header = make_header("MLS", block_map)
    mls_us = generate_us_body("s1", allocation, us_markings, groups)
    mls_ldc = generate_ldc_section("s1", us_ldcs)
    mls_ca = generate_ca_section_mls(ca_markings)

    mls_path = SCRIPT_DIR / "MLS-setrans.conf"
    with open(mls_path, "w") as f:
        f.write(mls_header)
        f.write(mls_us)
        f.write(mls_ldc)
        f.write(mls_ca)
        f.write("\n")

    # Summary
    us_count = len(allocation)
    ca_count = len(CA_MCS)
    max_cat = max(allocation.values())
    print(f"Generated TARGETED-setrans.conf ({us_count} US + {ca_count} CA entries)")
    print(f"Generated MLS-setrans.conf ({us_count} US + {ca_count} CA entries)")
    print(f"US MCS range: c0-c{max_cat} (of c0-c249 available)")
    print()
    print("Block allocation:")
    for ig in INDEX_GROUP_ORDER:
        if ig not in group_ranges:
            continue
        lo, hi = group_ranges[ig]
        print(f"  c{lo:>3}-c{hi:<3}  {GROUP_ABBREV[ig]:<10} {ig} ({hi - lo + 1})")


if __name__ == "__main__":
    main()
