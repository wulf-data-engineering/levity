#!/usr/bin/env python3
"""
Check generated project for undesired strings (cookiecutter defaults and/or explicit checks),
with Git-diff style output and custom delimiters.

Features:
- --check takes pairs: var_name literal_string
- Ignores variables starting with '_' or in --ignore
- Enforces explicit --check for defaults starting with Jinja2 start delimiter
- Git-diff output with %[cookiecutter.var_name]% replacements

Usage:
python scripts/check_defaults.py --template cookiecutter.json --generated out/MyProj --extra extra_context.json \
    -c project_name "Tool-Set Project" -c test_user_email "test@wulf.technology" \
    --ignore ignored_var
"""
from __future__ import annotations
import argparse
import json
import os
import sys
from typing import Dict, List, Optional, Set, Tuple

# --------------------------
# Helpers
# --------------------------
def load_json(path: str) -> object:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)

def flatten_values(obj) -> Set[str]:
    vals: Set[str] = set()
    if isinstance(obj, dict):
        for v in obj.values():
            vals.update(flatten_values(v))
    elif isinstance(obj, (list, tuple)):
        for v in obj:
            vals.update(flatten_values(v))
    elif obj is None:
        pass
    else:
        vals.add(str(obj))
    return vals

def reverse_lookup_in_mapping(mapping: Dict[str, object], target: str) -> Optional[str]:
    if not isinstance(mapping, dict):
        return None
    for k, v in mapping.items():
        if isinstance(v, (dict, list, tuple)):
            flat = flatten_values(v)
            if target in flat:
                return k
        else:
            if str(v) == target:
                return k
    return None

def reverse_lookup(extra_map: Dict[str, object], template_map: Dict[str, object], value: str) -> Optional[str]:
    name = reverse_lookup_in_mapping(extra_map or {}, value)
    if name:
        return name
    name = reverse_lookup_in_mapping(template_map or {}, value)
    return name

def is_binary_file(path: str) -> bool:
    try:
        with open(path, "rb") as fh:
            chunk = fh.read(1024)
            return b"\0" in chunk
    except Exception:
        return True

def gather_text_files(root: str, exclude_dirs: Optional[List[str]] = None) -> List[str]:
    exclude_dirs_set = set(exclude_dirs or [])
    files: List[str] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [
            d for d in dirnames
            if d not in exclude_dirs_set and os.path.join(dirpath, d) not in exclude_dirs_set
        ]
        for fn in filenames:
            full = os.path.join(dirpath, fn)
            if is_binary_file(full):
                continue
            files.append(full)
    return files

def find_matches_in_file(path: str, patterns: List[str]) -> List[Tuple[int, str, str]]:
    matches: List[Tuple[int, str, str]] = []
    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as fh:
            for i, line in enumerate(fh, start=1):
                for p in patterns:
                    if p and p in line:
                        matches.append((i, p, line.rstrip("\n")))
    except Exception:
        return []
    return matches

# --------------------------
# Main
# --------------------------
def main():
    ap = argparse.ArgumentParser(description="Check generated project for cookiecutter defaults and/or explicit strings.")
    ap.add_argument("--template", help="Path to cookiecutter.json (template defaults)")
    ap.add_argument("--generated", required=True, help="Path to generated project directory")
    ap.add_argument("--extra", help="Path to extra_context.json (seeded values you used)")
    ap.add_argument("--ignore", nargs="*", default=[], help="Variable names to ignore. Variables starting with _ are automatically ignored.")
    ap.add_argument("-c", "--check",  nargs=2, metavar=("VAR_NAME", "LITERAL_STRING"), action="append", default=[], help="Explicit check: pass in pairs var_name literal_string")
    args = ap.parse_args()

    # Load template
    template_map: Dict[str, object] = {}
    if args.template:
        template_map = load_json(args.template)

    # Determine Jinja2 delimiters
    jinja2_start = "{{"
    jinja2_end = "}}"
    if "_jinja2_env_vars" in template_map and isinstance(template_map["_jinja2_env_vars"], dict):
        jenv = template_map["_jinja2_env_vars"]
        jinja2_start = jenv.get("variable_start_string", jinja2_start)
        jinja2_end   = jenv.get("variable_end_string", jinja2_end)

    # Load extra_context
    extra_map: Dict[str, object] = {}
    if args.extra and os.path.exists(args.extra):
        extra_map = load_json(args.extra)

    # Build explicit_checks list as tuples (var_name, literal)
    explicit_checks: List[Tuple[str,str]] = [(vn, lit) for vn, lit in args.check]
    # extract explicit var names for fast lookup
    explicit_var_names = {vn for vn, _ in explicit_checks}

    # Build remaining defaults
    remaining_defaults: List[Tuple[str,str]] = []
    for var_name, val in template_map.items():
        if var_name.startswith("_") or var_name in (args.ignore or []) or var_name in explicit_var_names:
            continue
        val_str = str(val)
        # if default starts with start delimiter, require explicit --check
        if val_str.startswith(jinja2_start):
            print(f"ERROR: Variable '{var_name}' default starts with delimiter {jinja2_start}. Must pass it explicitly via --check.")
            sys.exit(1)
        remaining_defaults.append((var_name, val_str))

    # Prepare file patterns for searching
    patterns = [literal for _,literal in remaining_defaults] + [literal for _,literal in explicit_checks]
    files = gather_text_files(args.generated, exclude_dirs=[os.path.join(args.generated, ".git")])
    matches_found: List[Tuple[str,int,str,str,Optional[str]]] = []

    # Sort patterns longest first
    patterns_sorted = sorted(patterns, key=lambda s: -len(s))

    for fp in files:
        file_matches = find_matches_in_file(fp, patterns_sorted)
        for line_no, patt, line in file_matches:
            var_name: Optional[str] = None
            # First check explicit
            for vn, literal in explicit_checks:
                if literal == patt:
                    var_name = vn
                    break
            # Then defaults
            if not var_name:
                for vn, val in remaining_defaults:
                    if val == patt:
                        var_name = vn
                        break
            # Skip ignored or _ vars
            if var_name and (var_name in (args.ignore or []) or var_name.startswith("_")):
                continue
            matches_found.append((fp, line_no, patt, line, var_name))

    if matches_found:
        print("ERROR: Found occurrences of checked strings in generated project.\n")
        for i, (fp, line_no, patt, line, var_name) in enumerate(matches_found, start=1):
            print(f"{i}. {fp}:{line_no}  <-- matched: {patt!r}")
            # Git-diff style
            print(f"-   {line}")
            if var_name:
                replacement_line = line.replace(patt, f"{jinja2_start}cookiecutter.{var_name}{jinja2_end}")
                print(f"+   {replacement_line}")
            else:
                print(f"+   {line}  # no variable mapping found")
        print("")
        sys.exit(1)

    print("OK: No checked strings found in the generated project.")
    sys.exit(0)

if __name__ == "__main__":
    main()
