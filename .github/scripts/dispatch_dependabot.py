#!/usr/bin/env python3
"""Sequential Dependabot Dispatcher.

Selects the oldest open Dependabot PR with auto-merge enabled. If it is behind
main, it rebases it via 'gh pr update-branch --rebase' so that PRs are updated
and tested one at a time without parallel CI rebase storms.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from typing import Any

DEPENDABOT_AUTHORS = {"dependabot[bot]", "app/dependabot"}


def select_next_action(pull_requests: list[dict[str, Any]]) -> tuple[dict[str, Any] | None, str]:
    """Select the oldest eligible Dependabot PR and determine the required action.

    Eligibility:
    1. Authored by dependabot[bot] or app/dependabot.
    2. Has auto-merge enabled (autoMergeRequest is not None).

    Actions:
    - 'rebase': PR is behind main and needs a rebase.
    - 'already_rebasing': PR is behind main, but the latest comment is already a rebase command.
    - 'in_progress': PR is up-to-date with main and actively running checks or ready to merge.
    - 'no_candidates': No open Dependabot PR has auto-merge enabled.
    """
    candidates: list[dict[str, Any]] = []

    for pr in pull_requests:
        author = pr.get("author", {}).get("login", "")
        if author not in DEPENDABOT_AUTHORS:
            continue

        auto_merge = pr.get("autoMergeRequest")
        if not auto_merge:
            continue

        candidates.append(pr)

    if not candidates:
        return None, "no_candidates"

    # Sort by createdAt ascending (FIFO: oldest PR first)
    candidates.sort(key=lambda p: p.get("createdAt", ""))
    oldest = candidates[0]

    merge_state = oldest.get("mergeStateStatus", "")

    if merge_state == "BEHIND":
        # Check if the last comment was already a rebase command
        comments = oldest.get("comments", [])
        if comments:
            last_comment_body = comments[-1].get("body", "").strip()
            if last_comment_body == "@dependabot rebase":
                return oldest, "already_rebasing"
        return oldest, "rebase"

    # Any other state (BLOCKED, CLEAN, UNSTABLE, etc.) means it is not behind main
    return oldest, "in_progress"


def run_gh_json(cmd: list[str]) -> Any:
    """Execute a GitHub CLI command and parse JSON output."""
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(result.stdout)


def dispatch_rebase(pr_number: int, dry_run: bool = False) -> None:
    """Rebase the target pull request branch on top of main using GitHub API."""
    print(f"Rebasing PR #{pr_number} branch via 'gh pr update-branch --rebase'...")
    if dry_run:
        print("[Dry-run] Branch not actually updated.")
        return

    subprocess.run(
        ["gh", "pr", "update-branch", str(pr_number), "--rebase"],
        check=True,
    )
    print(f"Successfully rebased PR #{pr_number} branch.")


def main() -> int:
    parser = argparse.ArgumentParser(description="Sequential Dependabot Dispatcher")
    parser.add_argument(
        "--json-file",
        help="Optional path to a JSON file with pull request data (for testing)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Run evaluation without posting any comments",
    )
    args = parser.parse_args()

    if args.json_file:
        with open(args.json_file, encoding="utf-8") as f:
            prs = json.load(f)
    else:
        cmd = [
            "gh",
            "pr",
            "list",
            "--state",
            "open",
            "--json",
            "number,title,createdAt,author,autoMergeRequest,mergeStateStatus,comments,headRefName",
        ]
        prs = run_gh_json(cmd)

    candidate, action = select_next_action(prs)

    if action == "no_candidates":
        print("No open Dependabot pull requests with auto-merge enabled were found.")
        return 0

    assert candidate is not None
    pr_num = candidate.get("number")
    title = candidate.get("title", "")
    state = candidate.get("mergeStateStatus", "")

    if action == "rebase":
        print(f"Target PR: #{pr_num} ('{title}') is BEHIND main.")
        dispatch_rebase(pr_num, dry_run=args.dry_run)
    elif action == "already_rebasing":
        print(
            f"Target PR: #{pr_num} ('{title}') is BEHIND main, but '@dependabot rebase' "
            "was already requested in the latest comment. Awaiting Dependabot."
        )
    elif action == "in_progress":
        print(
            f"Target PR: #{pr_num} ('{title}') is up-to-date with main (status: {state}). "
            "Checks or auto-merge in progress; leaving untouched."
        )

    return 0


if __name__ == "__main__":
    sys.exit(main())
