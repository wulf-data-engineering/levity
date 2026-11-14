---
trigger: always_on
---

This is a monorepo.

- `.agent`: Your rules, skills and workflows.
- `frontend`: Static Svelte 5 page (TypeScript)
- `backend`: Lambda functions on AWS (Rust with cargo lambda)
- `infrastructure`: AWS CDK (TypeScript)
- `protocols`: API definitions (Protocol Buffers)
- `.github`: GitHub Actions for CI/CD
- `docker-compose.yml`: for local development
- `local`: volumes for local development (includes cognito-local config)

## Development

**CRITICAL**: You **MUST** write unit or integration tests for any newly added feature or component. Do not finish a task without providing corresponding test coverage.
If there are existing unit, integration, or end-to-end tests, extend or update them.
**CRITICAL**: You **MUST** write the test for a new pure function at the same time.

Add descriptive comments to every function, struct, class or construct that has more than 3 lines of code.
**CRITICAL:** If you change a function, struct, or construct that has a comment, you **MUST** update the comment as well. Check the comment for correctness and completeness and update it if necessary.

**CRITICAL**: If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning or editing in parallel. Only modify files after the user approves the plan, and you switch to execution mode.

During feature development check if deployment workflow needs modifications.

**CRITICAL**: Note that formatting is handled automatically by the `pre-commit` hook. Linting and type-checking are automatically enforced by the `pre-push` hook. You do not need to run formatters or linters manually before committing.

## Code Search & File Traversal Rules (CRITICAL)

NEVER search, index, or read inside `target/`, `node_modules/`, `dist/`, or `.git/` directories.

1. **Built-in Search Tools (`grep_search`, `find_by_name`):**
   - Always pass exclusion globs if the tool supports it (e.g., exclude `**/target/**`, `**/node_modules/**`).
   - Prefer scoping your search path directly to source directories (e.g., `backend/src/`, `infrastructure/lib/`, `frontend/e2e/`) instead of the workspace or sub project root `.`, `backend/`, `frontend/`.

2. **Terminal / CLI Searches:**
   - **PREFER `rg` (ripgrep)** over `grep` or `find`, as `rg` automatically respects `.gitignore` and skips binary files.
   - If you must use `grep`, ALWAYS append: `--exclude-dir={target,node_modules,.git,dist} -I` (the `-I` flag ignores binary files completely).
   - If you must use `find`, ALWAYS prune build folders: `find . -name "target" -prune -o -name "node_modules" -prune -o ...`

## Internationalization (I18N)

This application is built to be polyglot. 
**CRITICAL**: If you add or modify any end-user facing texts, you MUST add them to all existing language files.
- Frontend texts go into `frontend/messages/<lang>.json` using ParaglideJS format.
- Backend texts (like emails/notifications) go into `backend/locales/<lang>.yml` using rust-i18n format.

## MCP Usage

Prefer MCP usage over command line tools if MCP supports the intended action.
E.g. looking into a Github PR is easily done with Github MCP. It doesn't require usage of `gh` which might need manual approval by the developer.
Other examples for Github are search issues, create comments, or manage pull requests.
Use `gh` just for actions not supported by the MCP like repository configuration.
That applies to all MCP servers.

## .env

Check if there is a .env file using `cat .env` (it is in .gitignore and derived from the committed .env.example).
If that's the case, use the ports defined in that file.

## AWS CLI & Profile Rules

**CRITICAL**: In this project, all AWS operations (both AWS CLI `aws ...` and AWS CDK `cdk ...`) **MUST** use an explicit `--profile` flag. Never execute AWS commands without a profile.

The standard profile names are:
- **Staging**: `@@ cookiecutter.project_slug @@-staging`
- **Production**: `@@ cookiecutter.project_slug @@-production`
- **Sandbox**: Ask the user for their sandbox profile name (defaults to `@@ cookiecutter.project_slug @@-sandbox` or developer's own name).

Example command:
```bash
aws sts get-caller-identity --profile @@ cookiecutter.project_slug @@-staging
```