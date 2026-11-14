#!/usr/bin/env python3
"""PreToolUse hook that blocks file edits in the repository while on 'main'."""

import json
import os
import subprocess
import sys


def allow() -> None:
    print(json.dumps({"decision": "allow"}))


def deny(reason: str) -> None:
    print(json.dumps({"decision": "deny", "reason": reason}))


def main() -> None:
    if os.environ.get("CI", "").lower() == "true":
        allow()
        return

    try:
        payload = json.load(sys.stdin)
    except Exception:
        allow()
        return

    workspace_paths = payload.get("workspacePaths") or []
    if not workspace_paths:
        allow()
        return

    repo_root = os.path.realpath(workspace_paths[0])
    args = (payload.get("toolCall") or {}).get("args") or {}
    target_path = args.get("Path") or args.get("TargetFile") or args.get("FilePath") or ""

    if target_path:
        resolved_target = os.path.realpath(
            target_path if os.path.isabs(target_path) else os.path.join(repo_root, target_path)
        )
        # Allow writes to artifacts, scratch, /tmp, or sibling test instances outside this repo.
        try:
            in_repo = os.path.commonpath([repo_root, resolved_target]) == repo_root
        except ValueError:
            in_repo = False
        if not in_repo:
            allow()
            return

    try:
        result = subprocess.run(
            ["git", "branch", "--show-current"],
            cwd=repo_root,
            capture_output=True,
            text=True,
            timeout=3,
        )
        branch = result.stdout.strip()
    except Exception:
        allow()
        return

    if branch == "main":
        deny(
            "Editing files directly on 'main' is forbidden. "
            "Stop and ask the user to switch to (or create) a feature branch first."
        )
        return

    allow()


if __name__ == "__main__":
    main()
