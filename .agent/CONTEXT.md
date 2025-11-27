# Repository Context

## Purpose

This repository contains a **cookiecutter template** for generating applications in `template/%[cookiecutter.project_slug]%/`.

Consult its `README.md` and its own `.agent/CONTEXT.md` for details on the generated project structure and development workflow.

## Key Concepts

### 1. Template Structure

The source code for the generated project lives in `template/`.

- `template/cookiecutter.json`: Defines the variables and default values.
- `template/%[cookiecutter.project_slug]%/`: The root of the generated project.

### 2. Jinja2 Delimiters

**CRITICAL**: This project uses **custom Jinja2 delimiters** to avoid conflicts with Svelte, Cargo and other frameworks.
**Variable Start**: `%[`  
**Variable End**: `]%`

See `template/cookiecutter.json` for all.

**Example**:
Instead of `{{ cookiecutter.project_name }}`, you must use `%[ cookiecutter.project_name ]%`.

### 3. Planning Workflow

Before implementation check if the feature would make an existing adr obsolete or if a new adr is needed.
Suggest changes or a new adr and ask user for feedback.

### 4. Development Workflow

Since this is a template, you cannot run the code directly from `template/`.

1. Consult `workflow/instantiate_template.md` how to generate a temporary instance of the project using `cookiecutter`.
2. Consult `workflow/develop_feature.md` how to make changes and test them in the temporary instance.
3. Consult `workflow/backport_changes.md` how to manually apply the changes back to the `template/` directory, ensuring placeholders are preserved.
4. Make changes to the GitHub workflows of the template itself if needed.

## Important Paths

- `template/`: Source of the template.
- `template/cookiecutter.json`: Configuration.
- `.github/workflows/`: CI/CD for the template itself (linting, testing generation).
