#!/usr/bin/env python3
"""Helper utilities and orchestrator logic for Antigravity Security Remediation.

Categorizes and deduplicates GitHub Dependabot and CodeQL security alerts into:
1. Phase 1: A single consolidated version-upgrade PR ('dependabot/security-version-upgrades').
2. Phase 2: Grouped code/dependency-replacement PRs ('security/fix-<slug>', capped per run).
3. Phase 3: A single rolling GitHub Issue ('Security Alerts Requiring Human Decision') for
   unpatched or unfixable alerts assessed by Antigravity.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from typing import Any

ROLLING_ISSUE_TITLE = "Security Alerts Requiring Human Decision"
VERSION_UPGRADE_BRANCH = "dependabot/security-version-upgrades"

SEVERITY_RANK = {
    "critical": 0,
    "high": 1,
    "medium": 2,
    "moderate": 2,
    "warning": 2,
    "error": 1,
    "low": 3,
    "note": 3,
}


def slugify(value: str) -> str:
    """Convert an arbitrary rule ID or package name into a safe git branch slug."""
    slug = re.sub(r"[^a-zA-Z0-9]+", "-", value.strip().lower()).strip("-")
    return slug[:48] or "alert"


def severity_key(severity: str) -> int:
    """Return numeric sort rank for severity (lower is higher priority)."""
    return SEVERITY_RANK.get((severity or "").lower(), 4)


def parse_semver_tuple(version_str: str) -> tuple[int, ...]:
    """Parse the leading numeric components of a version string for comparison."""
    cleaned = version_str.lstrip("^~>=<v ")
    parts = re.split(r"[^0-9]+", cleaned)
    nums: list[int] = []
    for part in parts:
        if part.isdigit():
            nums.append(int(part))
        else:
            break
    return tuple(nums)


def extract_documented_alert_ids(issue_body: str) -> set[str]:
    """Extract all '<!-- alert-id: ... -->' markers from the rolling issue body."""
    if not issue_body:
        return set()
    return set(re.findall(r"<!--\s*alert-id:\s*([^\s>]+)\s*-->", issue_body))


def categorize_alerts(
    dependabot_alerts: list[dict[str, Any]],
    codeql_alerts: list[dict[str, Any]],
    open_prs: list[dict[str, Any]],
    rolling_issue_body: str = "",
    max_individual_prs: int = 3,
    promoted_packages: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    """Categorize open Dependabot and CodeQL alerts into the 3 remediation phases.

    Returns a dictionary with:
    - 'version_upgrades': list of patchable Dependabot package groups for Phase 1
    - 'skip_version_upgrades': True if an open PR already covers the exact same alerts
    - 'existing_version_upgrade_pr': PR number if 'dependabot/security-version-upgrades' is open
    - 'individual_groups': capped list of Phase 2 groups (CodeQL rule groups + promoted packages)
    - 'unpatched_alerts': list of Phase 3 unpatched alerts not yet in the rolling issue
    - 'active_alert_ids': sorted list of all currently open alert IDs
    """
    documented_ids = extract_documented_alert_ids(rolling_issue_body)
    if not isinstance(dependabot_alerts, list):
        dependabot_alerts = []
    if not isinstance(codeql_alerts, list):
        codeql_alerts = []
    if not isinstance(open_prs, list):
        open_prs = []

    open_pr_by_branch: dict[str, dict[str, Any]] = {
        pr.get("headRefName", ""): pr
        for pr in open_prs
        if isinstance(pr, dict) and pr.get("headRefName")
    }

    active_alert_ids: set[str] = set()
    patchable_by_key: dict[tuple[str, str, str], dict[str, Any]] = {}
    unpatched_list: list[dict[str, Any]] = []

    for alert in dependabot_alerts:
        if not isinstance(alert, dict) or alert.get("state") != "open":
            continue
        number = alert.get("number")
        alert_id = f"dependabot-{number}"
        active_alert_ids.add(alert_id)

        dep = alert.get("dependency", {})
        pkg = dep.get("package", {})
        ecosystem = (pkg.get("ecosystem") or "").lower()
        pkg_name = pkg.get("name") or ""
        manifest_path = dep.get("manifest_path") or ""

        vuln = alert.get("security_vulnerability", {}) or {}
        advisory = alert.get("security_advisory", {}) or {}
        severity = (vuln.get("severity") or advisory.get("severity") or "low").lower()
        first_patched = (vuln.get("first_patched_version") or {}).get("identifier")
        summary = advisory.get("summary") or ""
        ghsa_id = advisory.get("ghsa_id") or ""
        cve_id = advisory.get("cve_id") or ""

        if first_patched:
            key = (ecosystem, manifest_path, pkg_name)
            group = patchable_by_key.get(key)
            if group is None:
                group = {
                    "ecosystem": ecosystem,
                    "manifest_path": manifest_path,
                    "package": pkg_name,
                    "patched_version": first_patched,
                    "severity": severity,
                    "alert_numbers": [number],
                    "alert_ids": [alert_id],
                    "summaries": [summary] if summary else [],
                }
                patchable_by_key[key] = group
            else:
                if number not in group["alert_numbers"]:
                    group["alert_numbers"].append(number)
                    group["alert_ids"].append(alert_id)
                if summary and summary not in group["summaries"]:
                    group["summaries"].append(summary)
                if parse_semver_tuple(first_patched) > parse_semver_tuple(
                    group["patched_version"]
                ):
                    group["patched_version"] = first_patched
                if severity_key(severity) < severity_key(group["severity"]):
                    group["severity"] = severity
        else:
            if alert_id not in documented_ids:
                unpatched_list.append(
                    {
                        "alert_id": alert_id,
                        "alert_number": number,
                        "ecosystem": ecosystem,
                        "manifest_path": manifest_path,
                        "package": pkg_name,
                        "severity": severity,
                        "ghsa_id": ghsa_id,
                        "cve_id": cve_id,
                        "summary": summary,
                    }
                )

    version_upgrades = sorted(
        patchable_by_key.values(),
        key=lambda g: (
            severity_key(g["severity"]),
            g["manifest_path"],
            g["package"],
        ),
    )

    # Check if an existing open PR on VERSION_UPGRADE_BRANCH already covers all version_upgrades
    existing_upgrade_pr = open_pr_by_branch.get(VERSION_UPGRADE_BRANCH)
    existing_upgrade_pr_number = (
        existing_upgrade_pr.get("number") if existing_upgrade_pr else None
    )
    skip_version_upgrades = False
    if not version_upgrades:
        skip_version_upgrades = True
    elif existing_upgrade_pr:
        pr_documented_ids = extract_documented_alert_ids(
            existing_upgrade_pr.get("body") or ""
        )
        required_ids = {
            aid for g in version_upgrades for aid in g.get("alert_ids", [])
        }
        if required_ids and required_ids.issubset(pr_documented_ids):
            skip_version_upgrades = True

    # Phase 2 groups: CodeQL alerts grouped by rule.id + any promoted complex Dependabot groups
    phase2_candidates: list[dict[str, Any]] = []

    if promoted_packages:
        for prom in promoted_packages:
            pkg_slug = slugify(f"{prom.get('ecosystem', 'dep')}-{prom.get('package', 'pkg')}")
            branch = f"security/fix-{pkg_slug}"
            if branch in open_pr_by_branch:
                continue
            alert_ids = prom.get("alert_ids") or [
                f"dependabot-{n}" for n in prom.get("alert_numbers", [])
            ]
            if alert_ids and all(aid in documented_ids for aid in alert_ids):
                continue
            phase2_candidates.append(
                {
                    "kind": "dependabot",
                    "group_id": f"dependabot-{pkg_slug}",
                    "branch": branch,
                    "ecosystem": prom.get("ecosystem", ""),
                    "package": prom.get("package", ""),
                    "manifest_path": prom.get("manifest_path", ""),
                    "patched_version": prom.get("patched_version", ""),
                    "severity": prom.get("severity", "medium"),
                    "alert_numbers": sorted(prom.get("alert_numbers", [])),
                    "alert_ids": alert_ids,
                    "summaries": prom.get("summaries", []),
                }
            )

    codeql_by_rule: dict[str, dict[str, Any]] = {}
    for alert in codeql_alerts:
        if not isinstance(alert, dict) or alert.get("state") != "open":
            continue
        number = alert.get("number")
        alert_id = f"codeql-{number}"
        active_alert_ids.add(alert_id)

        if alert_id in documented_ids:
            continue

        rule = alert.get("rule", {}) or {}
        rule_id = rule.get("id") or f"rule-{number}"
        severity = (
            rule.get("security_severity_level")
            or rule.get("severity")
            or "medium"
        ).lower()
        description = rule.get("description") or ""

        instance = alert.get("most_recent_instance", {}) or {}
        loc = instance.get("location", {}) or {}
        msg = (instance.get("message") or {}).get("text") or ""
        location_entry = {
            "alert_number": number,
            "path": loc.get("path") or "",
            "start_line": loc.get("start_line") or 1,
            "end_line": loc.get("end_line") or 1,
            "message": msg,
        }

        group = codeql_by_rule.get(rule_id)
        if group is None:
            rule_slug = slugify(rule_id)
            group = {
                "kind": "codeql",
                "group_id": f"codeql-{rule_slug}",
                "branch": f"security/fix-codeql-{rule_slug}",
                "rule_id": rule_id,
                "description": description,
                "severity": severity,
                "alert_numbers": [number],
                "alert_ids": [alert_id],
                "locations": [location_entry],
            }
            codeql_by_rule[rule_id] = group
        else:
            if number not in group["alert_numbers"]:
                group["alert_numbers"].append(number)
                group["alert_ids"].append(alert_id)
            group["locations"].append(location_entry)
            if severity_key(severity) < severity_key(group["severity"]):
                group["severity"] = severity

    for group in codeql_by_rule.values():
        if group["branch"] in open_pr_by_branch:
            continue
        group["alert_numbers"].sort()
        phase2_candidates.append(group)

    phase2_candidates.sort(
        key=lambda g: (
            severity_key(g.get("severity", "low")),
            g["alert_numbers"][0] if g.get("alert_numbers") else 999999,
        )
    )

    individual_groups = phase2_candidates[: max(0, max_individual_prs)]

    unpatched_list.sort(
        key=lambda a: (severity_key(a.get("severity", "low")), a.get("alert_number", 0))
    )

    return {
        "version_upgrades": version_upgrades,
        "skip_version_upgrades": skip_version_upgrades,
        "existing_version_upgrade_pr": existing_upgrade_pr_number,
        "individual_groups": individual_groups,
        "unpatched_alerts": unpatched_list,
        "active_alert_ids": sorted(active_alert_ids),
    }


def update_cargo_toml_version(
    cargo_toml_text: str, crate_name: str, new_version: str
) -> tuple[str, bool]:
    """Update a direct dependency's version in Cargo.toml if present and older than new_version.

    Supports both simple string declarations (`crate = "1.2.3"`) and inline table
    declarations (`crate = { version = "1.2.3", ... }`).
    """
    escaped = re.escape(crate_name)
    modified = False

    # Pattern 1: crate = "1.2.3"
    simple_pattern = re.compile(
        rf'^(\s*{escaped}\s*=\s*")([^"]+)(".*)$', re.MULTILINE
    )

    def replace_simple(match: re.Match[str]) -> str:
        nonlocal modified
        current_ver = match.group(2)
        if parse_semver_tuple(new_version) > parse_semver_tuple(current_ver):
            modified = True
            return f"{match.group(1)}{new_version}{match.group(3)}"
        return match.group(0)

    updated = simple_pattern.sub(replace_simple, cargo_toml_text)

    # Pattern 2: crate = { ... version = "1.2.3" ... }
    table_pattern = re.compile(
        rf'^(\s*{escaped}\s*=\s*\{{[^}}]*?\bversion\s*=\s*")([^"]+)(".*\}}.*)$',
        re.MULTILINE,
    )

    def replace_table(match: re.Match[str]) -> str:
        nonlocal modified
        current_ver = match.group(2)
        if parse_semver_tuple(new_version) > parse_semver_tuple(current_ver):
            modified = True
            return f"{match.group(1)}{new_version}{match.group(3)}"
        return match.group(0)

    updated = table_pattern.sub(replace_table, updated)
    return updated, modified


def add_or_update_npm_override(
    package_json_text: str, pkg_name: str, patched_version: str
) -> str:
    """Add or update an npm 'overrides' entry in package.json while preserving indentation."""
    indent: str | int = 2
    for line in package_json_text.splitlines():
        if line.startswith("\t"):
            indent = "\t"
            break
        m = re.match(r"^( +)\"", line)
        if m:
            indent = len(m.group(1))
            break

    data = json.loads(package_json_text)
    overrides = data.get("overrides")
    if not isinstance(overrides, dict):
        overrides = {}
    target_spec = f"^{patched_version.lstrip('^~')}"
    overrides[pkg_name] = target_spec
    data["overrides"] = overrides

    trailing_newline = "\n" if package_json_text.endswith("\n") else ""
    return json.dumps(data, indent=indent) + trailing_newline


def render_rolling_issue_body(
    existing_body: str,
    new_assessments: list[dict[str, Any]],
    active_alert_ids: set[str] | None = None,
    repo: str = "",
) -> str:
    """Render the rolling 'Security Alerts Requiring Human Decision' issue markdown body.

    Preserves existing alert sections that are still active and appends newly assessed alerts.
    """
    header = (
        "## 🛡️ Security Alerts Requiring Human Decision\n\n"
        "This rolling issue tracks security alerts that either have **no upstream patch available** "
        "or **cannot be safely resolved via automated changes** without human architectural review.\n\n"
        "> [!NOTE]\n"
        "> Antigravity does **not** automatically dismiss security alerts. Review the reachability "
        "assessments below and either dismiss the alert on GitHub or schedule a manual refactoring."
    )

    # Extract existing blocks delimited by <!-- alert-start: ID --> ... <!-- alert-end: ID -->
    block_pattern = re.compile(
        r"(<!--\s*alert-start:\s*([^\s>]+)\s*-->.*?<!--\s*alert-end:\s*\2\s*-->)",
        re.DOTALL,
    )
    preserved_blocks: list[str] = []
    seen_ids: set[str] = set()

    for match in block_pattern.finditer(existing_body or ""):
        block_text, alert_id = match.group(1), match.group(2)
        if active_alert_ids is not None and alert_id not in active_alert_ids:
            continue
        preserved_blocks.append(block_text.strip())
        seen_ids.add(alert_id)

    for item in new_assessments:
        alert_id = item.get("alert_id") or f"dependabot-{item.get('alert_number')}"
        if alert_id in seen_ids:
            continue
        seen_ids.add(alert_id)

        alert_num = item.get("alert_number", "")
        pkg = item.get("package") or item.get("rule_id") or alert_id
        ecosystem = item.get("ecosystem", "")
        manifest = item.get("manifest_path", "")
        severity = (item.get("severity") or "unknown").upper()
        ghsa = item.get("ghsa_id") or ""
        cve = item.get("cve_id") or ""
        summary = item.get("summary") or item.get("description") or ""
        reachable = item.get("reachable", False)
        dev_only = item.get("dev_only", False)
        recommendation = item.get("recommendation") or "tolerable_risk"
        reasoning = item.get("reasoning") or "No reasoning provided."
        target_repo = item.get("repo") or repo or "<owner>/<repo>"

        advisory_str = " / ".join(x for x in [ghsa, cve] if x) or "N/A"
        scope_str = "Dev / Build-time only" if dev_only else "Runtime / Production"
        reach_str = "Potentially reachable" if reachable else "Not reachable in codebase"

        dismiss_cmd = ""
        if str(alert_id).startswith("dependabot-") and alert_num:
            dismiss_reason = (
                recommendation
                if recommendation in {"tolerable_risk", "not_used", "inaccurate"}
                else "tolerable_risk"
            )
            escaped_comment = reasoning.replace('"', '\\"')
            dismiss_cmd = (
                f"\n**Suggested CLI Dismissal Command (if approved)**:\n"
                f"```bash\n"
                f"gh api -X PATCH repos/{target_repo}/dependabot/alerts/{alert_num} \\\n"
                f"  -f state=dismissed \\\n"
                f"  -f dismissed_reason={dismiss_reason} \\\n"
                f'  -f dismissed_comment="{escaped_comment}"\n'
                f"```\n"
            )

        block = (
            f"<!-- alert-start: {alert_id} -->\n"
            f"<!-- alert-id: {alert_id} -->\n"
            f"### `{pkg}` — {severity} ({alert_id})\n"
            f"- **Advisory / Rule**: {advisory_str}\n"
            f"- **Manifest / Location**: `{manifest or ecosystem or 'repository'}`\n"
            f"- **Summary**: {summary}\n"
            f"- **Scope**: {scope_str} · **Reachability**: {reach_str}\n"
            f"- **Antigravity Assessment**: {reasoning}\n"
            f"{dismiss_cmd}"
            f"<!-- alert-end: {alert_id} -->"
        )
        preserved_blocks.append(block)

    if not preserved_blocks:
        return header + "\n\n_All previously tracked alerts have been resolved or dismissed._\n"

    return header + "\n\n---\n\n" + "\n\n---\n\n".join(preserved_blocks) + "\n"


def cmd_plan(args: argparse.Namespace) -> int:
    with open(args.dependabot_json, encoding="utf-8") as f:
        dep_alerts = json.load(f)
    with open(args.codeql_json, encoding="utf-8") as f:
        codeql_alerts = json.load(f)
    with open(args.prs_json, encoding="utf-8") as f:
        open_prs = json.load(f)

    issue_body = ""
    if args.issue_body_file:
        try:
            with open(args.issue_body_file, encoding="utf-8") as f:
                issue_body = f.read()
        except FileNotFoundError:
            issue_body = ""

    promoted: list[dict[str, Any]] = []
    if args.promoted_json:
        try:
            with open(args.promoted_json, encoding="utf-8") as f:
                promoted = json.load(f)
        except FileNotFoundError:
            promoted = []

    plan = categorize_alerts(
        dependabot_alerts=dep_alerts,
        codeql_alerts=codeql_alerts,
        open_prs=open_prs,
        rolling_issue_body=issue_body,
        max_individual_prs=args.max_individual_prs,
        promoted_packages=promoted,
    )

    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(plan, f, indent=2)
    return 0


def cmd_update_cargo_pin(args: argparse.Namespace) -> int:
    with open(args.cargo_toml, encoding="utf-8") as f:
        original = f.read()
    updated, changed = update_cargo_toml_version(original, args.package, args.version)
    if changed:
        with open(args.cargo_toml, "w", encoding="utf-8") as f:
            f.write(updated)
    return 0


def cmd_add_npm_override(args: argparse.Namespace) -> int:
    with open(args.package_json, encoding="utf-8") as f:
        original = f.read()
    updated = add_or_update_npm_override(original, args.package, args.version)
    with open(args.package_json, "w", encoding="utf-8") as f:
        f.write(updated)
    return 0


def cmd_render_issue(args: argparse.Namespace) -> int:
    existing_body = ""
    if args.existing_body_file:
        try:
            with open(args.existing_body_file, encoding="utf-8") as f:
                existing_body = f.read()
        except FileNotFoundError:
            existing_body = ""

    with open(args.assessments_json, encoding="utf-8") as f:
        assessments = json.load(f)

    active_ids: set[str] | None = None
    if args.plan_json:
        try:
            with open(args.plan_json, encoding="utf-8") as f:
                plan_data = json.load(f)
                active_ids = set(plan_data.get("active_alert_ids", []))
        except FileNotFoundError:
            active_ids = None

    rendered = render_rolling_issue_body(
        existing_body=existing_body,
        new_assessments=assessments,
        active_alert_ids=active_ids,
        repo=args.repo,
    )
    with open(args.output, "w", encoding="utf-8") as f:
        f.write(rendered)
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Antigravity Security Vulnerability Remediation Helper"
    )
    sub = parser.add_subparsers(dest="subcommand", required=True)

    p_plan = sub.add_parser("plan", help="Categorize and deduplicate open security alerts")
    p_plan.add_argument("--dependabot-json", required=True)
    p_plan.add_argument("--codeql-json", required=True)
    p_plan.add_argument("--prs-json", required=True)
    p_plan.add_argument("--issue-body-file", default="")
    p_plan.add_argument("--promoted-json", default="")
    p_plan.add_argument("--max-individual-prs", type=int, default=3)
    p_plan.add_argument("--output", required=True)
    p_plan.set_defaults(func=cmd_plan)

    p_cargo = sub.add_parser(
        "update-cargo-pin", help="Update direct dependency version pin in Cargo.toml"
    )
    p_cargo.add_argument("--cargo-toml", required=True)
    p_cargo.add_argument("--package", required=True)
    p_cargo.add_argument("--version", required=True)
    p_cargo.set_defaults(func=cmd_update_cargo_pin)

    p_npm = sub.add_parser(
        "add-npm-override", help="Add or update an npm override in package.json"
    )
    p_npm.add_argument("--package-json", required=True)
    p_npm.add_argument("--package", required=True)
    p_npm.add_argument("--version", required=True)
    p_npm.set_defaults(func=cmd_add_npm_override)

    p_issue = sub.add_parser(
        "render-issue", help="Render markdown body for rolling security tracking issue"
    )
    p_issue.add_argument("--existing-body-file", default="")
    p_issue.add_argument("--assessments-json", required=True)
    p_issue.add_argument("--plan-json", default="")
    p_issue.add_argument("--repo", default="")
    p_issue.add_argument("--output", required=True)
    p_issue.set_defaults(func=cmd_render_issue)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
