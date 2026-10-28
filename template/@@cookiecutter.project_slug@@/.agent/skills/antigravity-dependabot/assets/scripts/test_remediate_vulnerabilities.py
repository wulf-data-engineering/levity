#!/usr/bin/env python3
"""Unit tests for remediate_vulnerabilities.py."""

import json
import unittest
from remediate_vulnerabilities import (
    VERSION_UPGRADE_BRANCH,
    add_or_update_npm_override,
    categorize_alerts,
    extract_documented_alert_ids,
    render_rolling_issue_body,
    update_cargo_toml_version,
)


class TestRemediateVulnerabilities(unittest.TestCase):
    def setUp(self):
        self.sample_dependabot_alerts = [
            {
                "number": 101,
                "state": "open",
                "dependency": {
                    "package": {"ecosystem": "cargo", "name": "quinn-proto"},
                    "manifest_path": "backend/Cargo.lock",
                },
                "security_advisory": {
                    "ghsa_id": "GHSA-1111-2222-3333",
                    "cve_id": "CVE-2026-1111",
                    "summary": "DoS in quinn-proto",
                    "severity": "high",
                },
                "security_vulnerability": {
                    "severity": "high",
                    "first_patched_version": {"identifier": "0.11.14"},
                },
            },
            {
                "number": 102,
                "state": "open",
                "dependency": {
                    "package": {"ecosystem": "cargo", "name": "quinn-proto"},
                    "manifest_path": "backend/Cargo.lock",
                },
                "security_advisory": {
                    "ghsa_id": "GHSA-4444-5555-6666",
                    "cve_id": "CVE-2026-2222",
                    "summary": "Critical RCE in quinn-proto",
                    "severity": "critical",
                },
                "security_vulnerability": {
                    "severity": "critical",
                    "first_patched_version": {"identifier": "0.11.19"},
                },
            },
            {
                "number": 153,
                "state": "open",
                "dependency": {
                    "package": {"ecosystem": "npm", "name": "deepmerge"},
                    "manifest_path": "frontend/package-lock.json",
                },
                "security_advisory": {
                    "ghsa_id": "GHSA-gcqx-w48m-w2h4",
                    "cve_id": "CVE-2026-93753",
                    "summary": "Prototype pollution in deepmerge",
                    "severity": "medium",
                },
                "security_vulnerability": {
                    "severity": "medium",
                    "first_patched_version": None,
                },
            },
        ]

        self.sample_codeql_alerts = [
            {
                "number": 1,
                "state": "open",
                "rule": {
                    "id": "js/shell-command-constructed-from-input",
                    "security_severity_level": "high",
                    "description": "Uncontrolled data used in OS command",
                },
                "most_recent_instance": {
                    "location": {
                        "path": "frontend/scripts/generate-protobuf-files.mjs",
                        "start_line": 22,
                        "end_line": 22,
                    },
                    "message": {"text": "Shell command constructed from input"},
                },
            },
            {
                "number": 2,
                "state": "open",
                "rule": {
                    "id": "actions/missing-workflow-permissions",
                    "security_severity_level": "medium",
                    "description": "Workflow does not contain explicit permissions",
                },
                "most_recent_instance": {
                    "location": {
                        "path": ".github/workflows/pull-request.yml",
                        "start_line": 1,
                        "end_line": 10,
                    },
                    "message": {"text": "Missing permissions block"},
                },
            },
            {
                "number": 3,
                "state": "open",
                "rule": {
                    "id": "actions/missing-workflow-permissions",
                    "security_severity_level": "medium",
                    "description": "Workflow does not contain explicit permissions",
                },
                "most_recent_instance": {
                    "location": {
                        "path": ".github/workflows/rust-cache.yml",
                        "start_line": 1,
                        "end_line": 10,
                    },
                    "message": {"text": "Missing permissions block"},
                },
            },
        ]

    def test_categorize_alerts_groups_and_merges_versions(self):
        plan = categorize_alerts(
            dependabot_alerts=self.sample_dependabot_alerts,
            codeql_alerts=self.sample_codeql_alerts,
            open_prs=[],
            rolling_issue_body="",
            max_individual_prs=3,
        )

        # Two quinn-proto alerts in same manifest merge into 1 upgrade entry with highest version (0.11.19) and severity (critical)
        self.assertEqual(len(plan["version_upgrades"]), 1)
        upg = plan["version_upgrades"][0]
        self.assertEqual(upg["package"], "quinn-proto")
        self.assertEqual(upg["patched_version"], "0.11.19")
        self.assertEqual(upg["severity"], "critical")
        self.assertEqual(upg["alert_numbers"], [101, 102])
        self.assertFalse(plan["skip_version_upgrades"])

        # CodeQL alerts 2 and 3 share the same rule.id so they group into 1 PR, sorted after high-severity alert 1
        self.assertEqual(len(plan["individual_groups"]), 2)
        self.assertEqual(
            plan["individual_groups"][0]["rule_id"],
            "js/shell-command-constructed-from-input",
        )
        self.assertEqual(plan["individual_groups"][0]["alert_numbers"], [1])
        self.assertEqual(
            plan["individual_groups"][1]["rule_id"],
            "actions/missing-workflow-permissions",
        )
        self.assertEqual(plan["individual_groups"][1]["alert_numbers"], [2, 3])
        self.assertEqual(len(plan["individual_groups"][1]["locations"]), 2)

        # Unpatched deepmerge alert goes to Phase 3
        self.assertEqual(len(plan["unpatched_alerts"]), 1)
        self.assertEqual(plan["unpatched_alerts"][0]["alert_id"], "dependabot-153")

    def test_deduplicates_existing_version_upgrade_pr(self):
        open_prs = [
            {
                "number": 500,
                "headRefName": VERSION_UPGRADE_BRANCH,
                "body": "Fixes:\n<!-- alert-id: dependabot-101 -->\n<!-- alert-id: dependabot-102 -->",
            }
        ]
        plan = categorize_alerts(
            dependabot_alerts=self.sample_dependabot_alerts,
            codeql_alerts=[],
            open_prs=open_prs,
        )
        self.assertTrue(plan["skip_version_upgrades"])
        self.assertEqual(plan["existing_version_upgrade_pr"], 500)

    def test_deduplicates_open_individual_prs_and_enforces_cap(self):
        extra_codeql = self.sample_codeql_alerts + [
            {
                "number": i,
                "state": "open",
                "rule": {
                    "id": f"rust/rule-{i}",
                    "security_severity_level": "low",
                    "description": f"Rule {i}",
                },
                "most_recent_instance": {
                    "location": {"path": "backend/src/lib.rs", "start_line": i, "end_line": i},
                    "message": {"text": "msg"},
                },
            }
            for i in range(10, 15)
        ]
        # Mark js/shell-command-constructed-from-input as already having an open PR
        open_prs = [
            {
                "number": 501,
                "headRefName": "security/fix-codeql-js-shell-command-constructed-from-input",
                "body": "",
            }
        ]
        plan = categorize_alerts(
            dependabot_alerts=[],
            codeql_alerts=extra_codeql,
            open_prs=open_prs,
            max_individual_prs=3,
        )
        self.assertEqual(len(plan["individual_groups"]), 3)
        branches = [g["branch"] for g in plan["individual_groups"]]
        self.assertNotIn(
            "security/fix-codeql-js-shell-command-constructed-from-input", branches
        )
        self.assertEqual(
            branches[0], "security/fix-codeql-actions-missing-workflow-permissions"
        )

    def test_update_cargo_toml_version_simple_and_table(self):
        cargo_toml = (
            '[dependencies]\n'
            'anyhow = "1.0.102"\n'
            'tokio = { version = "1.51.1", features = ["macros"] }\n'
            'wiremock = "0.6.5"\n'
        )
        updated, changed = update_cargo_toml_version(cargo_toml, "anyhow", "1.0.104")
        self.assertTrue(changed)
        self.assertIn('anyhow = "1.0.104"', updated)

        updated2, changed2 = update_cargo_toml_version(updated, "tokio", "1.54.1")
        self.assertTrue(changed2)
        self.assertIn('tokio = { version = "1.54.1", features = ["macros"] }', updated2)

        # Downgrade or equal version is a no-op
        updated3, changed3 = update_cargo_toml_version(updated2, "wiremock", "0.6.5")
        self.assertFalse(changed3)
        self.assertEqual(updated2, updated3)

    def test_add_or_update_npm_override_preserves_indent(self):
        pkg_json = '{\n\t"name": "frontend",\n\t"dependencies": {\n\t\t"svelte": "^5.0.0"\n\t}\n}\n'
        updated = add_or_update_npm_override(
            pkg_json, "postcss-selector-parser", "6.1.5"
        )
        self.assertTrue(updated.startswith("{\n\t"))
        parsed = json.loads(updated)
        self.assertEqual(
            parsed["overrides"]["postcss-selector-parser"], "^6.1.5"
        )

    def test_render_rolling_issue_body_preserves_active_and_prunes_closed(self):
        first_body = render_rolling_issue_body(
            existing_body="",
            new_assessments=[
                {
                    "alert_id": "dependabot-153",
                    "alert_number": 153,
                    "package": "deepmerge",
                    "ecosystem": "npm",
                    "manifest_path": "frontend/package-lock.json",
                    "severity": "medium",
                    "ghsa_id": "GHSA-gcqx-w48m-w2h4",
                    "cve_id": "CVE-2026-93753",
                    "summary": "Prototype pollution in deepmerge",
                    "reachable": False,
                    "dev_only": True,
                    "recommendation": "tolerable_risk",
                    "reasoning": "Only used at build time by @sveltejs/kit config merging.",
                    "repo": "wulf-data-engineering/levity",
                }
            ],
            active_alert_ids={"dependabot-153"},
        )
        self.assertIn("<!-- alert-id: dependabot-153 -->", first_body)
        self.assertIn(
            "gh api -X PATCH repos/wulf-data-engineering/levity/dependabot/alerts/153",
            first_body,
        )
        self.assertEqual(extract_documented_alert_ids(first_body), {"dependabot-153"})

        # Add a second alert while keeping 153 active
        second_body = render_rolling_issue_body(
            existing_body=first_body,
            new_assessments=[
                {
                    "alert_id": "dependabot-200",
                    "alert_number": 200,
                    "package": "some-pkg",
                    "ecosystem": "cargo",
                    "manifest_path": "backend/Cargo.lock",
                    "severity": "low",
                    "summary": "Minor issue",
                    "reachable": False,
                    "dev_only": False,
                    "recommendation": "not_used",
                    "reasoning": "Dead code path.",
                    "repo": "wulf-data-engineering/levity",
                }
            ],
            active_alert_ids={"dependabot-153", "dependabot-200"},
        )
        self.assertEqual(
            extract_documented_alert_ids(second_body),
            {"dependabot-153", "dependabot-200"},
        )

        # Once 153 is dismissed/closed on GitHub, active_alert_ids only has 200 -> 153 is pruned
        third_body = render_rolling_issue_body(
            existing_body=second_body,
            new_assessments=[],
            active_alert_ids={"dependabot-200"},
        )
        self.assertEqual(
            extract_documented_alert_ids(third_body), {"dependabot-200"}
        )

    def test_handles_disabled_api_error_dicts_gracefully(self):
        # When Code Scanning or Dependabot alerts are not enabled on a repo, gh api may return an error dict
        plan = categorize_alerts(
            dependabot_alerts={"message": "Dependabot alerts are disabled"},  # type: ignore[arg-type]
            codeql_alerts={"message": "Advanced Security must be enabled"},  # type: ignore[arg-type]
            open_prs=[],
        )
        self.assertTrue(plan["skip_version_upgrades"])
        self.assertEqual(plan["version_upgrades"], [])
        self.assertEqual(plan["individual_groups"], [])
        self.assertEqual(plan["unpatched_alerts"], [])


if __name__ == "__main__":
    unittest.main()
