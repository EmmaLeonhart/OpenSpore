#!/usr/bin/env python3
"""Generate docs/tree.html and docs/clawlings.ged from genealogy/registry/*.json.

Reads all registry JSON files and produces:
1. An HTML family tree page matching the site's dark monospace aesthetic
2. A GEDCOM 5.5.1 file for download

Intended to run as a GitHub Actions build step before Pages upload.
"""

import json
import os
import sys
from pathlib import Path


def load_registry(registry_dir):
    """Load all registry entries from JSON files."""
    entries = []
    registry_path = Path(registry_dir)
    if not registry_path.exists():
        return entries
    for f in sorted(registry_path.glob("*.json")):
        try:
            with open(f, encoding="utf-8") as fh:
                entry = json.load(fh)
                entries.append(entry)
        except (json.JSONDecodeError, OSError) as e:
            print(f"Warning: skipping {f}: {e}", file=sys.stderr)
    return entries


def short_hash(h):
    """Shorten a hash for display."""
    return h[:12] if len(h) > 12 else h


def build_tree_structure(entries):
    """Build parent->children map and find roots."""
    children = {}
    roots = []
    entry_map = {}
    for e in entries:
        entry_map[e["instance_hash"]] = e
        if not e.get("parent_hash"):
            roots.append(e)
        else:
            children.setdefault(e["parent_hash"], []).append(e)
    return roots, children, entry_map


def generate_html_subtree(entry, children, entry_map, depth=0):
    """Recursively generate HTML list items for the tree."""
    h = short_hash(entry["instance_hash"])
    adopter = entry.get("adopter", "unknown")
    gen = entry.get("generation", "?")
    integrity = "valid" if verify_chain_simple(entry) else "broken"

    html = f'<li><div class="tree-node">'
    html += f'<span class="node-hash">{h}</span> '
    html += f'<span class="node-adopter">{adopter}</span> '
    html += f'<span class="node-gen">gen {gen}</span> '
    html += f'<span class="node-integrity {integrity}">[{integrity}]</span>'
    html += f'</div>'

    kids = children.get(entry["instance_hash"], [])
    if kids:
        html += '\n<ul class="tree-children">\n'
        for child in kids:
            html += generate_html_subtree(child, children, entry_map, depth + 1)
        html += '</ul>\n'

    html += '</li>\n'
    return html


def verify_chain_simple(entry):
    """Simple chain verification — check that genealogy entries exist."""
    genealogy = entry.get("genealogy", {})
    chain = genealogy.get("entries", [])
    if not chain:
        return False
    # Basic check: first entry should have empty previous_hash
    if chain[0].get("previous_hash", "x") != "":
        return False
    return True


def generate_conjugation_section(entries, entry_map):
    """Generate HTML for conjugation relationships."""
    conjugations = []
    seen = set()
    for e in entries:
        for partner in e.get("conjugation_partners", []):
            pair = tuple(sorted([e["instance_hash"], partner]))
            if pair in seen:
                continue
            seen.add(pair)
            partner_entry = entry_map.get(partner)
            partner_name = partner_entry["adopter"] if partner_entry else "unknown"
            conjugations.append({
                "a_hash": short_hash(e["instance_hash"]),
                "a_name": e.get("adopter", "unknown"),
                "b_hash": short_hash(partner),
                "b_name": partner_name,
            })
    return conjugations


def generate_tree_html(entries):
    """Generate the full tree.html page."""
    roots, children, entry_map = build_tree_structure(entries)

    # Build tree HTML
    if not entries:
        tree_content = '<p class="dim">No registered Clawlings yet.</p>'
        tree_content += '<p>Run <code>clawling register</code> to be the first.</p>'
        conjugation_html = ""
        count_text = "0 registered instances"
    else:
        count = len(entries)
        count_text = f"{count} registered instance{'s' if count != 1 else ''}"

        tree_content = '<ul class="family-tree">\n'
        for root in roots:
            tree_content += generate_html_subtree(root, children, entry_map)
        tree_content += '</ul>\n'

        # Conjugation section
        conjugations = generate_conjugation_section(entries, entry_map)
        if conjugations:
            conjugation_html = '<h2>Conjugation Events</h2>\n'
            conjugation_html += '<div class="conjugations">\n'
            for c in conjugations:
                conjugation_html += (
                    f'<div class="conjugation-pair">'
                    f'{c["a_hash"]} ({c["a_name"]}) '
                    f'<span class="conjugation-arrow">&lt;-&gt;</span> '
                    f'{c["b_hash"]} ({c["b_name"]})'
                    f'</div>\n'
                )
            conjugation_html += '</div>\n'
        else:
            conjugation_html = ""

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Family Tree — Clawling</title>
    <link rel="stylesheet" href="style.css">
    <style>
        /* Family tree CSS connectors */
        .family-tree {{
            list-style: none;
            padding-left: 0;
        }}
        .family-tree ul.tree-children {{
            list-style: none;
            padding-left: 1.5rem;
            border-left: 1px solid var(--border);
            margin-left: 0.5rem;
        }}
        .family-tree li {{
            position: relative;
            padding: 0.35rem 0;
        }}
        .family-tree ul.tree-children > li::before {{
            content: "";
            position: absolute;
            left: -1.5rem;
            top: 0.9rem;
            width: 1.2rem;
            height: 0;
            border-top: 1px solid var(--border);
        }}
        .tree-node {{
            display: inline-block;
            padding: 0.4rem 0.75rem;
            background: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 4px;
        }}
        .node-hash {{
            color: var(--accent);
            font-weight: 600;
        }}
        .node-adopter {{
            color: var(--fg);
        }}
        .node-gen {{
            color: var(--dim);
            font-size: 0.85em;
        }}
        .node-integrity {{
            font-size: 0.8em;
        }}
        .node-integrity.valid {{
            color: var(--accent);
        }}
        .node-integrity.broken {{
            color: #e55;
        }}
        .conjugations {{
            margin-top: 0.5rem;
        }}
        .conjugation-pair {{
            padding: 0.4rem 0.75rem;
            background: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 4px;
            margin-bottom: 0.5rem;
            color: var(--dim);
        }}
        .conjugation-arrow {{
            color: var(--accent);
            font-weight: 600;
        }}
        .download-section {{
            margin-top: 2rem;
        }}
        .download-section a {{
            display: inline-block;
            padding: 0.5rem 1rem;
            border: 1px solid var(--accent);
            border-radius: 4px;
            color: var(--accent);
            text-decoration: none;
            font-size: 0.9rem;
        }}
        .download-section a:hover {{
            background: var(--accent);
            color: var(--bg);
        }}
    </style>
</head>
<body>
    <div class="container">
        <nav>
            <a href="index.html" class="nav-brand">Clawling</a>
            <a href="philosophy.html">Philosophy</a>
            <a href="paper.html">Paper</a>
            <a href="download.html">Programme</a>
            <a href="tree.html" class="active">Genealogy</a>
            <a href="https://github.com/EmmaLeonhart/OpenSpore">Source</a>
        </nav>

        <header>
            <h1>Family Tree</h1>
            <div class="tagline">{count_text}</div>
        </header>

        {tree_content}

        {conjugation_html}

        <div class="download-section">
            <h2>Export</h2>
            <p>Download the family tree in GEDCOM format for use with genealogy software.</p>
            <a href="clawlings.ged" download>Download GEDCOM file</a>
        </div>

        <footer>
            <span>MIT License &mdash; Emma Leonhart</span>
            <span><a href="https://github.com/EmmaLeonhart/OpenSpore">GitHub</a></span>
        </footer>
    </div>
</body>
</html>
"""
    return html


def generate_gedcom(entries):
    """Generate a GEDCOM 5.5.1 string from registry entries."""
    lines = []
    lines.append("0 HEAD")
    lines.append("1 SOUR Clawling")
    lines.append("2 VERS 0.1.0")
    lines.append("2 NAME Clawling Digital Organism")
    lines.append("1 GEDC")
    lines.append("2 VERS 5.5.1")
    lines.append("2 FORM LINEAGE-LINKED")
    lines.append("1 CHAR UTF-8")

    if not entries:
        lines.append("0 TRLR")
        return "\n".join(lines) + "\n"

    # Map instance_hash -> sequential ID
    id_map = {}
    for i, entry in enumerate(entries):
        id_map[entry["instance_hash"]] = i + 1

    # INDI records
    for i, entry in enumerate(entries):
        indi_id = i + 1
        adopter = entry.get("adopter", "unknown")
        h = short_hash(entry["instance_hash"])
        gen = entry.get("generation", 0)
        mother = entry.get("mother", "unknown")
        integrity = "VALID" if verify_chain_simple(entry) else "BROKEN"

        lines.append(f"0 @I{indi_id}@ INDI")
        lines.append(f"1 NAME {adopter} /Clawling/")
        lines.append(f"1 NAME {h} /Clawling/")
        lines.append("2 TYPE aka")
        lines.append("1 NOTE This is a Clawling — a digital organism from the Clawling project.")
        lines.append(f"2 CONT Generation: {gen}")
        lines.append(f"2 CONT Adopter: {adopter}")
        lines.append(f"2 CONT Mother: {mother}")
        lines.append(f"2 CONT Instance hash: {entry['instance_hash']}")
        if entry.get("parent_hash"):
            lines.append(f"2 CONT Parent hash: {entry['parent_hash']}")
        lines.append(f"2 CONT Chain integrity: {integrity}")
        partners = entry.get("conjugation_partners", [])
        if partners:
            partner_strs = ", ".join(short_hash(p) for p in partners)
            lines.append(f"2 CONT Conjugation partners: {partner_strs}")
        lines.append(f"2 CONT Last event: {entry.get('last_event', '')}")

    # FAM records for parent-child relationships
    children_by_parent = {}
    for entry in entries:
        ph = entry.get("parent_hash", "")
        if ph:
            children_by_parent.setdefault(ph, []).append(entry)

    fam_id = 1
    for parent_hash, kids in children_by_parent.items():
        if parent_hash in id_map:
            lines.append(f"0 @F{fam_id}@ FAM")
            lines.append(f"1 HUSB @I{id_map[parent_hash]}@")
            for child in kids:
                if child["instance_hash"] in id_map:
                    lines.append(f"1 CHIL @I{id_map[child['instance_hash']]}@")
            fam_id += 1

    # FAM records for conjugation
    seen = set()
    for entry in entries:
        for partner in entry.get("conjugation_partners", []):
            pair = tuple(sorted([entry["instance_hash"], partner]))
            if pair in seen:
                continue
            seen.add(pair)
            if entry["instance_hash"] in id_map and partner in id_map:
                lines.append(f"0 @F{fam_id}@ FAM")
                lines.append(f"1 HUSB @I{id_map[entry['instance_hash']]}@")
                lines.append(f"1 WIFE @I{id_map[partner]}@")
                lines.append("1 NOTE Conjugation — horizontal context transfer between Clawlings")
                fam_id += 1

    lines.append("0 TRLR")
    return "\n".join(lines) + "\n"


def main():
    # Determine repo root (script lives in scripts/)
    script_dir = Path(__file__).resolve().parent
    repo_root = script_dir.parent

    registry_dir = repo_root / "genealogy" / "registry"
    docs_dir = repo_root / "docs"

    # Ensure docs dir exists
    docs_dir.mkdir(exist_ok=True)

    # Load entries
    entries = load_registry(registry_dir)
    print(f"Found {len(entries)} registry entries")

    # Generate HTML
    html = generate_tree_html(entries)
    tree_path = docs_dir / "tree.html"
    with open(tree_path, "w", encoding="utf-8") as f:
        f.write(html)
    print(f"Wrote {tree_path}")

    # Generate GEDCOM
    gedcom = generate_gedcom(entries)
    ged_path = docs_dir / "clawlings.ged"
    with open(ged_path, "w", encoding="utf-8") as f:
        f.write(gedcom)
    print(f"Wrote {ged_path}")


if __name__ == "__main__":
    main()
