---
trigger: always_on
---

# Jinja2 Delimiter Rules

To avoid conflicts with frameworks like Svelte (`{ ... }`) or Cargo, this project uses **custom Jinja2 delimiters** for the template generation.

## CRITICAL: Use Custom Delimiters

When modifying files within the `template/` directory, you **MUST** use the following custom delimiters instead of standard Jinja2 syntax.

| Type         | Standard    | **Project Custom** |
| :----------- | :---------- | :----------------- |
| **Variable** | `{{ ... }}` | **`%[ ... ]%`**    |
| **Block**    | `{% ... %}` | **`%%[ ... ]%%`**  |
| **Comment**  | `{# ... #}` | **`%[# ... #]%`**  |

### Examples

#### ❌ Incorrect (Will Conflict)

```jinja
{{ cookiecutter.project_name }}
{% if cookiecutter.use_docker %}
{# This is a comment #}
```

#### ✅ Correct

```jinja
%[ cookiecutter.project_name ]%
%%[ if cookiecutter.use_docker ]%%
%[# This is a comment #]%
```

### Reference

See `template/cookiecutter.json` for the full configuration.
