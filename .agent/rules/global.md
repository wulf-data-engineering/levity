---
trigger: always_on
---

This repository contains a **cookiecutter template** for generating applications in `template/%[cookiecutter.project_slug]%/`.

The template's rules apply to you if you change the template:

Ref: @../../template/%[cookiecutter.project_slug]%/.agent/rules/global.md
Ref: @../../template/%[cookiecutter.project_slug]%/.agent/skills/backend/SKILL.md
Ref: @../../template/%[cookiecutter.project_slug]%/.agent/skills/frontend/SKILL.md
Ref: @../../template/%[cookiecutter.project_slug]%/.agent/skills/protocols/SKILL.md
Ref: @../../template/%[cookiecutter.project_slug]%/.agent/skills/database/SKILL.md

## Template Structure

The source code for the generated project lives in `template/`.

- @../../template/cookiecutter.json: Defines the variables and default values.
- `template/%[cookiecutter.project_slug]%/`: The root of the cookiecutter template.

## Wording

- `.` is the **template repo/repository**.
- `template/%[cookiecutter.project_slug]%/` is the **template**.
- `../levity-instances/<project_slug>/` is a **template instance** when generated.

> **Note**: The `.. /levity-instances/` directory is outside of the workspace. In case of read or write failures you **MUST** ask the user to allow it via
> _Settings > Agent > File Access > Agent Non-Workspace File Access_.

## Important Paths

- `template/`: Source of the template
- `template/cookiecutter.json`: Configuration
- `template/%[cookiecutter.project_slug]%/.agent`: AI rules and workflows for generated projects
- `.github/workflows/`: CI/CD for the template itself (linting, testing generation)
- `..levity-instances/`: Temporary instances of generated projects

## Template Application Context

The `template/` directory contains the source for a **Levity Application**.
Even when working on the template repo itself, you must understand the architecture and rules of the application it generates.
