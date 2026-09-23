#!/usr/bin/env python3
"""Unit tests for dispatch_dependabot.py."""

import unittest
from unittest.mock import patch
from dispatch_dependabot import dispatch_rebase, select_next_action


class TestDispatchDependabot(unittest.TestCase):
    def test_empty_pull_requests(self):
        candidate, action = select_next_action([])
        self.assertIsNone(candidate)
        self.assertEqual(action, "no_candidates")

    def test_no_dependabot_prs(self):
        prs = [
            {
                "number": 1,
                "author": {"login": "octocat"},
                "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
                "createdAt": "2026-09-19T09:00:00Z",
                "mergeStateStatus": "BEHIND",
            }
        ]
        candidate, action = select_next_action(prs)
        self.assertIsNone(candidate)
        self.assertEqual(action, "no_candidates")

    def test_dependabot_pr_without_automerge(self):
        prs = [
            {
                "number": 2,
                "author": {"login": "dependabot[bot]"},
                "autoMergeRequest": None,
                "createdAt": "2026-09-19T09:00:00Z",
                "mergeStateStatus": "BEHIND",
            }
        ]
        candidate, action = select_next_action(prs)
        self.assertIsNone(candidate)
        self.assertEqual(action, "no_candidates")

    def test_dependabot_pr_behind_needs_rebase(self):
        pr = {
            "number": 10,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
            "createdAt": "2026-09-19T09:00:00Z",
            "mergeStateStatus": "BEHIND",
            "comments": [],
        }
        candidate, action = select_next_action([pr])
        self.assertEqual(candidate, pr)
        self.assertEqual(action, "rebase")

    def test_app_dependabot_author_supported(self):
        pr = {
            "number": 11,
            "author": {"login": "app/dependabot"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
            "createdAt": "2026-09-19T09:00:00Z",
            "mergeStateStatus": "BEHIND",
            "comments": [],
        }
        candidate, action = select_next_action([pr])
        self.assertEqual(candidate, pr)
        self.assertEqual(action, "rebase")

    def test_already_requested_rebase_not_duplicated(self):
        pr = {
            "number": 12,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
            "createdAt": "2026-09-19T09:00:00Z",
            "mergeStateStatus": "BEHIND",
            "comments": [{"body": "CI passed"}, {"body": " @dependabot rebase \n"}],
        }
        candidate, action = select_next_action([pr])
        self.assertEqual(candidate, pr)
        self.assertEqual(action, "already_rebasing")

    def test_up_to_date_blocked_in_progress(self):
        pr = {
            "number": 13,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
            "createdAt": "2026-09-19T09:00:00Z",
            "mergeStateStatus": "BLOCKED",
            "comments": [],
        }
        candidate, action = select_next_action([pr])
        self.assertEqual(candidate, pr)
        self.assertEqual(action, "in_progress")

    def test_up_to_date_clean_in_progress(self):
        pr = {
            "number": 14,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T10:00:00Z"},
            "createdAt": "2026-09-19T09:00:00Z",
            "mergeStateStatus": "CLEAN",
            "comments": [],
        }
        candidate, action = select_next_action([pr])
        self.assertEqual(candidate, pr)
        self.assertEqual(action, "in_progress")

    def test_selects_oldest_candidate_first(self):
        older_pr = {
            "number": 20,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T08:00:00Z"},
            "createdAt": "2026-09-19T07:00:00Z",
            "mergeStateStatus": "BEHIND",
            "comments": [],
        }
        newer_pr = {
            "number": 21,
            "author": {"login": "dependabot[bot]"},
            "autoMergeRequest": {"enabledAt": "2026-09-19T09:00:00Z"},
            "createdAt": "2026-09-19T08:30:00Z",
            "mergeStateStatus": "BEHIND",
            "comments": [],
        }
        candidate, action = select_next_action([newer_pr, older_pr])
        self.assertEqual(candidate, older_pr)
        self.assertEqual(action, "rebase")

    @patch("subprocess.run")
    def test_dispatch_rebase_calls_gh_update_branch(self, mock_run):
        """Verifies dispatch_rebase executes 'gh pr update-branch <pr> --rebase'."""
        dispatch_rebase(42, dry_run=False)
        mock_run.assert_called_once_with(
            ["gh", "pr", "update-branch", "42", "--rebase"],
            check=True,
        )

    @patch("subprocess.run")
    def test_dispatch_rebase_dry_run_does_not_call_subprocess(self, mock_run):
        """Verifies dispatch_rebase in dry-run mode does not invoke subprocess."""
        dispatch_rebase(42, dry_run=True)
        mock_run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
