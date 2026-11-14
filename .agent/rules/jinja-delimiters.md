---
trigger: glob
globs: "template/**"
description: Custom Jinja2 delimiters required when reading or editing files inside template/.
---

# Jinja2 Delimiter Rules

To avoid conflicts with frameworks like Svelte (`{ ... }`) or Cargo, this project uses **custom Jinja2 delimiters** for template generation.

## CRITICAL: Use Custom Delimiters

When modifying files within `template/`, you **MUST** use the following custom delimiters instead of standard Jinja2 syntax:

| Type         | Standard    | **Project Custom** |
| :----------- | :---------- | :----------------- |
| **Variable** | `{{ ... }}` | **`@@ ... @@`**    |
| **Block**    | `{% ... %}` | **`@= ... =@`**    |
| **Comment**  | `{# ... #}` | **`@- ... -@`**    |

### Examples

- **Incorrect**: `{{ cookiecutter.project_name }}`, `{% if cookiecutter.use_docker %}`, `{# comment #}`
- **Correct**: `@@ cookiecutter.project_name @@`, `@= if cookiecutter.use_docker =@`, `@- comment -@`

See `template/cookiecutter.json` for all available variables.
