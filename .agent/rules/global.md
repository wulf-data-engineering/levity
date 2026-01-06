---
trigger: always_on
---

This repository contains a **cookiecutter template** for generating applications in `template/%[cookiecutter.project_slug]%/`.

Read the template's [CONTEXT.md](template/%[cookiecutter.project_slug]%/CONTEXT.md) for detailed repository architecture and rules.

## Template Structure

The source code for the generated project lives in `template/`.

- `template/cookiecutter.json`: Defines the variables and default values.
- `template/%[cookiecutter.project_slug]%/`: The root of the cookiecutter template.

## Wording

- `.` is the **template repo/repository**.
- `template/%[cookiecutter.project_slug]%/` is the **template**.
- `../levity-instances/<project_slug>/` is a **template instance** when generated.

> **Note**: The `.. /levity-instances/` directory is outside of the workspace. In case of read or write failures you **MUST** ask the user to allow it via 
*Settings > Agent > File Access > Agent Non-Workspace File Access*.

## Important Paths

- `template/`: Source of the template
- `template/cookiecutter.json`: Configuration
- `template/%[cookiecutter.project_slug]%/.agent`: AI rules and workflows for generated projects
- `.github/workflows/`: CI/CD for the template itself (linting, testing generation)
- `..levity-instances/`: Temporary instances of generated projects