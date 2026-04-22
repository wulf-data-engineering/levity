---
name: Changing Dependabot Config
description: Instructions for changing and testing the dependabot configs.
---

Usually changes to dependabot in the template also should affect the top-level dependabot config that keeps the template up to date.

A change in `template/@@cookiecutter.project_slug@@/.github/dependabot.yml` should be accompanied by a change in `.github/dependabot.yml`.

## Example

### Change in template

```yaml
# template/@@cookiecutter.project_slug@@/.github/dependabot.yml
...
updates:
  ...
  # Backend
  - package-ecosystem: "cargo"
    directories:
      - "backend/"
    schedule:
      interval: "daily"
      ...
    groups:
      ...
```

### Corresponding change in top-level configuration

```yaml
# .github/dependabot.yml
...
updates:
  ...
  # Template's Backend
  - package-ecosystem: "cargo"
    directories:
      - "template/*/backend/"
    schedule:
      interval: "daily"
      ...
    groups:
      ...
```

## Special handling

Always target the template as `template/*/"` from top-level.
Dependabot cannot handle the actual template name.

Some top-level dependabot configuration like Github Actions covers the template AND the this repository for the template itself:

```yaml
# .github/dependabot.yml
...
  # GitHub Actions (Own & Template)
  - package-ecosystem: "github-actions"
    directories:
      - "/"
      - "template/*/"
```

## Testing Dependabot Configs

If you changed the top-level config, check it using

```sh
npx @bugron/validate-dependabot-yaml@latest
```

If you changed the template's config, check it using

```sh
npx @bugron/validate-dependabot-yaml@latest "template/@@cookiecutter.project_slug@@/.github/dependabot.yml"
```