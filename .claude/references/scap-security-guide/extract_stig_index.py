#!/usr/bin/env python3
"""Extract structured signal index and CCE cross-reference from RHEL10 STIG playbook."""

import re
import sys
from collections import OrderedDict
from pathlib import Path

STIG_FILE = Path(__file__).parent / "rhel10-playbook-stig.yml"
SIGNAL_INDEX_OUT = Path(__file__).parent / "stig-signal-index.md"
CCE_CROSSREF_OUT = Path(__file__).parent / "cce-nist-crossref.md"

# Tag classification patterns
CCE_RE = re.compile(r'^CCE-\d+-\d+$')
NIST_RE = re.compile(r'^NIST-800-53-')
SEVERITY_RE = re.compile(r'^(low|medium|high|unknown)_severity$')
SKIP_TAGS = {
    'disable_strategy', 'enable_strategy', 'restrict_strategy',
    'configure_strategy', 'patch_strategy', 'unknown_strategy',
    'low_complexity', 'medium_complexity', 'high_complexity',
    'low_disruption', 'medium_disruption', 'high_disruption',
    'no_reboot_needed', 'reboot_required',
}


def classify_check_method(signal_name):
    """Categorize signal by its name prefix."""
    if signal_name.startswith('sysctl_'):
        return 'sysctl'
    if signal_name.startswith('audit_rules_'):
        return 'audit-rule'
    if signal_name.startswith('service_') or signal_name.startswith('socket_'):
        return 'service-check'
    if signal_name.startswith('package_'):
        return 'package-check'
    if signal_name.startswith('grub2_') or signal_name.startswith('coreos_'):
        return 'cmdline'
    if any(signal_name.startswith(p) for p in [
        'file_', 'dir_', 'mount_', 'permissions_', 'owner_', 'group_',
    ]):
        return 'file-check'
    return 'other'


def parse_stig(path):
    """Line-by-line parse of the STIG YAML to extract task metadata."""
    signals = OrderedDict()  # signal_name -> {cce, nist, severity, description, ...}

    current_task_name = None
    in_tags = False
    current_tags = []

    with open(path, 'r') as f:
        for line in f:
            stripped = line.strip()

            # Detect task name
            if stripped.startswith('- name:'):
                # Flush previous task
                if current_task_name and current_tags:
                    _process_task(signals, current_task_name, current_tags)
                current_task_name = stripped[7:].strip()
                in_tags = False
                current_tags = []
                continue

            # Detect tags section
            if stripped == 'tags:':
                in_tags = True
                current_tags = []
                continue

            # Collect tag values
            if in_tags and stripped.startswith('- '):
                tag = stripped[2:].strip()
                current_tags.append(tag)
                continue

            # End of tags section
            if in_tags and not stripped.startswith('- '):
                in_tags = False

    # Flush last task
    if current_task_name and current_tags:
        _process_task(signals, current_task_name, current_tags)

    return signals


def _process_task(signals, task_name, tags):
    """Process a single task's tags into the signals dict."""
    cce = None
    nist_controls = []
    severity = None
    signal_name = None

    for tag in tags:
        if CCE_RE.match(tag):
            cce = tag
        elif NIST_RE.match(tag):
            nist_controls.append(tag.replace('NIST-800-53-', ''))
        elif SEVERITY_RE.match(tag):
            severity = tag.replace('_severity', '')
        elif tag in SKIP_TAGS:
            continue
        else:
            signal_name = tag

    if not signal_name or not cce:
        return

    # Only keep the first occurrence (dedup by signal name)
    if signal_name not in signals:
        signals[signal_name] = {
            'cce': cce,
            'nist': sorted(set(nist_controls)),
            'severity': severity or 'unknown',
            'description': task_name,
            'check_method': classify_check_method(signal_name),
        }
    else:
        # Merge NIST controls from subsequent tasks
        existing = signals[signal_name]
        for ctrl in nist_controls:
            ctrl_clean = ctrl
            if ctrl_clean not in existing['nist']:
                existing['nist'].append(ctrl_clean)
                existing['nist'].sort()


def write_signal_index(signals, out_path):
    """Write the deduplicated signal index markdown table."""
    with open(out_path, 'w') as f:
        f.write("# RHEL 10 STIG — Signal Index\n\n")
        f.write(f"**Source:** `rhel10-playbook-stig.yml` (SCAP Security Guide)\n")
        f.write(f"**Profile:** xccdf_org.ssgproject.content_profile_stig\n")
        f.write(f"**Signals:** {len(signals)}\n\n")
        f.write("| Signal Name | CCE | NIST Controls | Severity | Description | Check Method |\n")
        f.write("|---|---|---|---|---|---|\n")
        for name, info in sorted(signals.items()):
            nist = ', '.join(info['nist'])
            desc = info['description'].replace('|', '\\|')[:80]
            f.write(f"| `{name}` | {info['cce']} | {nist} | {info['severity']} | {desc} | {info['check_method']} |\n")

    print(f"Signal index: {len(signals)} signals written to {out_path}")


def write_cce_crossref(signals, out_path):
    """Write the CCE → NIST cross-reference table sorted by CCE."""
    # Build CCE-keyed view
    by_cce = {}
    for name, info in signals.items():
        cce = info['cce']
        if cce not in by_cce:
            by_cce[cce] = {
                'nist': list(info['nist']),
                'signal': name,
                'description': info['description'],
            }
        else:
            # Merge if same CCE maps to multiple signals (unlikely but safe)
            for ctrl in info['nist']:
                if ctrl not in by_cce[cce]['nist']:
                    by_cce[cce]['nist'].append(ctrl)

    with open(out_path, 'w') as f:
        f.write("# RHEL 10 STIG — CCE → NIST Cross-Reference\n\n")
        f.write(f"**Source:** `rhel10-playbook-stig.yml` (SCAP Security Guide)\n")
        f.write(f"**Unique CCEs:** {len(by_cce)}\n\n")
        f.write("| CCE | NIST Controls | Signal Name | Description |\n")
        f.write("|---|---|---|---|\n")
        for cce in sorted(by_cce.keys()):
            info = by_cce[cce]
            nist = ', '.join(sorted(info['nist']))
            desc = info['description'].replace('|', '\\|')[:80]
            f.write(f"| {cce} | {nist} | `{info['signal']}` | {desc} |\n")

    print(f"CCE cross-reference: {len(by_cce)} CCEs written to {out_path}")


def main():
    if not STIG_FILE.exists():
        print(f"ERROR: {STIG_FILE} not found", file=sys.stderr)
        sys.exit(1)

    signals = parse_stig(STIG_FILE)
    write_signal_index(signals, SIGNAL_INDEX_OUT)
    write_cce_crossref(signals, CCE_CROSSREF_OUT)


if __name__ == '__main__':
    main()
