---
trigger: always_on
---

This repository contains a **cookiecutter template** for generating Levity applications in `template/@@cookiecutter.project_slug@@/` (variables defined in `template/cookiecutter.json`).

## Repository Layout & Context

- `.`: **Template repository** (`.github/workflows/` runs template CI).
- `template/@@cookiecutter.project_slug@@/`: **Template source** — when modifying it, the template's rules and skills in `template/@@cookiecutter.project_slug@@/.agent/` also apply.
- `../levity-instances/<project_slug>/`: **Temporary template instance** for building and testing.
  > If reading or writing in `../levity-instances/` is blocked by workspace access controls, ask the user to allow it via **Settings > Non-Workspace File Access** (or project **File Access Policy**).

## Building & Testing

Never run `npm run build|check|test|lint`, `cargo update|check|test`, etc. directly inside `template/` subdirectories.
Follow the `template_development` skill and use `/instantiate-template` (`@../skills/instantiate-template/SKILL.md`) to test inside `../levity-instances/<project_slug>/`.

## Planning, ADRs & Commits

- **Planning**: If the user asks for a _plan_ (or `/plan`), do **not** modify any files until the user approves the plan.
- **ADRs**: Before implementing a feature, check `adr/` to see if an existing ADR becomes obsolete or a new ADR is needed, and suggest changes to the user. *(Ignore in headless CI runs.)*
- **Commits**: Do not run `git commit` unless explicitly requested by the user.

## MCP Usage

Prefer configured MCP servers (e.g., `github`) over networked CLI tools like `gh` when MCP supports the action, as MCP tools run without requiring manual sandbox elevation.
